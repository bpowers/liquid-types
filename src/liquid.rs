#[cfg(test)]
use std;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use explicit;
use lambdal;
use lambdal::{Expr, Op, Imm};
use common::{Id, Result, LiquidError};
use refined::{Base, T};

use rustproof_libsmt::backends::smtlib2::SMTLib2;
use rustproof_libsmt::backends::backend::*;
use rustproof_libsmt::backends::z3::Z3;
use rustproof_libsmt::theories::{core, integer};
use rustproof_libsmt::logics::lia::LIA;
use rustproof_libsmt::logics::lia::LIA_Sorts;

macro_rules! otry {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => return None,
    })
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum C {
    WellFormed(Box<Type>),
    Subtype(Box<Type>, Box<Type>),
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Liquid {
    E(Expr),
    K(Id, Box<LinkedList<Expr>>), // list of pending substitutions
}

pub type Type = T<Liquid>;
// if we have a type T1 with two subtype constraints: T2 <: T1 and T3
// <: T1, that would be a single STConstraint with 2 antedents.
pub type STConstraints = (Vec<(LinkedList<Expr>, Box<Type>)>, Id);

pub type Idx = i32; // constraint index

pub type Constraint = ((HashSet<Id>, LinkedList<Expr>), C); // Boolean valued expressions & their environments

#[derive(Debug, Clone)]
pub struct KInfo {
    all_qs: HashSet<lambdal::Expr>,
    curr_qs: Vec<lambdal::Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Env {
    shape: HashMap<Id, explicit::Type>,
    refined_env: HashMap<Id, Type>,
    path_constraints: LinkedList<Expr>,
}

impl Env {
    fn new(shape: &HashMap<Id, explicit::Type>) -> Env {
        Env {
            shape: shape.clone(),
            refined_env: HashMap::new(),
            path_constraints: LinkedList::new(),
        }
    }

    fn get(&self, s: &Id) -> Type {
        match self.refined_env.get(s) {
            Some(ty) => ty.clone(),
            None => panic!("env.get('{}' missing ({:?})", s, self.in_scope()),
        }
    }

    fn insert(&mut self, s: &Id, ty: &Type) {
        self.refined_env.insert(s.clone(), ty.clone());
    }

    fn add_constraint(&mut self, e: &Expr) {
        self.path_constraints.push_back(e.clone())
    }

    fn in_scope(&self) -> HashSet<Id> {
        let keys: HashSet<_> = self.refined_env.keys().cloned().collect();
        return keys
    }

    fn snapshot(&self) -> (HashSet<Id>, LinkedList<Expr>) {
        (self.in_scope(), self.path_constraints.clone())
    }
}

fn hm_shape_imm(env: &HashMap<Id, explicit::Type>, imm: &Imm) -> explicit::Type {
    use explicit::Type::*;
    use lambdal::Imm::*;

    match *imm {
        Bool(_) => TBool,
        Int(_) => TInt,
        Var(ref id) => env[id].clone(),
        Fun(ref id, ref e) => {
            let argty = env[id].clone();
            TFun(id.clone(), box argty, box hm_shape_expr(env, e))
        }
        Fix(ref id, _) => env[id].clone(),
        V => env["!v"].clone(),
        Star => unreachable!("shape of star"),
    }
}

fn hm_shape_op(env: &HashMap<Id, explicit::Type>, op: &Op) -> explicit::Type {
    use explicit::Type::*;
    use lambdal::Op::*;

    match *op {
        Op2(op, _, _) => explicit::opty(op),
        MkArray(_, _) => TIntArray,
        GetArray(_, _) => TInt,
        SetArray(_, _, _) => TIntArray,
        Imm(ref imm) => hm_shape_imm(env, imm),
    }
}

fn hm_shape_expr(env: &HashMap<Id, explicit::Type>, expr: &Expr) -> explicit::Type {
    use explicit::Type::TFun;
    use lambdal::Expr::*;

    match *expr {
        If(_, ref e2, _) => hm_shape_expr(env, e2),
        Let(_, _, ref e2) => hm_shape_expr(env, e2),
        App(ref e1, _) => {
            if let TFun(_, _, rtype) = hm_shape_imm(env, e1) {
                *rtype
            } else {
                panic!("app for non-fun: {:?}", e1);
            }
        }
        Op(ref op) => hm_shape_op(env, op),
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KEnv {
    shape: HashMap<Id, explicit::Type>,
    env_id: String,
    next_id: i32,
}

impl KEnv {
    fn new(shape: &HashMap<Id, explicit::Type>) -> KEnv {
        KEnv {
            shape: shape.clone(),
            env_id: String::from("κ"), // ν
            next_id: 0,
        }
    }

    fn fresh_ty(&mut self, env: &Env, ty: explicit::Type) -> Type {
        let id = self.next_id;
        self.next_id += 1;

        let base = match ty {
            explicit::Type::TInt => Base::Int,
            explicit::Type::TBool => Base::Bool,
            explicit::Type::TFun(id, a, b) => {
                let fx = self.fresh_ty(env, *a);
                let f = self.fresh_ty(env, *b);
                return T::Fun(id, box fx, box f);
            }
            _ => panic!("FIXME: handle {:?}", ty),
        };
        let k = Liquid::K(format!("!k{}", id), box LinkedList::new());
        T::Ref(env.in_scope(), base, box k)
    }

    fn fresh(&mut self, env: &Env, e: &Expr) -> Type {
        let hm_type = hm_shape_expr(&env.shape, e);
        self.fresh_ty(env, hm_type)
    }
}

fn ty<'a>(_: &mut KEnv, _: &Env, c: &Imm) -> Type {
    use common::Op2;
    use self::Liquid::E;

    let base = match *c {
        Imm::Int(_) => Base::Int,
        Imm::Bool(_) => Base::Bool,
        _ => unreachable!("only called for constants."),
    };

    println!("ty({:?})", base);

    // {ν : int | ν = 3 }
    let eq = E(Expr::Op(Op::Op2(Op2::Eq, box Imm::V, box c.clone())));
    T::Ref(HashSet::new(), base, box eq)
}

fn base(ty: &Type) -> Option<Base> {
    match *ty {
        T::Ref(_, b, _) => Some(b),
        _ => None,
    }
}

fn subst(_: &Id, _: &Imm, ty: &Type) -> Type {
    println!("TODO: subst");
    ty.clone()
}

pub fn cons_imm<'a>(k_env: &mut KEnv, env: &Env, imm: &Imm) -> (Type, LinkedList<Constraint>) {
    use lambdal::Imm::*;
    use common::Op2::Eq;

    match *imm {
        Var(ref id) => {
            let ty: Type = if let Some(b) = base(&env.get(id)) {
                let eq = Expr::Op(Op::Op2(Eq, box Imm::V, box Imm::Var(id.clone())));
                T::Ref(env.in_scope(), b, box Liquid::E(eq))
            } else {
                println!("{} not base -- using just env ({:?})", id, env.get(id));
                env.get(id)
            };
            (ty, LinkedList::new())
        }
        Bool(_) => {
            (ty(k_env, &env, imm), LinkedList::new())
        }
        Int(_) => {
            (ty(k_env, &env, imm), LinkedList::new())
        }
        Fun(ref x, ref e) => {
            let mut env = env.clone();
            let fx = k_env.fresh(&env, &Expr::Op(Op::Imm(Var(x.clone()))));
            env.insert(x, &fx);
            let f = k_env.fresh(&env, e);
            let (fe, mut c) = cons_expr(k_env, &env, e);
            // Γ ⊢ (x:fx → f)
            c.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,x:fx ⊢ (fe <: f)
            c.push_back((env.snapshot(), C::Subtype(box fe.clone(), box f.clone())));

            (T::Fun(x.clone(), box fx.clone(), box f), c)
        }
        Fix(ref x, ref e) => {
            // const w/ ∀α.(α→α)→α
            let mut env = env.clone();
            let fx = k_env.fresh(&env, e);
            env.insert(x, &fx);

            println!("FIXME: cons_imm(Fix)");
            cons_expr(k_env, &env, e)
        }
        _ => {
            panic!("TODO: cons_imm for {:?}", imm);
        }
    }
}

pub fn cons_op<'a>(k_env: &mut KEnv, env: &Env, e: &Op) -> (Type, LinkedList<Constraint>) {
    match *e {
        Op::Imm(ref imm) => cons_imm(k_env, env, imm),
        Op::Op2(_, _, _) => {
            panic!("ugh");
            // let (_, mut c1) = cons_imm(k_env, env, e1);
            // let (_, mut c2) = cons_imm(k_env, env, e2);
            // c1.append(&mut c2);

            // let ty = explicit::opty(op);
            // let base = match ty {
            //     explicit::Type::TInt => Base::Int,
            //     explicit::Type::TBool => Base::Bool,
            //     _ => panic!("FIXME: handle {:?}", ty),
            // };

            // let eq = Expr::Op(Op::Op2(Eq, box Imm::V, box Expr::Op(e.clone())));
            // let f = T::Ref(env.in_scope(), base, box Liquid::E(eq));

            // (f, c1)
        }
        _ => {
            panic!("TODO: cons_op for {:?}", e);
        }
    }
}

pub fn cons_expr<'a>(k_env: &mut KEnv, env: &Env, expr: &Expr) -> (Type, LinkedList<Constraint>) {
    use lambdal::Op as LOp;
    use lambdal::Expr::*;

    match *expr {
        If(ref e1, ref e2, ref e3) => {
            let mut env_t = env.clone();
            let mut env_f = env.clone();
            env_t.add_constraint(&Expr::Op(LOp::Imm(*e1.clone())));
            env_f.add_constraint(&Expr::App(box Imm::Var(String::from("not")), e1.clone()));

            let f = k_env.fresh(&env, expr);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons_imm(k_env, &env, e1);
            let (f2, mut c2) = cons_expr(k_env, &env_t, e2);
            let (f3, mut c3) = cons_expr(k_env, &env_f, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // Γ ⊢ (f)
            c1.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,e1 ⊢ (f2 <: f)
            c1.push_back((env_t.snapshot(), C::Subtype(box f2.clone(), box f.clone())));
            // Γ,¬e1 ⊢ (f3 <: f)
            c1.push_back((env_f.snapshot(), C::Subtype(box f3.clone(), box f.clone())));

            (f, c1)
        }
        Let(ref id, ref e1, ref e2) => {
            let mut env = env.clone();

            let f = k_env.fresh(&env, expr);
            let (f1, mut c1) = cons_expr(k_env, &env, e1);
            env.insert(id, &f1);
            let (f2, mut c2) = cons_expr(k_env, &env, e2);
            c1.append(&mut c2);
            // Γ ⊢ (f)
            c1.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,x:f1 ⊢ (f2 <: f)
            c1.push_back((env.snapshot(), C::Subtype(box f2.clone(), box f.clone())));

            (f, c1)
        }
        App(ref e1, ref e2) => {
            let (f1, mut c1) = cons_imm(k_env, env, e1);
            println!("## {:?}\t:\t{:?}", e1, f1);
            let (f2, mut c2) = cons_imm(k_env, env, e2);
            c1.append(&mut c2);
            if let T::Fun(ref x, ref fx, ref f) = f1 {
                let f = subst(x, e2, f);
                // Γ ⊢ (f2 <: fx)
                c1.push_back((env.snapshot(), C::Subtype(box f2.clone(), fx.clone())));
                return (f, c1);
            } else {
                panic!("expected TFun, not {:?}", f1);
            }
            // let (x:Fx → F, C1) = Cons(Γ, e1) in
            // let (F
            // 0
            // x, C2) = Cons(Γ, e2) in
            // ([e2/x]F, C1 ∪ C2 ∪ {Γ ` F
            // 0
            // x <: Fx})
        }
        Op(ref op) => cons_op(k_env, &env, op),
    }
}

fn split(map: &mut HashMap<Idx, Constraint>, constraints: &LinkedList<Constraint>) {
    let mut idx = (map.len() as i32)+1;

    for c in constraints.iter() {
        if let &((ref scope, ref pathc), C::Subtype(box T::Fun(_, ref tx1, ref t1),
                                                    box T::Fun(ref x2, ref tx2, ref t2))) = c {

            let mut contra_cs: LinkedList<Constraint> = LinkedList::new();
            contra_cs.push_back(((scope.clone(), pathc.clone()), C::Subtype(tx2.clone(), tx1.clone())));

            let mut rscope = scope.clone();
            rscope.insert(x2.clone());
            contra_cs.push_back(((rscope, pathc.clone()), C::Subtype(t1.clone(), t2.clone())));

            // recurse
            split(map, &contra_cs);
            idx = (map.len() as i32)+1;
        } else if let &((ref scope, ref pathc), C::WellFormed(box T::Fun(ref id, _, ref t))) = c {
            let mut wf_cs: LinkedList<Constraint> = LinkedList::new();

            let mut scope = scope.clone();
            scope.insert(id.clone());
            wf_cs.push_back(((scope, pathc.clone()), C::WellFormed(t.clone())));

            // recurse
            split(map, &wf_cs);
            idx = (map.len() as i32)+1;
        } else {
            map.insert(idx, c.clone());
            idx += 1;
        }
    }
}

fn replace_imm(v: &Id, q: &Imm) -> Option<Imm> {
    use lambdal::Imm::*;

    let r = match *q {
        Bool(_) | Int(_) | Var(_) | V => q.clone(),
        Fun(ref id, ref e)            => Fun(id.clone(), box otry!(replace_expr(v, e))),
        Fix(ref id, ref e)            => Fix(id.clone(), box otry!(replace_expr(v, e))),
        Star                          => Var(v.clone()),
    };

    Some(r)
}
fn replace_op(v: &Id, q: &lambdal::Op) -> Option<lambdal::Op> {
    use lambdal::Op::*;

    let r = match *q {
        Op2(ref op, ref l, ref r)          => Op2(*op, box otry!(replace_imm(v, l)), box otry!(replace_imm(v, r))),
        MkArray(ref sz, ref n)             => MkArray(box otry!(replace_imm(v, sz)), box otry!(replace_imm(v, n))),
        GetArray(ref id, ref idx)          => GetArray(box otry!(replace_imm(v, id)), box otry!(replace_imm(v, idx))),
        SetArray(ref id, ref idx, ref var) => SetArray(box otry!(replace_imm(v, id)), box otry!(replace_imm(v, idx)), box otry!(replace_imm(v, var))),
        Imm(ref imm)                       => Imm(otry!(replace_imm(v, imm))),
    };

    Some(r)
}
fn replace_expr(v: &Id, q: &lambdal::Expr) -> Option<lambdal::Expr> {
    use lambdal::Expr::*;

    let r = match *q {
        If(ref e1, ref e2, ref e3)  => If(box otry!(replace_imm(v, e1)), box otry!(replace_expr(v, e2)), box otry!(replace_expr(v, e3))),
        App(ref e1, ref e2)         => App(box otry!(replace_imm(v, e1)), box otry!(replace_imm(v, e2))),
        Let(ref id, ref e1, ref e2) => Let(id.clone(), box otry!(replace_expr(v, e1)), box otry!(replace_expr(v, e2))),
        Op(ref op)                  => Op(otry!(replace_op(v, op))),
    };

    Some(r)
}

// instantiate Q for k w/ alpha-renamed variables that are in-scope
// and of the right shape at the location of the well-formedness
// constraint
fn qstar(_: &Id, in_scope: &HashSet<Id>, _: &HashMap<Id, explicit::Type>, qset: &[lambdal::Expr]) -> HashSet<lambdal::Expr> {
    let mut qstar: HashSet<lambdal::Expr> = HashSet::new();

    for tmpl in qset {
        for v in in_scope.iter() {
            match replace_expr(v, tmpl) {
                Some(e) => {
                    qstar.insert(e);
                },
                None => {
                    println!("not used:\t{:?}", tmpl);
                },
            };
        }
    }

    qstar
}

fn build_a(constraints: &HashMap<Idx, Constraint>, env: &HashMap<Id, explicit::Type>, q: &[Expr]) -> HashMap<Id, KInfo> {
    let mut a: HashMap<Id, KInfo> = HashMap::new();

    for (_, c) in constraints.iter() {
        if let &((_, _), C::WellFormed(ref ty)) = c {
            if let &box T::Ref(ref in_scope, _, box Liquid::K(ref id, _)) = ty {
                // TODO: subst?
                let all_qs = qstar(id, in_scope, env, q);
                let curr_qs: Vec<_> = all_qs.iter().cloned().collect();
                a.insert(id.clone(), KInfo{
                    all_qs: all_qs,
                    curr_qs: curr_qs,
                });
            } else {
                panic!("WellFormed with E doesn't make sense: {:?}.", ty)
            }
        }
        // TODO: track antecedents that reference each k
    }

    a
}

fn smt_from_imm(
    s: &mut SMTLib2<LIA>,
    vars: &HashMap<String, <SMTLib2<LIA> as SMTBackend>::Idx>,
    q: &lambdal::Imm) -> <SMTLib2<LIA> as SMTBackend>::Idx {

    use lambdal::Imm as I;

    match *q {
        I::Var(ref id)        => vars[id],
        I::Bool(b)            => s.new_const(core::OpCodes::Const(b)),
        I::Int(n)             => s.new_const(integer::OpCodes::Const(n as u64)),
        I::V                  => vars["!v"],
        I::Star               => unreachable!(),
        I::Fun(_, _) => unreachable!("fun in smt?"),
        I::Fix(_, _) => unreachable!("fix in smt?"),
    }
}

fn smt_from_op(
    s: &mut SMTLib2<LIA>,
    vars: &HashMap<String, <SMTLib2<LIA> as SMTBackend>::Idx>,
    q: &lambdal::Op) -> <SMTLib2<LIA> as SMTBackend>::Idx {

    use lambdal::Op as O;
    use common::Op2;


    match *q {
        O::Imm(ref imm)          => smt_from_imm(s, vars, imm),
        O::Op2(op, ref l, ref r) => {
            let il = smt_from_imm(s, vars, l);
            let ir = smt_from_imm(s, vars, r);
            match op {
                Op2::And | Op2::Or | Op2::Impl | Op2::Iff => {
                    let opcode = match op {
                        Op2::And  => core::OpCodes::And,
                        Op2::Or   => core::OpCodes::Or,
                        Op2::Impl => core::OpCodes::Imply,
                        Op2::Iff  => {panic!("iff not implemented");}
                        _         => unreachable!(),
                    };
                    s.assert(opcode, &[il, ir])
                }
                Op2::Add | Op2::Sub | Op2::Mul |
                Op2::LT | Op2::LTE | Op2::GT | Op2::GTE | Op2::Eq => {
                    let opcode = match op {
                        Op2::LT  => integer::OpCodes::Lt,
                        Op2::LTE => integer::OpCodes::Lte,
                        Op2::GT  => integer::OpCodes::Gt,
                        Op2::GTE => integer::OpCodes::Gte,
                        Op2::Eq  => integer::OpCodes::Cmp,
                        Op2::Add => integer::OpCodes::Add,
                        Op2::Sub => integer::OpCodes::Sub,
                        Op2::Mul => integer::OpCodes::Mul,
                        _        => unreachable!(),
                    };
                    s.assert(opcode, &[il, ir])
                }
            }
        }
        _ => {
            panic!("smt_from_op unimplemented {:?}", q);
        }
    }
}

fn smt_from_expr(
    s: &mut SMTLib2<LIA>,
    vars: &HashMap<String, <SMTLib2<LIA> as SMTBackend>::Idx>,
    q: &lambdal::Expr) -> <SMTLib2<LIA> as SMTBackend>::Idx {

    use lambdal::Expr as E;

    match *q {
        E::Let(ref id, ref e1, ref e2) => {
            let id_idx = match vars.get(id) {
                Some(idx) => *idx,
                None => panic!("key {} not found in {:?}", id, vars),
            };
            let eq_exprs = &[id_idx, smt_from_expr(s, vars, e1)];
            let _ = s.assert(integer::OpCodes::Cmp, eq_exprs);
            smt_from_expr(s, vars, e2)
        }
        E::App(ref e1, ref e2)         => {
            if *e1 == box Imm::Var(String::from("not")) {
                let exprs = &[smt_from_imm(s, vars, e2)];
                s.assert(core::OpCodes::Not, exprs)
            } else {
                panic!("TODO: only supported app is not");
            }
        }
        E::Op(ref op) => smt_from_op(s, vars, op),
        _ => {
            panic!("smt_from_expr unimplemented {:?}", q);
        }
    }
}

// whether the conjunction of all p implies the conjunction of all q
fn implication_holds(env: &HashMap<Id, explicit::Type>, p: &[lambdal::Expr], q: &[lambdal::Expr]) -> bool {
    let mut z3 = Z3::new_with_binary("./z3");
    let mut solver = SMTLib2::new(Some(LIA));
    solver.set_logic(&mut z3);

    let mut senv: HashMap<Id, _> = HashMap::new();

    // Defining the symbolic vars x & y
    for (var, ty) in env {
        let sty: LIA_Sorts = match *ty {
            explicit::Type::TInt => integer::Sorts::Int.into(),
            explicit::Type::TBool => core::Sorts::Bool.into(),
            _ => panic!("TODO: more sorts than int"),
        };
        let idx = solver.new_var(Some(&var), sty);
        senv.insert(var.clone(), idx);
    }
    // TODO: is v always an int?
    senv.insert(String::from("!v"), solver.new_var(Some("!v"), integer::Sorts::Int));

    //let strue = solver.new_const(core::OpCodes::Const(true));

    let mut ps: Vec<_> = Vec::new();
    for t in p {
        let pred = smt_from_expr(&mut solver, &senv, t);
        //let _ = solver.assert(integer::OpCodes::Cmp, &[pred, strue]);
        ps.push(pred);
    }

    let mut qs: Vec<_> = Vec::new();
    for t in q {
        let pred = smt_from_expr(&mut solver, &senv, t);
        //let _ = solver.assert(integer::OpCodes::Cmp, &[pred, strue]);
        qs.push(pred);
    }

    let p_all = if ps.len() > 1 {
        solver.assert(core::OpCodes::And, &ps)
    } else {
        ps[0]
    };
    let q_all = if qs.len() > 1 {
        solver.assert(core::OpCodes::And, &qs)
    } else {
        qs[0]
    };
    let imply = solver.assert(core::OpCodes::Imply, &[p_all, q_all]);
    let _ = solver.assert(core::OpCodes::Not, &[imply]);

    let (_, sat) = solver.solve(&mut z3, false);

    match sat {
        SMTRes::Unsat(_, _) => true,
        _ => false,
    }
}

// whether the conjunction of all p implies the conjunction of all q
fn weaken(
    env: &HashMap<Id, explicit::Type>,
    a: &HashMap<Id, KInfo>,
    all_p: &Vec<(LinkedList<Expr>, Box<Type>)>,
    qs: &HashSet<lambdal::Expr>) -> Option<Vec<lambdal::Expr>> {

    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));

    let mut curr_qs: Vec<lambdal::Expr> = Vec::new();
    for q in qs {
        curr_qs.push(q.clone());

        for &(ref path_constraints, ref p) in all_p {
            let mut p = match *p {
                box T::Ref(_, _, box Liquid::E(ref expr)) => vec![expr.clone()],
                box T::Ref(_, _, box Liquid::K(ref p_id, _)) => match a.get(p_id) {
                    Some(ps) => ps.clone().curr_qs,
                    None => vec![const_true.clone()],
                },
                box T::Fun(_, _, _) => {
                    panic!("unexpected {:?} -- should all be split() by now", p)
                }
            };
            for pc in path_constraints {
                p.push(pc.clone());
            }

            println!("check: {:?}", curr_qs);
            if !implication_holds(env, &p, &curr_qs) {
                let _ = curr_qs.pop();
                break;
            }
        };
    };

    Some(curr_qs)
}

