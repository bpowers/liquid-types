use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::result;
use std::fmt::{Debug, Formatter, Error};

use hindley_milner;
use explicit;
use implicit;
use lambdal;
use lambdal::{Expr, Op, Imm};
use common::{Id, Result};
use refined::{Base, T};

use rustproof_libsmt::backends::smtlib2::SMTLib2;
use rustproof_libsmt::backends::backend::*;
use rustproof_libsmt::backends::z3::Z3;
use rustproof_libsmt::theories::{core, integer};
use rustproof_libsmt::logics::lia::LIA;
use rustproof_libsmt::logics::lia::LIA_Sorts;

use explicit::Type::TInt;

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


#[derive(PartialEq, Eq, Clone)]
pub enum Liquid {
    E(Expr),
    K(Id, Box<LinkedList<(Imm, Id)>>), // list of pending substitutions [imm/id]
}

impl Debug for Liquid {
    fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), Error> {
        use self::Liquid::*;
        match *self {
            E(ref expr) => write!(fmt, "{:?}", expr),
            K(ref id, ref subs) => write!(fmt, "{} {:?}", id, subs),
        }
    }
}


pub type Type = T<Liquid>;
// if we have a type T1 with two subtype constraints: T2 <: T1 and T3
// <: T1, that would be a single STConstraint with 2 antedents.
pub type STConstraints = (Vec<(HashSet<Id>, LinkedList<Expr>, Box<Type>)>, Id);

pub type Idx = i32; // constraint index

pub type Constraint = ((HashSet<Id>, LinkedList<Expr>), C); // Boolean valued expressions & their environments

#[derive(Debug, Clone)]
pub struct KInfo {
    base: Base,
    all_qs: HashSet<lambdal::Expr>,
    curr_qs: Vec<lambdal::Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KEnv {
    shape: HashMap<Id, explicit::Type>,
    refined: HashMap<Id, Type>,
    next_id: i32,
}

impl KEnv {
    fn new(shape: &HashMap<Id, explicit::Type>) -> KEnv {
        KEnv {
            shape: shape.clone(),
            refined: HashMap::new(),
            next_id: 0,
        }
    }

    fn fresh_ty(&mut self, ty: explicit::Type) -> Type {
        let id = self.next_id;
        self.next_id += 1;

        let base = match ty {
            explicit::Type::TInt => Base::Int,
            explicit::Type::TBool => Base::Bool,
            explicit::Type::TFun(id, a, b) => {
                let fx = self.fresh_ty(*a);
                let f = self.fresh_ty(*b);
                return T::Fun(id, box fx, box f);
            }
            _ => panic!("FIXME: handle {:?}", ty),
        };
        let kid = format!("_k{}", id);
        let k = Liquid::K(kid, box LinkedList::new());
        T::Ref(self.in_scope(), base, box k)
    }

    fn fresh(&mut self, e: &Expr) -> Type {
        let hm_type = hm_shape_expr(&self.shape, e);
        self.fresh_ty(hm_type)
    }

    fn get(&self, s: &Id) -> Type {
        match self.refined.get(s) {
            Some(ty) => ty.clone(),
            None => panic!("env.get('{}' missing ({:?})", s, self.in_scope()),
        }
    }

    fn insert(&mut self, id: &Id, ty: &Type) {
        self.refined.insert(id.clone(), ty.clone());
    }

    fn in_scope(&self) -> HashSet<Id> {
        let keys: HashSet<_> = self.refined.keys().cloned().collect();
        return keys
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

fn ty<'a>(_: &mut KEnv, c: &Imm) -> Type {
    use common::Op2;
    use self::Liquid::E;

    let base = match *c {
        Imm::Int(_) => Base::Int,
        Imm::Bool(_) => Base::Bool,
        _ => unreachable!("only called for constants."),
    };

    // something like {ν : int | ν = 3 }
    let eq = E(Expr::Op(Op::Op2(Op2::Eq, box Op::Imm(Imm::V), box Op::Imm(c.clone()))));
    T::Ref(HashSet::new(), base, box eq)
}

fn base(ty: &Type) -> Option<Base> {
    match *ty {
        T::Ref(_, b, _) => Some(b),
        _ => None,
    }
}

fn subst(sid: &Id, imm: &Imm, ty: &Type) -> Type {
    if let &T::Ref(ref scope, base, box Liquid::K(ref id, ref pending)) = ty {
        let mut pending = pending.clone();
        pending.push_back((imm.clone(), sid.clone()));
        T::Ref(scope.clone(), base, box Liquid::K(id.clone(), pending))
    } else if let &T::Fun(ref id, ref tx, ref t) = ty {
        T::Fun(id.clone(), box subst(sid, imm, tx), box subst(sid, imm, t))
    } else {
        panic!("Expected liquid type, not fun or full dep type: {:?}", ty);
    }
}

pub fn cons_imm<'a>(k_env: &mut KEnv, pathc: &LinkedList<Expr>, imm: &Imm) -> (Type, LinkedList<Constraint>) {
    use lambdal::Op::Imm as I;
    use lambdal::Imm::*;
    use common::Op2::Eq;

