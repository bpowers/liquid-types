pub use common::{Id, Op2, Const};

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Typ {
    TMetavar(Metavar),
    TInt,
    TBool,
    TFun(Box<Typ>, Box<Typ>),
    TList(Box<Typ>),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Exp {
    Var(Id),
    Const(Const),
    Op2(Op2, Box<Exp>, Box<Exp>),
    Fun(Id, Typ, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
    Let(Id, Box<Exp>, Box<Exp>),
    Fix(Id, Typ, Box<Exp>),
    Empty(Typ),
    Cons(Box<Exp>, Box<Exp>),
    Head(Box<Exp>),
    Tail(Box<Exp>),
    IsEmpty(Box<Exp>),
}

// type abstraction
// type instantiation