fn solve(
    env: &HashMap<Id, explicit::Type>,
    constraints: &LinkedList<STConstraints>,
    a: &HashMap<Id, KInfo>) -> Result<HashMap<Id, KInfo>> {

    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));

    for &(ref all_p, ref id) in constraints.iter() {

        //println!("C\n\t\t{:?}\n\t\t\t{:?}", all_p, id);
        // if we don't find the ID in our environment, it means we are
        // looking at unbound function parameters -- which means we can just look
        let ref qs = match a.get(id) {
            Some(q) => q.clone(),
            None => {
                let mut all_qs = HashSet::new();
                all_qs.insert(const_true.clone());
                KInfo{
                    all_qs: all_qs,
                    curr_qs: vec![const_true.clone()],
                }
            }
        };

        for &(ref path_constraints, ref p) in all_p {
            let mut p = match *p {
                box T::Ref(_, _, box Liquid::E(ref expr)) => vec![expr.clone()],
                box T::Ref(_, _, box Liquid::K(ref p_id, _)) => match a.get(p_id) {
                    Some(ps) => ps.clone().curr_qs,
                    None => vec![const_true.clone()],
                },
                box T::Fun(_, _, _) => {
                    panic!("unexpected {:?} -- should all be split() by now", p)
                }
            };
            for pc in path_constraints {
                p.push(pc.clone());
            }
            if !implication_holds(env, &p, &qs.curr_qs) {
                if qs.curr_qs[0] == const_true {
                    return err!("implication failure for -> true");
                }
                match weaken(env, a, all_p, &qs.all_qs) {
                    Some(new_qs) => {
                        let mut new_a = a.clone();
                        new_a.insert(id.clone(), KInfo{
                            all_qs: qs.all_qs.clone(),
                            curr_qs: new_qs,
                        });
                        println!("RECURSOIN");
                        return solve(env, constraints, &new_a);
                    }
                    None => {
                        return err!("Weaken failed for {:?}", p);
                    }
                }
            } else {
                println!("{} is ok", id);
            }
        };
    };

    Ok(a.clone())
}

