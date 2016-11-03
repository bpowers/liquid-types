#[cfg(test)]
use std;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use common;
use implicit;
use explicit;

use implicit::Expr;
use common::{Id, Result};
use refined::{Base, T};

use rustproof_libsmt::backends::smtlib2::SMTLib2;
use rustproof_libsmt::backends::backend::*;
use rustproof_libsmt::backends::z3::Z3;
use rustproof_libsmt::theories::{core, integer};
use rustproof_libsmt::logics::lia::LIA;

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
    E(implicit::Expr),
    K(Id, Box<LinkedList<Expr>>), // list of pending substitutions
}

pub type Type = T<Liquid>;
pub type Implication = (LinkedList<Expr>, Box<Type>, Box<Type>);

pub type Idx = i32; // constraint index

pub type Constraint = ((HashSet<Id>, LinkedList<Expr>), C); // Boolean valued expressions & their environments

#[derive(Debug, Clone)]
pub struct KInfo {
    all_qs: HashSet<implicit::Expr>,
    curr_qs: HashSet<implicit::Expr>,
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KEnv {
    shape: HashMap<Id, explicit::Type>,
    env_id: String,
    next_id: i32,
}

fn hm_shape(env: &HashMap<Id, explicit::Type>, expr: &Expr) -> explicit::Type {
    use implicit::Expr::*;
    use explicit::Type::*;

    match *expr {
        Var(ref id) => env.get(id).unwrap().clone(),
        Const(common::Const::Int(_)) => TInt,
        Const(common::Const::Bool(_)) => TBool,
        Op2(op, _, _) => explicit::opty(op),
        Fun(ref id, ref e) => TFun(box env.get(id).unwrap().clone(), box hm_shape(env, e)),
        App(ref e1, _) => {
            if let TFun(_, e2) = hm_shape(env, e1) {
                *e2
            } else {
                panic!("expected TFun, not {:?}", expr);
            }
        }
        If(_, ref e2, _) => hm_shape(env, e2),
        Let(_, _, ref e2) => hm_shape(env, e2),
        Fix(_, ref e) => hm_shape(env, e),
        MkArray(_, _) => TIntArray,
        GetArray(_, _) => TInt,
        SetArray(_, _, _) => TIntArray,
        Star => panic!("star found when it shouldn't be"),
        V => panic!("v found when it shouldn't be"),
    }
}

impl KEnv {
    fn new(shape: &HashMap<Id, explicit::Type>) -> KEnv {
        KEnv {
            shape: shape.clone(),
            env_id: String::from("κ"), // ν
            next_id: 0,
        }
    }

    fn fresh_ty(&mut self, env: &Env, ty: &explicit::Type) -> Type {
        let id = self.next_id;
        self.next_id += 1;

        let base = match *ty {
            explicit::Type::TInt => Base::Int,
            explicit::Type::TBool => Base::Bool,
            _ => panic!("FIXME: handle {:?}", ty),
        };
        let k = Liquid::K(format!("!k{}", id), box LinkedList::new());
        T::Ref(env.in_scope(), base, box k)
    }

    fn fresh(&mut self, env: &Env, expr: &Expr) -> Type {
        if let &Expr::Fun(ref id, ref e) = expr {
            let t1 = &self.shape.get(id).unwrap().clone();
            let fx = self.fresh_ty(env, t1);
            let f = self.fresh(env, e);
            T::Fun(id.clone(), box fx, box f)
        } else {
            let ty = hm_shape(&env.shape, expr);
            self.fresh_ty(env, &ty)
        }
    }
}

fn ty<'a>(_: &mut KEnv, _: &Env, c: &common::Const) -> Type {
    use common::Op2;
    use common::Const::*;
    use self::Liquid::E;

    let base = match *c {
        Int(_) => Base::Int,
        Bool(_) => Base::Bool,
    };

    println!("ty({:?})", base);

    // {ν : int | ν = 3 }
    let eq = E(Expr::Op2(Op2::Eq, box Expr::V, box Expr::Const(*c)));
    T::Ref(HashSet::new(), base, box eq)
}

fn base(ty: &Type) -> Option<Base> {
    match *ty {
        T::Ref(_, b, _) => Some(b),
        _ => None,
    }
}