    match *imm {
        Var(ref id) => {
            let ty: Type = if let Some(b) = base(&k_env.get(id)) {
                let eq = Expr::Op(Op::Op2(Eq, box I(V), box I(Var(id.clone()))));
                T::Ref(k_env.in_scope(), b, box Liquid::E(eq))
            } else {
                k_env.get(id)
            };
            (ty, LinkedList::new())
        }
        Bool(_) => {
            (ty(k_env, imm), LinkedList::new())
        }
        Int(_) => {
            (ty(k_env, imm), LinkedList::new())
        }
        Fun(ref x, ref e) => {
            let fx = k_env.fresh(&Expr::Op(Op::Imm(Var(x.clone()))));
            let ffun_env = (k_env.in_scope(), pathc.clone());

            k_env.insert(x, &fx);
            let f = k_env.fresh(e);
            let (fe, mut c) = cons_expr(k_env, pathc, e);

            let ffun = T::Fun(x.clone(), box fx.clone(), box f.clone());
            // Γ ⊢ (x:fx → f)
            c.push_back((ffun_env.clone(), C::WellFormed(box fx.clone())));
            c.push_back((ffun_env.clone(), C::WellFormed(box ffun.clone())));
            // Γ,x:fx ⊢ (fe <: f)
            c.push_back((ffun_env.clone(), C::Subtype(box fe, box f)));

            (ffun, c)
        }
        Fix(ref x, ref e) => {
            // const w/ ∀α.(α→α)→α
            let f = k_env.fresh(e);
            k_env.insert(x, &f);
            let ffix_env = (k_env.in_scope(), pathc.clone());

            let (fe, mut c) = cons_expr(k_env, &pathc, e);
            c.push_back((ffix_env, C::Subtype(box fe.clone(), box f.clone())));

            (f, c)
        }
        _ => {
            panic!("TODO: cons_imm for {:?}", imm);
        }
    }
}

pub fn cons_op<'a>(k_env: &mut KEnv, pathc: &LinkedList<Expr>, e: &Op) -> (Type, LinkedList<Constraint>) {
    use common::Op2::Eq;

    match *e {
        Op::Imm(ref imm) => cons_imm(k_env, pathc, imm),
        Op::Op2(ref op, ref l, ref r) => {
            let (_, mut c1) = cons_op(k_env, pathc, l);
            let (_, mut c2) = cons_op(k_env, pathc, r);
            c1.append(&mut c2);

            let ty = explicit::opty(*op);
            let base = match ty {
                explicit::Type::TInt => Base::Int,
                explicit::Type::TBool => Base::Bool,
                _ => panic!("FIXME: handle {:?}", ty),
            };

            let eq = Expr::Op(Op::Op2(Eq, box Op::Imm(Imm::V), box e.clone()));
            let f = T::Ref(k_env.in_scope(), base, box Liquid::E(eq));

            (f, c1)
        }
        _ => {
            panic!("TODO: cons_op for {:?}", e);
        }
    }
}

