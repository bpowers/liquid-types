
pub type Id = String;

#[derive(Debug)]
pub enum Op2 {
    LT,
    GT,
    Eq,
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
pub enum Const {
    Int(i64),
    Bool(bool),
}