fn subst(_: &Id, _: &Expr, ty: &Type) -> Type {
    println!("TODO: subst");
    ty.clone()
}

pub fn cons<'a>(k_env: &mut KEnv, env: &Env, expr: &Expr) -> (Type, LinkedList<Constraint>) {
    use implicit::Expr::*;
    use common::Op2::Eq;

    match *expr {
        Var(ref id) => {
            let ty: Type = if let Some(b) = base(&env.get(id)) {
                let eq = Op2(Eq, box V, box Var(id.clone()));
                T::Ref(env.in_scope(), b, box Liquid::E(eq))
            } else {
                println!("{} not base -- using just env ({:?})", id, env.get(id));
                env.get(id)
            };
            (ty, LinkedList::new())
        }
        Const(ref c) => {
            (ty(k_env, &env, c), LinkedList::new())
        }
        Op2(op, ref e1, ref e2) => {
            let (_, mut c1) = cons(k_env, env, e1);
            let (_, mut c2) = cons(k_env, env, e2);
            c1.append(&mut c2);

            let ty = explicit::opty(op);
            let base = match ty {
                explicit::Type::TInt => Base::Int,
                explicit::Type::TBool => Base::Bool,
                _ => panic!("FIXME: handle {:?}", ty),
            };

            let eq = Op2(Eq, box V, box expr.clone());
            let f = T::Ref(env.in_scope(), base, box Liquid::E(eq));

            (f, c1)
        }
        If(ref e1, ref e2, ref e3) => {
            let mut env_t = env.clone();
            let mut env_f = env.clone();
            env_t.add_constraint(&e1.clone());
            env_f.add_constraint(&App(box Var(String::from("not")), e1.clone()));

            let f = k_env.fresh(&env, expr);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons(k_env, &env, e1);
            let (f2, mut c2) = cons(k_env, &env_t, e2);
            let (f3, mut c3) = cons(k_env, &env_f, e3);
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
        Fun(ref x, ref e) => {
            let mut env = env.clone();
            let fx = k_env.fresh(&env, &Var(x.clone()));
            env.insert(x, &fx);
            let f = k_env.fresh(&env, e);
            let (fe, mut c) = cons(k_env, &env, e);
            // Γ ⊢ (x:fx → f)
            c.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,x:fx ⊢ (fe <: f)
            c.push_back((env.snapshot(), C::Subtype(box fe.clone(), box f.clone())));

            (f, c)
        }
        Fix(ref x, ref e) => {
            // const w/ ∀α.(α→α)→α
            let mut env = env.clone();
            let fx = k_env.fresh(&env, e);
            env.insert(x, &fx);

            // FIXME
            cons(k_env, &env, e)
        }
        Let(ref id, ref e1, ref e2) => {
            let mut env = env.clone();

            let f = k_env.fresh(&env, expr);
            let (f1, mut c1) = cons(k_env, &env, e1);
            env.insert(id, &f1);
            let (f2, mut c2) = cons(k_env, &env, e2);
            c1.append(&mut c2);
            // Γ ⊢ (f)
            c1.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,x:f1 ⊢ (f2 <: f)
            c1.push_back((env.snapshot(), C::Subtype(box f2.clone(), box f.clone())));

            (f, c1)
        }
        App(ref e1, ref e2) => {
            let (f1, mut c1) = cons(k_env, env, e1);
            println!("## {:?}\t:\t{:?}", e1, f1);
            let (f2, mut c2) = cons(k_env, env, e2);
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
        _ => {
            println!("unhandled {:?}", expr);
            (T::Ref(env.in_scope(), Base::Bool, box Liquid::E(Const(common::Const::Bool(true)))), LinkedList::new())
        }
    }
}

fn split(map: &mut HashMap<Idx, Constraint>, constraints: &LinkedList<Constraint>) {
    let mut idx = 1;

    for c in constraints.iter() {
        if let &((ref scope, ref pathc), C::Subtype(box T::Fun(_, ref tx1, ref t1), box T::Fun(ref x2, ref tx2, ref t2))) = c {
            let mut contra_cs: LinkedList<Constraint> = LinkedList::new();

            contra_cs.push_back(((scope.clone(), pathc.clone()), C::Subtype(tx2.clone(), tx1.clone())));

            let mut rscope = scope.clone();
            rscope.insert(x2.clone());
            contra_cs.push_back(((rscope, pathc.clone()), C::Subtype(t1.clone(), t2.clone())));

            // recurse
            split(map, &contra_cs);
        } else if let &((ref scope, ref pathc), C::WellFormed(box T::Fun(ref id, _, ref t))) = c {
            let mut wf_cs: LinkedList<Constraint> = LinkedList::new();

            let mut scope = scope.clone();
            scope.insert(id.clone());
            wf_cs.push_back(((scope, pathc.clone()), C::WellFormed(t.clone())));

            // recurse
            split(map, &wf_cs);
        } else {
            map.insert(idx, c.clone());
            idx += 1;
        }
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
fn qstar(_: &Id, in_scope: &HashSet<Id>, _: &HashMap<Id, explicit::Type>, qset: &[implicit::Expr]) -> HashSet<implicit::Expr> {
    let mut qstar: HashSet<implicit::Expr> = HashSet::new();

    for tmpl in qset {
        for v in in_scope.iter() {
            match replace(v, tmpl) {
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

fn build_a(constraints: &HashMap<Idx, Constraint>, env: &HashMap<Id, explicit::Type>, q: &[implicit::Expr]) -> HashMap<Id, KInfo> {
    let mut a: HashMap<Id, KInfo> = HashMap::new();

    for (_, c) in constraints.iter() {
        if let &((_, _), C::WellFormed(ref ty)) = c {
            if let &box T::Ref(ref in_scope, _, box Liquid::K(ref id, _)) = ty {
                // TODO: subst?
                let all_qs = qstar(id, in_scope, env, q);
                a.insert(id.clone(), KInfo{
                    all_qs: all_qs.clone(),
                    curr_qs: all_qs,
                });
            } else {
                panic!("WellFormed with E doesn't make sense: {:?}.", ty)
            }
        }
        // TODO: track antecedents that reference each k
    }

    a
}

fn expr_to_term(s: &mut SMTLib2<LIA>, vars: &HashMap<String, <SMTLib2<LIA> as SMTBackend>::Idx>, q: &implicit::Expr) -> <SMTLib2<LIA> as SMTBackend>::Idx {
    use implicit::Expr as I;
    use common::Const::*;
    use common::Op2;

    match *q {
        I::Var(ref id)                        => vars[id],
        I::Const(Bool(c))                     => s.new_const(core::OpCodes::Const(c)),
        I::Const(Int(c))                      => s.new_const(integer::OpCodes::Const(c as u64)),
        I::Op2(op, ref l, ref r)          => {
            let il = expr_to_term(s, vars, l);
            let ir = expr_to_term(s, vars, r);
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
        I::V                                  => vars["!v"],
        I::Star                               => unreachable!(),
        _ => {
            panic!("ugh");
        }
        // I::Fun(ref id, ref e)                 => I::Fun(id.clone(), box replace(v, e))),
        // I::App(ref e1, ref e2)                => I::App(box replace(v, e1)), box replace(v, e2))),
        // I::If(ref e1, ref e2, ref e3)         => I::If(box replace(v, e1)), box replace(v, e2)), box replace(v, e3))),
        // I::Let(ref id, ref e1, ref e2)        => I::Let(id.clone(), box replace(v, e1)), box replace(v, e2))),
        // I::Fix(ref id, ref e)                 => I::Fix(id.clone(), box replace(v, e))),
        // I::MkArray(ref sz, ref n)             => I::MkArray(box replace(v, sz)), box replace(v, n))),
        // I::GetArray(ref id, ref idx)          => I::GetArray(box replace(v, id)), box replace(v, idx))),
        // I::SetArray(ref id, ref idx, ref var) => I::SetArray(box replace(v, id)), box replace(v, idx)), box replace(v, var))),
    }
}

// whether the conjunction of all p implies the conjunction of all q
fn implication_holds(env: &HashMap<Id, explicit::Type>, p: &[implicit::Expr], q: &[implicit::Expr]) -> bool {
    let mut z3 = Z3::new_with_binary("./z3");
    let mut solver = SMTLib2::new(Some(LIA));
    solver.set_logic(&mut z3);

    let mut senv: HashMap<Id, _> = HashMap::new();

    // Defining the symbolic vars x & y
    for (var, ty) in env {
        let sty = match *ty {
            explicit::Type::TInt => integer::Sorts::Int,
            _ => panic!("TODO: more sorts than int"),
        };
        let idx = solver.new_var(Some(&var), sty);
        senv.insert(var.clone(), idx);
    }
    senv.insert(String::from("v"), solver.new_var(Some("v"), integer::Sorts::Int));

    let mut ps: Vec<_> = Vec::new();
    for t in p {
        ps.push(expr_to_term(&mut solver, &senv, t));
    }

    let mut qs: Vec<_> = Vec::new();
    for t in q {
        qs.push(expr_to_term(&mut solver, &senv, t));
    }

    let p_all = solver.assert(core::OpCodes::And, &ps);
    let q_all = solver.assert(core::OpCodes::And, &qs);
    let imply = solver.assert(core::OpCodes::Imply, &[p_all, q_all]);
    let _ = solver.assert(core::OpCodes::Not, &[imply]);

    let (_, sat) = solver.solve(&mut z3, false);

    match sat {
        SMTRes::Unsat(_, _) => true,
        _ => false,
    }
}

fn solve(constraints: &LinkedList<Implication>, a: &mut HashMap<Id, KInfo>) -> Result<HashMap<Id, KInfo>> {

    for &(ref path, ref a, ref p) in constraints.iter() {
        println!("C\t{:?}\n\t\t{:?}\n\t\t\t{:?}", path, a, p);
    };

    Ok(a.clone())
}

pub fn infer(expr: &Expr, env: &HashMap<Id, explicit::Type>, q: &[implicit::Expr]) -> Result<HashMap<Id, HashSet<implicit::Expr>>> {
    let mut k_env = KEnv::new(env);
    let (_, constraint_list) = cons(&mut k_env, &Env::new(env), expr);

    let mut constraints: HashMap<Idx, Constraint> = HashMap::new();
    split(&mut constraints, &constraint_list);

    let mut a = build_a(&constraints, env, q);

    let mut all_constraints: LinkedList<Implication> = LinkedList::new();
    for (_, c) in constraints.iter() {
        if let &((_, ref path), C::Subtype(ref p, ref e)) = c {
            all_constraints.push_back((path.clone(), p.clone(), e.clone()));
        }
    }

    let min_a = solve(&all_constraints, &mut a)?;

    let mut res = HashMap::new();
    for (k, v) in min_a {
        res.insert(k, v.curr_qs.clone());
    }

    Ok(res)
}

#[test]
fn test_implication() {
    use implicit::Expr::*;
    use common::Op2::*;

    let mut env: HashMap<Id, explicit::Type> = HashMap::new();
    env.insert(String::from("x"), explicit::Type::TInt);
    env.insert(String::from("y"), explicit::Type::TInt);

    let p = [
        Op2(And,
            box Op2(LTE, box Var(String::from("x")), box Var(String::from("y"))),
            box Op2(Eq, box Var(String::from("v")), box Var(String::from("y")))),
    ];

    let q = [
        Op2(And,
            box Op2(GTE, box Var(String::from("v")), box Var(String::from("x"))),
            box Op2(GTE, box Var(String::from("v")), box Var(String::from("y")))),
    ];

    // expect this to hold
    if !implication_holds(&env, &p, &q) {
        die!("expected {:?} => {:?}", p, q);
    }

    let p = [
        Op2(And,
            box Op2(LTE, box Var(String::from("x")), box Var(String::from("y"))),
            box Op2(Eq, box Var(String::from("v")), box Var(String::from("y")))),
    ];

    let q = [
        Op2(And,
            box Op2(LT, box Var(String::from("v")), box Const(common::Const::Int(0))),
            box Op2(And,
                    box Op2(GTE, box Var(String::from("v")), box Var(String::from("x"))),
                    box Op2(GTE, box Var(String::from("v")), box Var(String::from("y"))))),
    ];

    // but this shouldn't
    if implication_holds(&env, &p, &q) {
        die!("expected {:?} => {:?}", p, q);
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
