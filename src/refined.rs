pub use common::{Id};

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Base {
    Bool,
    Int,
    IntArray,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum T<Env, B> {
    Ref(Box<Env>, Base, Box<B>), /*closure,?*/
    Fun(Id, Box<T<Env, B>>, Box<T<Env, B>>),
    // type variable -- TODO
}

//pub type Type = T<(), implicit::Expr>;
//pub type Expr = typed::Expr<Type>;
