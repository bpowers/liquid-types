use std;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::error;

use common;
use refined;
use explicit;
use typed;

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

#[derive(Clone)]
pub struct Env<'a> {
    env_id: &'a str,
    next_id: i32,
}

impl<'a> Env<'a> {
    fn new() -> Env<'a> {
        Env {
            env_id: "ν",
            next_id: 0,
        }
    }

    fn alloc(&mut self, s: &String) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        refined::T::Metavar((id, s.clone()))
    }

    fn alloc_empty(&mut self) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        refined::T::Metavar((id, String::from(self.env_id)))
    }
}

pub fn cons<'a>(env: &Env<'a>, expr: &explicit::Expr) -> (Env<'a>, Type, LinkedList<Constraint>) {
    (env.clone(), refined::T::Ref(String::from("idk"), refined::Base::Bool, box Q::True), LinkedList::new())
}

pub fn infer(expr: &explicit::Expr) -> refined::Expr {
    let env = Env::new();
    let (_, f, c) = cons(&env, expr);
    typed::Expr::Const(common::Const::Bool(false))
}
