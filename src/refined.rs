use std::collections::HashSet;
pub use common::Id;

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Base {
    Bool,
    Int,
//    IntArray,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum T<B> {
    Ref(HashSet<Id>, Base, Box<B>), // set of identifiers in scope
    Fun(Id, Box<T<B>>, Box<T<B>>),
    // TODO: type variable?
}