pub fn infer(expr: &Expr, env: &HashMap<Id, explicit::Type>, q: &[lambdal::Expr]) -> Result<HashMap<Id, Vec<lambdal::Expr>>> {
    let mut k_env = KEnv::new(env);
    let (_, constraint_list) = cons_expr(&mut k_env, &Env::new(env), expr);

    let mut constraints: HashMap<Idx, Constraint> = HashMap::new();
    split(&mut constraints, &constraint_list);

    let a = build_a(&constraints, env, q);

    // group subtyping constraints by supertype
    let mut by_id: HashMap<Id, Vec<(LinkedList<Expr>, Box<Type>)>> = HashMap::new();
    for (_, c) in constraints.iter() {
        if let &((_, ref path), C::Subtype(ref p, ref e)) = c {
            if let box T::Ref(_, _, box Liquid::K(ref id, _)) = *e {
                let mut antecedent = vec![(path.clone(), p.clone())];
                if by_id.contains_key(id) {
                    let mut others = by_id[id].clone();
                    others.append(&mut antecedent);
                    by_id.insert(id.clone(), others);
                } else {
                    by_id.insert(id.clone(), antecedent);
                }
            }
        }
    }
    let mut all_constraints: LinkedList<STConstraints> = LinkedList::new();
    for (id, v) in by_id {
        all_constraints.push_back((v.clone(), id.clone()));
    }

    let min_a = solve(env, &all_constraints, &a)?;

    let mut res = HashMap::new();
    for (k, v) in min_a {
        res.insert(k, v.curr_qs.clone());
    }

    Ok(res)
}

