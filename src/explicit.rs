use common::Op2;
use typed;

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Type {
    TMetavar(Metavar),
    TInt,
    TBool,
    TIntArray,
    TFun(Box<Type>, Box<Type>),
}

pub fn opty(op: Op2) -> Type {
    match op {
        Op2::And | Op2::Or | Op2::Impl | Op2::Iff |
        Op2::LT | Op2::LTE | Op2::GT | Op2::GTE | Op2::Eq => Type::TBool,
        Op2::Add | Op2::Sub | Op2::Mul => Type::TInt,
    }
}


pub type Expr = typed::Expr<Type>;
