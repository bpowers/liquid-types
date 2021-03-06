pub use crate::common::{Const, Id, Op2};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr<Ty> {
    Var(Id),
    Const(Const),
    Op2(Op2, Box<Expr<Ty>>, Box<Expr<Ty>>),
    Fun(Id, Ty, Box<Expr<Ty>>),
    App(Box<Expr<Ty>>, Box<Expr<Ty>>),
    If(Box<Expr<Ty>>, Box<Expr<Ty>>, Box<Expr<Ty>>),
    Let(Id, Box<Expr<Ty>>, Box<Expr<Ty>>),
    Fix(Id, Ty, Box<Expr<Ty>>),
    MkArray(Box<Expr<Ty>>, Box<Expr<Ty>>),
    GetArray(Box<Expr<Ty>>, Box<Expr<Ty>>),
    SetArray(Box<Expr<Ty>>, Box<Expr<Ty>>, Box<Expr<Ty>>),
    V,
    Star,
}
