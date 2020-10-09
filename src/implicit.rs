pub use crate::common::{Const, Id, Op2};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Expr {
    Var(Id),
    Const(Const),
    Op2(Op2, Box<Expr>, Box<Expr>),
    Fun(Id, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Id, Box<Expr>, Box<Expr>),
    Fix(Id, Box<Expr>),
    MkArray(Box<Expr>, Box<Expr>),
    GetArray(Box<Expr>, Box<Expr>),
    SetArray(Box<Expr>, Box<Expr>, Box<Expr>),
    // liquid-type constructs
    Star,
    V,
}