pub fn cons_expr<'a>(k_env: &mut KEnv, pathc: &LinkedList<Expr>, expr: &Expr) -> (Type, LinkedList<Constraint>) {
    use lambdal::Op as LOp;
    use lambdal::Expr::*;

    match *expr {
        If(ref e1, ref e2, ref e3) => {
            let mut pathc_t = pathc.clone();
            let mut pathc_f = pathc.clone();
            pathc_t.push_back(Expr::Op(LOp::Imm(*e1.clone())));
            pathc_f.push_back(Expr::App(box Imm::Var(String::from("not")), e1.clone()));

            let f = k_env.fresh(expr);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons_imm(k_env, &pathc, e1);
            let (f2, mut c2) = cons_expr(k_env, &pathc_t, e2);
            let (f3, mut c3) = cons_expr(k_env, &pathc_f, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // Γ ⊢ (f)
            c1.push_back(((k_env.in_scope(), pathc.clone()), C::WellFormed(box f.clone())));
            // Γ,e1 ⊢ (f2 <: f)
            c1.push_back(((k_env.in_scope(), pathc_t), C::Subtype(box f2.clone(), box f.clone())));
            // Γ,¬e1 ⊢ (f3 <: f)
            c1.push_back(((k_env.in_scope(), pathc_f), C::Subtype(box f3.clone(), box f.clone())));

            (f, c1)
        }
        Let(ref id, ref e1, ref e2) => {
            let f = k_env.fresh(expr);
            let mut in_scope = k_env.in_scope();
            let (f1, mut c1) = cons_expr(k_env, pathc, e1);
            k_env.insert(id, &f1);
            in_scope.insert(id.clone());

            let let_env = (in_scope, pathc.clone());

            let (f2, mut c2) = cons_expr(k_env, pathc, e2);
            c1.append(&mut c2);
            // Γ ⊢ (f)
            c1.push_back((let_env.clone(), C::WellFormed(box f.clone())));
            // Γ,x:f1 ⊢ (f2 <: f)
            c1.push_back((let_env, C::Subtype(box f2.clone(), box f.clone())));

            (f, c1)
        }
        App(ref e1, ref e2) => {
            let (f1, mut c1) = cons_imm(k_env, pathc, e1);
            let (f2, mut c2) = cons_imm(k_env, pathc, e2);
            c1.append(&mut c2);
            if let T::Fun(ref x, ref fx, ref f) = f1 {
                let f = subst(x, e2, f);
                // Γ ⊢ (f2 <: fx)
                c1.push_back(((HashSet::new(), pathc.clone()), C::WellFormed(fx.clone())));
                c1.push_back(((k_env.in_scope(), pathc.clone()), C::Subtype(box f2.clone(), fx.clone())));
                return (f, c1);
            } else {
                panic!("expected TFun, not {:?}", f1);
            }
        }
        Op(ref op) => cons_op(k_env, &pathc, op),
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


fn replace_imm(imm: &Imm, from: &Imm, to: &Imm) -> Imm {
    if imm == from {
        return to.clone();
    }

    imm.clone()
}

fn replace_op(op: &Op, from: &Imm, to: &Imm) -> Op {
    use lambdal::Op::*;
    match *op {
        Op2(op, ref l, ref r) => Op2(op, box replace_op(l, from, to), box replace_op(r, from, to)),
        MkArray(ref sz, ref n) => MkArray(box replace_imm(sz, from, to), box replace_imm(n, from, to)),
        GetArray(ref a, ref i) => GetArray(box replace_imm(a, from, to), box replace_imm(i, from, to)),
        SetArray(ref a, ref i, ref v) => SetArray(box replace_imm(a, from, to), box replace_imm(i, from, to), box replace_imm(v, from, to)),
        Imm(ref imm) => Imm(replace_imm(imm, from, to)),
    }
}

fn replace_expr(expr: &Expr, from: &Imm, to: &Imm) -> Expr {
    use lambdal::Expr::*;
    match *expr {
        If(ref imm, ref e1, ref e2) => If(box replace_imm(imm, from, to), box replace_expr(e1, from, to), box replace_expr(e2, from, to)),
        Let(ref id, ref e1, ref e2) => Let(id.clone(), box replace_expr(e1, from, to), box replace_expr(e2, from, to)),
        App(ref imm1, ref imm2) => App(box replace_imm(imm1, from, to), box replace_imm(imm2, from, to)),
        Op(ref op) => Op(replace_op(op, from, to)),
    }
}

fn replace(v: &Id, q: &implicit::Expr) -> Option<implicit::Expr> {
    use implicit::Expr as I;

    let r = match *q {
        I::Var(ref id)                        => I::Var(id.clone()),
        I::Const(ref c)                       => I::Const(*c),
        I::Op2(ref op, ref l, ref r)          => I::Op2(*op, box otry!(replace(v, l)), box otry!(replace(v, r))),
        I::Fun(ref id, ref e)                 => I::Fun(id.clone(), box otry!(replace(v, e))),
        I::App(ref e1, ref e2)                => I::App(box otry!(replace(v, e1)), box otry!(replace(v, e2))),
        I::If(ref e1, ref e2, ref e3)         => I::If(box otry!(replace(v, e1)), box otry!(replace(v, e2)), box otry!(replace(v, e3))),
        I::Let(ref id, ref e1, ref e2)        => I::Let(id.clone(), box otry!(replace(v, e1)), box otry!(replace(v, e2))),
        I::Fix(ref id, ref e)                 => I::Fix(id.clone(), box otry!(replace(v, e))),
        I::MkArray(ref sz, ref n)             => I::MkArray(box otry!(replace(v, sz)), box otry!(replace(v, n))),
        I::GetArray(ref id, ref idx)          => I::GetArray(box otry!(replace(v, id)), box otry!(replace(v, idx))),
        I::SetArray(ref id, ref idx, ref var) => I::SetArray(box otry!(replace(v, id)), box otry!(replace(v, idx)), box otry!(replace(v, var))),
        I::V                                  => I::V,
        I::Star                               => I::Var(v.clone()),
    };

    Some(r)
}

// instantiate Q for k w/ alpha-renamed variables that are in-scope
// and of the right shape at the location of the well-formedness
// constraint
fn qstar(
    _: &Id,
    in_scope: &HashSet<Id>,
    env: &HashMap<Id, explicit::Type>,
    qset: &[implicit::Expr]) -> HashSet<lambdal::Expr> {

    use explicit::Type::TBool;

    let mut qstar: HashSet<lambdal::Expr> = HashSet::new();
    for tmpl in qset {
        if in_scope.len() == 0 {
            let r = hindley_milner::infer_in(env.clone(), tmpl);
            if let Ok(_) = r {
                if let Ok(ee) = lambdal::q(tmpl) {
                    let ty = hm_shape_expr(env, &ee);
                    if let TBool = ty.clone() {
                        qstar.insert(ee);
                    } else {
                        println!("not TBool for {:?} ({:?})", ee, ty);
                    }
                }
            }
            continue;
        }
        for v in in_scope.iter() {
            if let Some(e) = replace(v, tmpl) {
                let r = hindley_milner::infer_in(env.clone(), &e);
                if let Ok(_) = r {
                    if let Ok(ee) = lambdal::q(&e) {
                        let ty = hm_shape_expr(env, &ee);
                        if let TBool = ty.clone() {
                            qstar.insert(ee);
                        } else {
                            println!("not TBool for {:?} ({:?})", ee, ty);
                        }
                    } else {
                        println!("lambdal conv failed for {:?}", e);
                    }
                }
            };
        }
    }

    qstar
}

fn build_a(constraints: &HashMap<Idx, Constraint>, env: &HashMap<Id, explicit::Type>, q: &[implicit::Expr]) -> HashMap<Id, KInfo> {
    let mut a: HashMap<Id, KInfo> = HashMap::new();

    for (_, c) in constraints.iter() {
        if let &((ref in_scope, _), C::WellFormed(ref ty)) = c {
            if let &box T::Ref(_, ref base, box Liquid::K(ref id, _)) = ty {
                let all_qs = qstar(id, in_scope, env, q);
                let curr_qs: Vec<_> = all_qs.iter().cloned().collect();
                a.insert(id.clone(), KInfo{
                    base: *base,
                    all_qs: all_qs,
                    curr_qs: curr_qs,
                });
            } else {
                panic!("WellFormed with E doesn't make sense: {:?}.", ty)
            }
        }
    }

    a
}

fn smt_from_imm(
    s: &mut SMTLib2<LIA>,
    vars: &HashMap<String, <SMTLib2<LIA> as SMTBackend>::Idx>,
    q: &lambdal::Imm) -> <SMTLib2<LIA> as SMTBackend>::Idx {

    use lambdal::Imm as I;

    match *q {
        I::Var(ref id) => {
            match vars.get(id) {
                Some(v) => *v,
                None => panic!("smt_from_imm: {} not in {:?}", id, vars),
            }
        }
        I::Bool(b)   => s.new_const(core::OpCodes::Const(b)),
        I::Int(n)    => s.new_const(integer::OpCodes::Const(n)),
        I::V         => vars["!v"],
        I::Star      => unreachable!("star in smt?"),
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
            let il = smt_from_op(s, vars, l);
            let ir = smt_from_op(s, vars, r);
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
        E::App(ref e1, ref e2) => {
            if **e1 == Imm::Var(String::from("not")) {
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

fn expr_from_var(a: &HashMap<Id, KInfo>, var: &Id, ty: &Type) -> Vec<lambdal::Expr> {
    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));
    let exprs = match *ty {
        T::Ref(_, _, box Liquid::E(ref expr)) => vec![expr.clone()],
        T::Ref(_, _, box Liquid::K(ref p_id, ref substs)) => {
            let qs = match a.get(p_id) {
                Some(ps) => ps.clone().curr_qs,
                None => vec![const_true.clone()],
            };
            let mut qs_replaced = vec![];
            for mut q in qs {
                for &(ref sub, ref var) in substs.iter() {
                    q = replace_expr(&q, &Imm::Var(var.clone()), sub);
                }
                qs_replaced.push(q);
            }
            qs_replaced
        },
        T::Fun(_, _, _) => {
            // functions are uninterpreted.
            vec![const_true.clone()]
        }
    };
    let mut instantiated = vec![];
    for e in exprs {
        instantiated.push(replace_expr(&e, &Imm::V, &Imm::Var(var.clone())));
    }
    instantiated
}

fn sort_from_ty(ty: &explicit::Type) -> Option<LIA_Sorts> {
    let sort: LIA_Sorts = match *ty {
        explicit::Type::TInt => integer::Sorts::Int.into(),
        explicit::Type::TBool => core::Sorts::Bool.into(),
        _ => {
            //println!("TODO: v'{}' more sorts than int ({:?})", var, ty);
            return None
        }
    };
    Some(sort)
}

// whether the conjunction of all p implies the conjunction of all q
fn implication_holds(
    env: &HashMap<Id, explicit::Type>,
    v_ty: explicit::Type,
    p: &[lambdal::Expr],
    q: &[lambdal::Expr]) -> bool {

    let mut z3 = Z3::new_with_binary("./z3");
    let mut solver = SMTLib2::new(Some(LIA));
    solver.set_logic(&mut z3);

    let mut senv: HashMap<Id, _> = HashMap::new();

    // Defining the symbolic vars x & y
    for (var, ty) in env {
        if let Some(sort) = sort_from_ty(ty) {
            let idx = solver.new_var(Some(&var), sort);
            senv.insert(var.clone(), idx);
        }
    }

    senv.insert(String::from("!v"), solver.new_var(Some("!v"), sort_from_ty(&v_ty).unwrap()));

    let mut ps: Vec<_> = Vec::new();
    for t in p {
        let pred = smt_from_expr(&mut solver, &senv, t);
        ps.push(pred);
    }

    let mut qs: Vec<_> = Vec::new();
    for t in q {
        let pred = smt_from_expr(&mut solver, &senv, t);
        qs.push(pred);
    }

    let p_all = match ps.len() {
        0 => solver.new_const(core::OpCodes::Const(true)),
        1 => ps[0],
        _ => solver.assert(core::OpCodes::And, &ps),
    };
    let q_all = match qs.len() {
        0 => solver.new_const(core::OpCodes::Const(true)),
        1 => qs[0],
        _ => solver.assert(core::OpCodes::And, &qs),
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
    renv: &HashMap<Id, Type>,
    a: &HashMap<Id, KInfo>,
    all_p: &Vec<(HashSet<Id>, LinkedList<Expr>, Box<Type>)>,
    qs: &HashSet<lambdal::Expr>) -> Option<Vec<lambdal::Expr>> {

    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));

    let mut curr_qs: Vec<lambdal::Expr> = Vec::new();
    'outer: for q in qs {
        for &(ref in_scope, ref path_constraints, ref p) in all_p {
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
            for var in in_scope {
                p.extend(expr_from_var(a, var, &renv[var]));
            }
            for pc in path_constraints {
                p.push(pc.clone());
            }

            if !implication_holds(env, TInt, &p, &[q.clone()]) {
                continue 'outer;
            }
        };
        curr_qs.push(q.clone());
    };

    curr_qs.sort();
    Some(curr_qs)
}

fn solve(
    env: &HashMap<Id, explicit::Type>,
    renv: &HashMap<Id, Type>,
    constraints: &LinkedList<STConstraints>,
    a: &HashMap<Id, KInfo>) -> Result<HashMap<Id, KInfo>> {

    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));

    for &(ref all_p, ref id) in constraints.iter() {

        // if we don't find the ID in our environment, it means we are
        // looking at unbound function parameters -- which means we can just look
        let ref qs = match a.get(id) {
            Some(q) => q.clone(),
            None => {
                let mut all_qs = HashSet::new();
                all_qs.insert(const_true.clone());
                // FIXME: shouldn't default to Int
                KInfo{
                    base: Base::Int,
                    all_qs: all_qs,
                    curr_qs: vec![const_true.clone()],
                }
            }
        };

        for &(ref in_scope, ref path_constraints, ref p) in all_p {
            let mut p = match *p {
                box T::Ref(_, _, box Liquid::E(ref expr)) => vec![expr.clone()],
                box T::Ref(_, _, box Liquid::K(ref p_id, _)) => {
                    let qs = match a.get(p_id) {
                        Some(ps) => ps.clone().curr_qs,
                        None => vec![const_true.clone()],
                    };
                    qs
                },
                box T::Fun(_, _, _) => {
                    panic!("unexpected {:?} -- should all be split() by now", p)
                }
            };

            for var in in_scope {
                p.extend(expr_from_var(a, var, &renv[var]));
            }
            for pc in path_constraints {
                p.push(pc.clone());
            }

            let implication = implication_holds(env, TInt, &p, &qs.curr_qs);
            if !implication {
                if qs.curr_qs[0] == const_true {
                    return err!("implication failure for -> true");
                }
                match weaken(env, renv, a, all_p, &qs.all_qs) {
                    Some(new_qs) => {
                        let mut new_a = a.clone();
                        new_a.insert(id.clone(), KInfo{
                            base: qs.base,
                            all_qs: qs.all_qs.clone(),
                            curr_qs: new_qs.clone(),
                        });
                        return solve(env, renv, constraints, &new_a);
                    }
                    None => {
                        return err!("Weaken failed for {:?}", p);
                    }
                }
            }
        };
    };

    Ok(a.clone())
}

