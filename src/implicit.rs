pub use common::{Id, Op2, Const};

#[derive(Debug)]
pub enum Exp {
    Var(Id),
    Const(Const),
    Op2(Op2, Box<Exp>, Box<Exp>),
    Fun(Id, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
    Let(Id, Box<Exp>, Box<Exp>),
    Fix(Id, Box<Exp>),
    Empty,
    Cons(Box<Exp>, Box<Exp>),
    Head(Box<Exp>),
    Tail(Box<Exp>),
    IsEmpty(Box<Exp>),
    Pair(Box<Exp>, Box<Exp>),
    ProjL(Box<Exp>),
    ProjR(Box<Exp>),
}
