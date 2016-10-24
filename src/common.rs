use std::{fmt, error, result};

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

//const EXIT_FAILURE: i32 = 1;

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
	    eprintln!($($arg)*);
	    std::process::exit(1/*EXIT_FAILURE*/)
    } }
);
#[macro_export]
macro_rules! err(
    ($($arg:tt)*) => { {
	Err(LiquidError::new(format!($($arg)*)))
    } }
);


#[derive(Debug)]
pub struct LiquidError {
    msg: String,
}

impl LiquidError {
    pub fn new(msg: String) -> LiquidError {
        LiquidError {msg: msg}
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