fn conjoin(qs: &[Expr]) -> Op {
    use common::Op2;
    if let Expr::Op(op) = qs[0].clone() {
        match qs[1..].len() {
            0 => op,
            _ => Op::Op2(Op2::And, box op, box conjoin(&qs[1..])),
        }
    } else {
        panic!("expected q to be an Op, not {:?}", qs)
    }
}

fn concretize_liquid(_: &HashMap<Id, Type>, a: &HashMap<Id, KInfo>, lqdt: &Liquid) -> Liquid {
    match *lqdt {
        Liquid::E(ref expr) => Liquid::E(expr.clone()),
        Liquid::K(ref id, _) => {
            // TODO: substitutions
            if !a.contains_key(id) {
                //panic!("NOT FOUND {}", id);
                return Liquid::E(Expr::Op(Op::Imm(Imm::Bool(true))));
            }
            let ref qs = a[id].curr_qs;
            match qs.len() {
                0 => Liquid::E(Expr::Op(Op::Imm(Imm::Bool(true)))),
                _ => Liquid::E(Expr::Op(conjoin(&qs))),
            }
        }
    }
}

fn concretize_ty(renv: &HashMap<Id, Type>, a: &HashMap<Id, KInfo>, ty: &Type) -> Type {
    match *ty {
        T::Ref(ref in_scope, base, ref predicate) => T::Ref(in_scope.clone(), base, box concretize_liquid(renv, a, predicate)),
        T::Fun(ref id, ref xty, ref yty) => T::Fun(id.clone(), box concretize_ty(renv, a, xty), box concretize_ty(renv, a, yty)),
    }
}

