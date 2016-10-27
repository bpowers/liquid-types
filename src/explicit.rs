pub use common::{Id, Op2, Const};

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Type {
    TMetavar(Metavar),
    TInt,
    TBool,
    TFun(Box<Type>, Box<Type>),
    TList(Box<Type>),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Expr {
    Var(Id),
    Const(Const),
    Op2(Op2, Box<Expr>, Box<Expr>),
    Fun(Id, Type, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Id, Box<Expr>, Box<Expr>),
    Fix(Id, Type, Box<Expr>),
    Empty(Type),
    Cons(Box<Expr>, Box<Expr>),
    Head(Box<Expr>),
    Tail(Box<Expr>),
    IsEmpty(Box<Expr>),
}

// type abstraction
// type instantiation
