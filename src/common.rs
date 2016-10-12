
pub type Id = String;

#[derive(Debug, Copy, Clone)]
pub enum Op2 {
    LT,
    GT,
    Eq,
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Copy, Clone)]
pub enum Const {
    Int(i64),
    Bool(bool),
}