// turn types that have k in them into dependent types w/o ks
fn concretize(renv: &HashMap<Id, Type>, a: &HashMap<Id, KInfo>) -> HashMap<Id, Type> {
    let mut result = HashMap::new();

    for (id, ty) in renv {
        result.insert(id.clone(), concretize_ty(renv, a, ty));
    }

    result
}

pub fn infer(expr: &Expr, env: &HashMap<Id, explicit::Type>, q: &[implicit::Expr]) -> Result<HashMap<Id, Type>> {
    let mut k_env = KEnv::new(env);
    println!("infer:\t{:?}\n", expr);
    let (top, constraint_list) = cons_expr(&mut k_env, &LinkedList::new(), expr);

    {
        println!("Liquid Γ:");
        let mut ids: Vec<_> = k_env.refined.keys().clone().collect();
        ids.sort();
        for id in ids {
            println!("{}:\t{:?}", id, k_env.refined[id]);
        };
    }
    println!("");

    let mut constraints: HashMap<Idx, Constraint> = HashMap::new();
    split(&mut constraints, &constraint_list);

    println!("Subtype Constraints");
    let mut by_id: HashMap<Id, Vec<(HashSet<Id>, LinkedList<Expr>, Box<Type>)>> = HashMap::new();
    let mut ckeys: Vec<Idx> = constraints.keys().map(|i| *i).collect();
    // sort constrains to make output easier to understand
    ckeys.sort_by_key(|k| {
        if let ((_, _), C::Subtype(_, box T::Ref(_, _, box Liquid::K(ref id, _)))) = constraints[k] {
            i32::from_str(&id[2..]).unwrap_or(-1)
        } else {
            -2
        }
    });

    // group subtyping constraints by supertype
    for ckey in ckeys {
        let ref c = constraints[&ckey];
        if let &((ref in_scope, ref path), C::Subtype(ref p, ref e)) = c {
            //println!("Γ ⊢ {:?}   \t<: {:?} (PATH: {:?})", p, e, path);
            println!("Γ ⊢ {:?}   \t<: {:?} (PATH: {:?}) (IN_SCOPE: {:?})", p, e, path, in_scope);
            if let box T::Ref(_, _, box Liquid::K(ref id, _)) = *e {
                let mut antecedent = vec![(in_scope.clone(), path.clone(), p.clone())];
                if by_id.contains_key(id) {
                    let mut others = by_id[id].clone();
                    others.append(&mut antecedent);
                    by_id.insert(id.clone(), others);
                } else {
                    by_id.insert(id.clone(), antecedent);
                }
            } else {
                println!("TODO: subtype constraint for non-liquid type {:?}", e);
            }
        }
    }
    println!("");

    let mut all_constraints: LinkedList<STConstraints> = LinkedList::new();
    for (id, v) in by_id {
        //println!("->{}:\t{:?}", id, v);
        all_constraints.push_back((v.clone(), id.clone()));
    }

    let a = build_a(&constraints, env, q);
    let min_a = solve(env, &k_env.refined, &all_constraints, &a)?;

    use std::str::FromStr;
    {
        println!("a:");
        let mut ids: Vec<_> = a.keys().clone().collect();
        ids.sort_by_key(|id| i32::from_str(&id[2..]).unwrap_or(-1));
        for id in ids {
            println!("{}\t{:?}", id, a[id].all_qs);
        };
    }
    {
        use std::str::FromStr;
        println!("min_a:");
        let mut ids: Vec<_> = min_a.keys().clone().collect();
        ids.sort_by_key(|id| i32::from_str(&id[2..]).unwrap_or(-1));
        for id in ids {
            println!("{}\t{:?}", id, min_a[id].curr_qs);
        };
    }

    println!("\nRESULT TYPE: {:?}", concretize_ty(&k_env.refined, &min_a, &top));
    Ok(concretize(&k_env.refined, &min_a))
}

#[macro_export]
macro_rules! q(
    ($s:expr) => { {
        use implicit_parse;
        use tok::Tokenizer;
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        let iexpr = match implicit_parse::parse_Program(&s, tokenizer) {
            Ok(iexpr) => *iexpr,
            Err(e) => {
                die!("parse_Program({}): {:?}", $s, e);
            }
        };
        iexpr
    } }
);

#[cfg(test)]
macro_rules! expr(
    ($s:expr) => { {
        use lambdal;
        use implicit_parse;
        use tok::Tokenizer;
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
    if !implication_holds(&env, TInt, &p, &q) {
        die!("1 expected {:?} => {:?}", p, q);
    }

    let p = [
        expr!("x <= y ∧ ν = y"),
    ];

    let q = [
        expr!("ν < 0 ∧ ν >= x ∧ ν >= y"),
    ];

    // but this shouldn't
    if implication_holds(&env, TInt, &p, &q) {
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
