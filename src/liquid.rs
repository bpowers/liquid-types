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
pub enum Liquid {
    E(implicit::Expr),
    K(Metavar, Box<LinkedList<Expr>>), // list of pending substitutions
}

pub type Type = T<Liquid>;

pub type Constraint = (HashSet<Id>, Expr); // Boolean valued expressions & their environments

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

    fn fresh(&mut self, env: &Env, x: &Id) -> Type {
        let id = self.next_id;
        self.next_id += 1;

        let base = match self.shape.get(x).unwrap() {
            &explicit::Type::TInt => Base::Int,
            &explicit::Type::TBool => Base::Bool,
            _ => panic!("FIXME: handle more base types in alloc")
        };
        let k = Liquid::K((id, self.env_id.clone()), box LinkedList::new());
        T::Ref(env.in_scope(), base, box k)
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
            //let f = k_env.fresh(&env, &explicit::Type::TInt);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons(k_env, &env, e1);
            // add e1 to path constraints in env_t
            // add not e1 to path constraints in env_f
            let (f2, mut c2) = cons(k_env, &env_t, e2);
            let (f3, mut c3) = cons(k_env, &env_f, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // Γ ⊢ (f)
            c1.push_back((env.in_scope(), WellFormed(String::from("__if"))));

            // TODO: add constraints:
            // Γ,e1 ⊢ (f2 <: f)
            // Γ,¬e1 ⊢ (f3 <: f)

            (f2, c1)
        }
        &Fun(ref x, ref e) => {
            let mut env = env.clone();
            let fx = k_env.fresh(&env, x);
            env.insert(x, &fx);
            // FIXME: Trump-esque WRONG.
            let f = k_env.fresh(&env, x);
            let ffn = T::Fun(x.clone(), box fx.clone(), box f.clone());
            let (fe, mut c) = cons(k_env, &env, e);
            // Γ ⊢ (x:fx → f)
            c.push_back((env.in_scope(), WellFormed(String::from("__fun"))));

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
