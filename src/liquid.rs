use std;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::error;

use common;
use refined;
use explicit;
use typed;

use common::Result;
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
pub enum Liquid {
    E(typed::Expr<explicit::Type>),
    K(Metavar, Box<LinkedList<typed::Expr<explicit::Type>>>), // list of pending substitutions
}

pub type Type = T<Env, Liquid>;
pub type Expr = typed::Expr<Type>;

pub type Constraint = Expr; // Boolean valued expressions

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Env {
    refined_env: Box<HashMap<String, Type>>,
    path_constraints: Box<LinkedList<Expr>>,
}

impl Env {
    fn new() -> Env {
        Env {
            refined_env: box HashMap::new(),
            path_constraints: box LinkedList::new(),
        }
    }

    fn get(&self, s: &String) -> Type {
        self.refined_env.get(s).unwrap().clone()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KEnv {
    env_id: String,
    next_id: i32,
}

impl KEnv {
    fn new() -> KEnv {
        KEnv {
            env_id: String::from("κ"), // ν
            next_id: 0,
        }
    }

    fn fresh(&mut self, env: &Env, ty: &explicit::Type) -> Type {
        let id = self.next_id;
        self.next_id += 1;

        let base = match ty {
            &explicit::Type::TInt => Base::Int,
            &explicit::Type::TBool => Base::Bool,
            _ => panic!("FIXME: handle more base types in alloc")
        };
        let k = Liquid::K((id, self.env_id.clone()), box LinkedList::new());
        T::Ref(box env.clone(), base, box k)
    }
}

fn ty<'a>(k_env: &mut KEnv, env: &Env, c: &common::Const) -> (Env, Type) {
    use common::Const::*;
    match *c {
        Int(ref i) => {
            println!("ty(int)");
            (env.clone(), k_env.fresh(env, &explicit::Type::TInt))
        }
        Bool(ref b) => {
            println!("ty(bool)");
            (env.clone(), k_env.fresh(env, &explicit::Type::TBool))
        }
    }
}

fn base(ty: &Type) -> Option<Base> {
    match *ty {
        T::Ref(_, b, _) => Some(b),
        _ => None,
    }
}

pub fn cons<'a>(k_env: &mut KEnv, env: &Env, expr: &explicit::Expr) -> (Type, LinkedList<Constraint>) {
    use typed::Expr::*;

    let mut env = env.clone();
    match expr {
        &Var(ref id) => {
            let ty: Type = if let Some(b) = base(&env.get(id)) {
                let eq = Op2(common::Op2::Eq, box V, box Var(id.clone()));
                T::Ref(box env.clone(), b, box Liquid::E(eq))
            } else {
                env.get(id)
            };
            (ty, LinkedList::new())
        }
        &Const(ref c) => {
            let (env, t) = ty(k_env, &env, c);
            (t, LinkedList::new())
        }
        &If(ref e1, ref e2, ref e3) => {
            // FIXME: need to pass up type of expr?
            let f = k_env.fresh(&env, &explicit::Type::TInt);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons(k_env, &env, e1);
            let (t2, mut c2) = cons(k_env, &env, e2);
            let (t3, mut c3) = cons(k_env, &env, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // TODO: add constraints:
            // Γ ⊢ (f)
            // Γ,e1 ⊢ (f2 <: f)
            // Γ,¬e1 ⊢ (f3 <: f)
            (f, c1)
        }
        &Fun(ref x, ref ty, ref e) => {
            let fx = k_env.fresh(&env, ty);
            let f = k_env.fresh(&env, ty);
            let ffn = T::Fun(x.clone(), box fx.clone(), box f.clone());
            let (fe, c) = cons(k_env, &env, e);
            // TODO: add constraints:
            // Γ ⊢ (x:fx → f)
            // Γ,x:fx ⊢ (fe <: f)
            (ffn, c)
        }
        _ => {
            (T::Ref(box env.clone(), Base::Bool, box Liquid::E(typed::Expr::Const(common::Const::Bool(true)))), LinkedList::new())
        }
    }
}

pub fn infer(expr: &explicit::Expr) -> Result<Expr> {
    let mut k_env = KEnv::new();
    let (f, c) = cons(&mut k_env, &Env::new(), expr);
    println!("CONS:\n");
    println!("f\t{:?}", f);
    println!("c\t{:?}", c);
    Ok(typed::Expr::Const(common::Const::Bool(false)))
}
