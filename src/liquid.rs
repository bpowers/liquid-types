use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use common;
use implicit;
use explicit;
use typed;

use implicit::Expr;
use common::{Id, Result};
use refined::{Base, T};
pub use explicit::Metavar;

// Qbc (bounds checking)
// X: 0, *, len *
// ν < X
// ν <= X
// ν = X
// ν >= X
// ν > X

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum C {
    WellFormed(Box<Type>),
    SubType(Box<Type>, Box<Type>),
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Liquid {
    E(implicit::Expr),
    K(Metavar, Box<LinkedList<Expr>>), // list of pending substitutions
}

pub type Type = T<Liquid>;

pub type Constraint = ((HashSet<Id>, LinkedList<Expr>), C); // Boolean valued expressions & their environments

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
        self.refined_env.get(s).unwrap().clone()
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
        Let(ref id, ref e1, ref e2) => hm_shape(env, e2),
        Fix(ref id, ref e) => hm_shape(env, e),
        MkArray(ref sz, ref n) => TIntArray,
        GetArray(ref id, ref idx) => TInt,
        SetArray(ref id, ref idx, ref v) => TIntArray,
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
        let k = Liquid::K((id, self.env_id.clone()), box LinkedList::new());
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

fn ty<'a>(k_env: &mut KEnv, env: &Env, c: &common::Const) -> Type {
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

pub fn cons<'a>(k_env: &mut KEnv, env: &Env, expr: &Expr) -> (Type, LinkedList<Constraint>) {
    use implicit::Expr::*;
    use common::Op2::Eq;

    match expr {
        &Var(ref id) => {
            let ty: Type = if let Some(b) = base(&env.get(id)) {
                let eq = Op2(Eq, box V, box Var(id.clone()));
                T::Ref(env.in_scope(), b, box Liquid::E(eq))
            } else {
                env.get(id)
            };
            (ty, LinkedList::new())
        }
        &Const(ref c) => {
            (ty(k_env, &env, c), LinkedList::new())
        }
        &If(ref e1, ref e2, ref e3) => {
            let mut env_t = env.clone();
            let mut env_f = env.clone();
            env_t.add_constraint(&e1.clone());
            env_f.add_constraint(&App(box Var(String::from("not")), e1.clone()));

            // FIXME: need to pass up type of expr?
            let f = k_env.fresh(&env, expr);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons(k_env, &env, e1);
            // add e1 to path constraints in env_t
            // add not e1 to path constraints in env_f
            let (f2, mut c2) = cons(k_env, &env_t, e2);
            let (f3, mut c3) = cons(k_env, &env_f, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // Γ ⊢ (f)
            c1.push_back((env.snapshot(), C::WellFormed(box f.clone())));

            // TODO: add constraints:
            // Γ,e1 ⊢ (f2 <: f)
            // Γ,¬e1 ⊢ (f3 <: f)

            (f2, c1)
        }
        &Fun(ref x, ref e) => {
            let mut env = env.clone();
            let fx = k_env.fresh(&env, &Var(x.clone()));
            env.insert(x, &fx);
            // FIXME: Trump-esque WRONG.
            let f = k_env.fresh(&env, e);
            let ffn = T::Fun(x.clone(), box fx.clone(), box f.clone());
            let (fe, mut c) = cons(k_env, &env, e);
            // Γ ⊢ (x:fx → f)
            c.push_back((env.snapshot(), C::WellFormed(box f.clone())));

            // TODO: add constraints:
            // Γ,x:fx ⊢ (fe <: f)

            (ffn, c)
        }
        _ => {
            (T::Ref(env.in_scope(), Base::Bool, box Liquid::E(Const(common::Const::Bool(true)))), LinkedList::new())
        }
    }
}

pub fn infer(expr: &Expr, env: &HashMap<Id, explicit::Type>) -> Result<Expr> {
    let mut k_env = KEnv::new(env);
    let (f, c) = cons(&mut k_env, &Env::new(env), expr);
    println!("CONS:\n");
    println!("f\t{:?}", f);
    println!("c\t{:?}", c);

    // TODO:
    // let a = Solve(Split(c), λκ.Inst(Γ, e, Q)) in
    // a(f)

    Ok(Expr::Const(common::Const::Bool(false)))
}
