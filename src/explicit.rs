use typed;

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Type {
    TMetavar(Metavar),
    TInt,
    TBool,
    TIntArray,
    TFun(Box<Type>, Box<Type>),
    TList(Box<Type>),
}

pub type Expr = typed::Expr<Type>;
