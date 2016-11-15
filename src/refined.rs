use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Error};
pub use common::Id;

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Base {
    Bool,
    Int,
//    IntArray,
}

#[derive(PartialEq, Eq, Clone)]
pub enum T<B: Debug> {
    Ref(HashSet<Id>, Base, Box<B>), // set of identifiers in scope
    Fun(Id, Box<T<B>>, Box<T<B>>),
    // TODO: type variable?
}


impl<B: Debug> Debug for T<B> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::T::*;
        match *self {
            Ref(_, base, ref b) => write!(fmt, "{{ν: {:?} | {:?}}}", base, b),
            Fun(ref id, ref tx, ref t) => write!(fmt, "F({}: {:?} → {:?})", id, tx, t),
        }
    }
}