macro_rules! expr(
    ($s:expr) => { {
        use lambdal;
        use implicit_parse;
        use tok::Tokenizer;
        use std;
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        let iexpr = match implicit_parse::parse_Program(&s, tokenizer) {
            Ok(iexpr) => iexpr,
            Err(e) => {
                die!("parse_Program({}): {:?}", $s, e);
            }
        };
        match lambdal::q(&iexpr) {
            Ok(expr) => expr,
            Err(e) => {
                die!("anf: {:?}", e);
            }
        }
    } }
);

#[test]
fn test_implication() {

    let mut env: HashMap<Id, explicit::Type> = HashMap::new();
    env.insert(String::from("x"), explicit::Type::TInt);
    env.insert(String::from("y"), explicit::Type::TInt);
    env.insert(String::from("!tmp-0!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-0!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-0!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-1!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-1!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-1!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-2!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-2!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-2!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!3"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!4"), explicit::Type::TBool);

    let p = [
        expr!("x <= y ∧ ν = y"),
    ];

    let q = [
        expr!("ν >= x ∧ ν >= y"),
    ];

    // expect this to hold
    if !implication_holds(&env, &p, &q) {
        die!("1 expected {:?} => {:?}", p, q);
    }

    let p = [
        expr!("x <= y ∧ ν = y"),
    ];

    let q = [
        expr!("ν < 0 ∧ ν >= x ∧ ν >= y"),
    ];

    // but this shouldn't
    if implication_holds(&env, &p, &q) {
        die!("2 expected {:?} => {:?}", p, q);
    }
}

#[test]
fn z3_works() {
    let mut z3 = Z3::new_with_binary("./z3");
    let mut solver = SMTLib2::new(Some(LIA));
    solver.set_logic(&mut z3);

    // Defining the symbolic vars x & y
    let x = solver.new_var(Some("x"), integer::Sorts::Int);
    let y = solver.new_var(Some("y"), integer::Sorts::Int);
    let v = solver.new_var(Some("v"), integer::Sorts::Int);

    //let int0 = solver.new_const(integer::OpCodes::Const(0));

    let p1 = solver.assert(integer::OpCodes::Lte, &[x, y]);
    let p2 = solver.assert(integer::OpCodes::Cmp, &[v, y]);
    let p_all = solver.assert(core::OpCodes::And, &[p1, p2]);


    let k1 = solver.assert(integer::OpCodes::Gte, &[v, x]);
    let k2 = solver.assert(integer::OpCodes::Gte, &[v, y]);
    let k_all = solver.assert(core::OpCodes::And, &[k1, k2]);

    let imply = solver.assert(core::OpCodes::Imply, &[p_all, k_all]);
    let _ = solver.assert(core::OpCodes::Not, &[imply]);

    let (_, sat) = solver.solve(&mut z3, false);
    match sat {
        SMTRes::Unsat(_, _) => {}
        _ => {
            die!("expected unsat, not {:?}", sat);
        }
    }
}
