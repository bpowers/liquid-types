pub use common::{Id};
use implicit;
use typed;

pub type Metavar = (i32, String);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Base {
    Int,
    Bool,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Q {
    True,
    // logical quantifiers in Q
    And(Box<Q>, Box<Q>)
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum T<B> {
    Ref(Id, Base, Box<B>), /*closure,?*/
    Fun(Id, Box<T<B>>, Box<T<B>>),
    // type variable -- TODO
    Metavar(Metavar),
}



pub type DepType = T<implicit::Expr>;
pub type LiquidType = T<Q>;

pub type DepExpr = typed::Expr<DepType>;
pub type LiquidExpr = typed::Expr<LiquidType>;
