use std;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::error;

use common;
use refined;
use explicit;
use typed;

use common::Result;
use refined::T;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Q {
    True,
    // logical quantifiers in Q
    And(Box<Q>, Box<Q>)
}

pub type Type = T<Q>;
pub type Expr = typed::Expr<Type>;

pub type Constraint = Expr; // Boolean valued expressions

#[derive(Debug, Clone)]
pub struct Env<'a> {
    env_id: &'a str,
    next_id: i32,

    hm_env: Box<HashMap<String, explicit::Type>>,
    refined_env: Box<HashMap<String, explicit::Type>>,
}

impl<'a> Env<'a> {
    fn new() -> Env<'a> {
        Env {
            env_id: "ν",
            next_id: 0,
            hm_env: box HashMap::new(),
            refined_env: box HashMap::new(),
        }
    }

    fn alloc(&mut self, s: &String, expr: &explicit::Type) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        self.hm_insert(s, expr);
        T::Metavar((id, s.clone()))
    }

    fn alloc_empty(&mut self) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        T::Metavar((id, String::from(self.env_id)))
    }

    fn hm_get(&self, s: &String) -> explicit::Type {
        self.hm_env.get(s).unwrap().clone()
    }

    fn hm_insert(&mut self, s: &String, expr: &explicit::Type) {
        self.hm_env.insert(s.clone(), expr.clone());
    }
}

fn ty<'a>(env: &Env<'a>, c: &common::Const) -> (Env<'a>, Type) {
    use common::Const::*;
    match *c {
        Int(ref i) => {
            println!("ty(int)");
            (env.clone(), T::Ref(String::from("i"), refined::Base::Int, box Q::True))
        }
        Bool(ref b) => {
            println!("ty(bool)");
            (env.clone(), T::Ref(String::from("b"), refined::Base::Bool, box Q::True))
        }
    }
}

pub fn cons<'a>(env: &Env<'a>, expr: &explicit::Expr) -> (Env<'a>, Type, LinkedList<Constraint>) {
    use typed::Expr::*;

    let mut env = env.clone();
    match expr {
        &Var(ref id) => {
            (env, T::Ref(String::from("var"), refined::Base::Bool, box Q::True), LinkedList::new())
        }
        &Const(ref c) => {
            let (env, t) = ty(&env, c);
            (env, t, LinkedList::new())
        }
        &If(ref e1, ref e2, ref e3) => {
            let f = env.alloc_empty();
            // type of e1 has already been verified to be a bool by HM
            let (env, _, mut c1) = cons(&env, e1);
            let (env, t2, mut c2) = cons(&env, e2);
            let (env, t3, mut c3) = cons(&env, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // TODO: add constraints:
            // Γ ⊢ (f)
            // Γ,e1 ⊢ (f2 <: f)
            // Γ,¬e1 ⊢ (f3 <: f)
            (env, f, c1)
        }
        &Fun(ref x, ref ty, ref e) => {
            let fx = env.alloc(x, ty);
            let f = env.alloc_empty();
            let ffn = T::Fun(x.clone(), box fx.clone(), box f.clone());
            let (env, fe, c) = cons(&env, e);
            // TODO: add constraints:
            // Γ ⊢ (x:fx → f)
            // Γ,x:fx ⊢ (fe <: f)
            (env, ffn, c)
        }
        _ => {
            (env, T::Ref(String::from("idk"), refined::Base::Bool, box Q::True), LinkedList::new())
        }
    }
}

pub fn infer(expr: &explicit::Expr) -> Result<refined::Expr> {
    let (env, f, c) = cons(&Env::new(), expr);
    println!("CONS:\n");
    println!("env\t{:?}", env);
    println!("f\t{:?}", f);
    println!("c\t{:?}", c);
    Ok(typed::Expr::Const(common::Const::Bool(false)))
}
