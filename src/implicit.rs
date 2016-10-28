pub use common::{Id, Op2, Const};

#[derive(Debug)]
pub enum Expr {
    Var(Id),
    Const(Const),
    Op2(Op2, Box<Expr>, Box<Expr>),
    Fun(Id, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Id, Box<Expr>, Box<Expr>),
    Fix(Id, Box<Expr>),
    Empty,
    Cons(Box<Expr>, Box<Expr>),
    Head(Box<Expr>),
    Tail(Box<Expr>),
    IsEmpty(Box<Expr>),
    MkArray(Box<Expr>, Box<Expr>),
    GetArray(Box<Expr>, Box<Expr>),
    SetArray(Box<Expr>, Box<Expr>, Box<Expr>),
}
