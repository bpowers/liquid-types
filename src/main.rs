#[macro_use]
extern crate lazy_static;
extern crate unicode_xid;

use std::{env, fmt, error, result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const EXIT_FAILURE: i32 = 1;

pub mod implicit_parse; // synthesized by LALRPOP

mod tok;
mod common;
mod implicit;
mod explicit;

use tok::Tokenizer;

// from https://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr
macro_rules! eprintln(
    ($($arg:tt)*) => { {
	    use std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

macro_rules! die(
    ($($arg:tt)*) => { {
	    eprintln!($($arg)*);
	    std::process::exit(EXIT_FAILURE)
    } }
);

fn usage() -> ! {
    let argv0 = env::args().nth(0).unwrap_or("<mdl>".to_string());
    die!("Usage: {} [OPTION...] PATH\n\
          Type inference.\n\
          \n\
          Options:\n\
            -help:\tshow this message",
         argv0);
}

macro_rules! err(
    ($($arg:tt)*) => { {
	Err(LiquidError{msg: format!($($arg)*)})
    } }
);


fn parse_args() -> String {

    for arg in env::args().skip(1) {
        if arg == "-help" {
            usage();
        } else if arg.chars().nth(0).unwrap_or(' ') == '-' {
            eprintln!("unknown arg '{}'", arg);
            usage();
        } else {
            return arg;
        }
    }
    // no args? die
    usage();
}

#[derive(Debug)]
pub struct LiquidError {
    msg: String,
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

pub fn implicit_open<'a>(path: &Path) -> Result<Box<implicit::Exp>> {

    let mut file = match File::open(path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => {
            return err!("open({}): {}",
                        path.display(),
                        error::Error::description(&why))
        }
        Ok(file) => file,
    };

    let s = {
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => {
                return err!("read({}): {}",
                            path.display(),
                            error::Error::description(&why))
            }
            Ok(_) => {}
        }
        s
    };

    let tokenizer = Tokenizer::new(&s);
    match implicit_parse::parse_Program(&s, tokenizer) {
        Ok(v) => Ok(v),
        Err(e) => err!("parse_Program: {:?}", e),
    }
}

fn add_metavars(exp: &implicit::Exp) -> Box<explicit::Exp> {
    Box::new(explicit::Exp::Empty)
}

fn main() {

    let path_s = parse_args();
    let path = Path::new(&path_s);

    // let fd = open(&path);

    // let state = make_state(fd);

    let exp = match implicit_open(&path) {
        Ok(v) => v,
        Err(e) => die!("implicit_open: {}", error::Error::description(&e)),
    };

    let typedWithMetavars = add_metavars(&exp);

    // let m = make_machine(&asm);

    // let s = p.main().sim();

    // match s.run_to_end() {
    // 	Ok(_) =>
    //     Err()
    // }

    println!("result:\n\n{:?}", exp);
}


macro_rules! test_parse(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        if let Err(e) = implicit_parse::parse_Program(&s, tokenizer) {
            die!("parse_Program({}): {:?}", $s, e)
        }
    } }
);

macro_rules! test_parse_fails(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        if let Ok(_) = implicit_parse::parse_Program(&s, tokenizer) {
            die!("parse_Program({}) should have failed", $s)
        }
    } }
);

#[test]
fn implicit_parse() {
    test_parse!("-22");
    test_parse!("(22)");
    test_parse!("((((-22))))");
    test_parse!("((((-22))))");

    test_parse!("1 :: 2 :: empty");
    test_parse!("true :: false :: empty");
    test_parse!("true :: true :: empty");
    test_parse!("(2 + 3 - 2) + 3");
    test_parse!("fun x -> x + 1");
    test_parse!("let inc = fun x -> x + 1 in inc 3");
    test_parse!("let factorial = fix fac -> fun n -> if n = 0 then 1 else n * (fac (n - 1)) in factorial 5");
    test_parse!("let b = true in if b then true else false");
    test_parse!("let double = fun x -> x * 2 in double 3");
    test_parse!("let h = (fun l -> head l) in let l = 1 :: 2 :: empty in (h l) :: l");

    test_parse_fails!("((22)");
    test_parse_fails!("let x = 1"); // only let-in supported
}
