pub use common::{Id, Op2, Const};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Expr<Ty> {
    Var(Id),
    Const(Const),
    Op2(Op2, Box<Expr<Ty>>, Box<Expr<Ty>>),
    Fun(Id, Ty, Box<Expr<Ty>>),
    App(Box<Expr<Ty>>, Box<Expr<Ty>>),
    If(Box<Expr<Ty>>, Box<Expr<Ty>>, Box<Expr<Ty>>),
    Let(Id, Box<Expr<Ty>>, Box<Expr<Ty>>),
    Fix(Id, Ty, Box<Expr<Ty>>),
    Empty(Ty),
    Cons(Box<Expr<Ty>>, Box<Expr<Ty>>),
    Head(Box<Expr<Ty>>),
    Tail(Box<Expr<Ty>>),
    IsEmpty(Box<Expr<Ty>>),
    MkArray(Box<Expr<Ty>>, Box<Expr<Ty>>),
    GetArray(Box<Expr<Ty>>, Box<Expr<Ty>>),
    SetArray(Box<Expr<Ty>>, Box<Expr<Ty>>, Box<Expr<Ty>>),

    // what is needed to represent liquid templates?
    Star,
    V,
}
