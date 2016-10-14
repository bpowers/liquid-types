#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;
extern crate unicode_xid;

use std::{env, fmt, error, result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

const EXIT_FAILURE: i32 = 1;

pub mod implicit_parse; // synthesized by LALRPOP

mod tok;
mod common;
mod implicit;
mod explicit;

use tok::Tokenizer;
use explicit::Typ;

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

struct MVEnv<'a> {
    env_id: &'a str,
    next_id: i32,
}

impl<'a> MVEnv<'a> {
    fn new(env: &'a str) -> MVEnv<'a> {
        MVEnv {
            env_id: env,
            next_id: 1,
        }
    }

    fn alloc(&mut self, s: &String) -> Typ {
        let id = self.next_id;
        self.next_id += 1;
        Typ::TMetavar((id, s.clone()))
    }

    fn alloc_empty(&mut self) -> Typ {
        let id = self.next_id;
        self.next_id += 1;
        Typ::TMetavar((id, String::from(self.env_id)))
    }
}

fn add_metavars_in(env: &mut MVEnv, exp: &implicit::Exp) -> explicit::Exp {
    use implicit::Exp as I;
    use explicit::Exp as E;

    match *exp {
        I::Var(ref id) => E::Var(id.clone()),
        I::Const(ref c) => E::Const(*c),
        I::Op2(ref op, ref l, ref r) => {
            let el = box add_metavars_in(env, l);
            let er = box add_metavars_in(env, r);
            E::Op2(*op, el, er)
        }
        I::Fun(ref id, ref e) => {
            let ee = box add_metavars_in(env, e);
            let mv = env.alloc(id);
            E::Fun(id.clone(), mv, ee)
        }
        I::App(ref e1, ref e2) => {
            let ee1 = box add_metavars_in(env, e1);
            let ee2 = box add_metavars_in(env, e2);
            E::App(ee1, ee2)
        }
        I::If(ref e1, ref e2, ref e3) => {
            let ee1 = box add_metavars_in(env, e1);
            let ee2 = box add_metavars_in(env, e2);
            let ee3 = box add_metavars_in(env, e3);
            E::If(ee1, ee2, ee3)
        }
        I::Let(ref id, ref e1, ref e2) => {
            let ee1 = box add_metavars_in(env, e1);
            let ee2 = box add_metavars_in(env, e2);
            E::Let(id.clone(), ee1, ee2)
        }
        I::Fix(ref id, ref e) => {
            let ee = box add_metavars_in(env, e);
            let mv = env.alloc(id);
            E::Fix(id.clone(), mv, ee)
        }
        I::Empty => E::Empty(env.alloc_empty()),
        I::Cons(ref hd, ref tl) => {
            let ehd = box add_metavars_in(env, hd);
            let etl = box add_metavars_in(env, tl);
            E::Cons(ehd, etl)
        }
        I::Head(ref e) => E::Head(box add_metavars_in(env, e)),
        I::Tail(ref e) => E::Tail(box add_metavars_in(env, e)),
        I::IsEmpty(ref e) => E::IsEmpty(box add_metavars_in(env, e)),
        I::Pair(ref l, ref r) => {
            let el = box add_metavars_in(env, l);
            let er = box add_metavars_in(env, r);
            E::Pair(el, er)
        }
        I::ProjL(ref e) => E::ProjL(box add_metavars_in(env, e)),
        I::ProjR(ref e) => E::ProjR(box add_metavars_in(env, e)),
    }
}

fn add_metavars(exp: &implicit::Exp) -> Box<explicit::Exp> {
    let mut env = MVEnv::new("α");
    box add_metavars_in(&mut env, exp)
}

fn gen_constraints(m: &mut MVEnv,
                   env: &mut HashMap<&str, &str>,
                   exp: &explicit::Exp)
                   -> (Vec<(Typ, Typ)>, Typ) {
    use common::{Const, Op2};
    use explicit::Exp as E;

    match *exp {
        E::Const(Const::Int(_)) => (Vec::new(), Typ::TInt),
        E::Const(Const::Bool(_)) => (Vec::new(), Typ::TBool),
        E::Op2(op, ref e1, ref e2) => {
            let opty = match op {
                Op2::LT | Op2::GT | Op2::Eq => Typ::TBool,
                Op2::Add | Op2::Sub | Op2::Mul => Typ::TInt,
            };
            let (c1, t1) = gen_constraints(m, env, e1);
            let (c2, t2) = gen_constraints(m, env, e2);
            (Vec::new(), Typ::TBool)
        }
        _ => (Vec::new(), Typ::TMetavar((1, String::from("a")))),
    }
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

    let typed_w_metavars = add_metavars(&exp);
    let mut cg_env = MVEnv::new("β");
    let mut id_env = HashMap::new();
    let constraints = gen_constraints(&mut cg_env, &mut id_env, &typed_w_metavars);

    // α β

    // let m = make_machine(&asm);

    // let s = p.main().sim();

    // match s.run_to_end() {
    // 	Ok(_) =>
    //     Err()
    // }

    println!("result:\n\n{:?}", exp);
    println!("typed :\n\n{:?}", typed_w_metavars);
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
