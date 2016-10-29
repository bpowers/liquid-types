pub use common::{Id};
use implicit;
use typed;

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Base {
    Bool,
    Int,
    IntArray,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum T<B> {
    Ref(Id, Base, Box<B>), /*closure,?*/
    Fun(Id, Box<T<B>>, Box<T<B>>),
    // type variable -- TODO
    Metavar(Metavar),
}



pub type Type = T<implicit::Expr>;
pub type Expr = typed::Expr<Type>;
