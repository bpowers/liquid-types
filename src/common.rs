use std::{error, result};
use std::fmt::{self, Debug, Formatter, Error};

pub type Id = String;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Op2 {
    LT,
    GT,
    LTE,
    GTE,
    Eq,
    Add,
    Sub,
    Mul,
    And,
    Or,
    Impl,
    Iff,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Const {
    Int(i64),
    Bool(bool),
}

impl Debug for Op2 {
    fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), Error> {
        use self::Op2::*;
        match *self {
            LT => write!(fmt, "<"),
            GT => write!(fmt, ">"),
            LTE => write!(fmt, "≤"),
            GTE => write!(fmt, "≥"),
            Eq => write!(fmt, "="),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mul => write!(fmt, "*"),
            And => write!(fmt, "∧"),
            Or => write!(fmt, "∨"),
            Impl => write!(fmt, "⇒"),
            Iff => write!(fmt, "⇔"),
        }
    }
}

// const EXIT_FAILURE: i32 = 1;

// from https://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr
#[macro_export]
macro_rules! eprintln(
    ($($arg:tt)*) => { {
        use std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

#[macro_export]
macro_rules! die(
    ($($arg:tt)*) => { {
        use std;
        eprintln!($($arg)*);
        std::process::exit(1/*EXIT_FAILURE*/)
    } }
);
#[macro_export]
macro_rules! err(
    ($($arg:tt)*) => { {
        use common::LiquidError;
        Err(LiquidError::new(format!($($arg)*)))
    } }
);


#[derive(Debug)]
pub struct LiquidError {
    msg: String,
}

impl LiquidError {
    pub fn new(msg: String) -> LiquidError {
        LiquidError { msg: msg }
    }
}

impl fmt::Display for LiquidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for LiquidError {
    fn description(&self) -> &str {
        &self.msg
    }
}

pub type Result<T> = result::Result<T, LiquidError>;
