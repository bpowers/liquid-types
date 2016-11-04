#![feature(box_syntax,box_patterns)]
#![feature(non_ascii_idents)]

#[macro_use]
extern crate lazy_static;
extern crate unicode_xid;
extern crate rustproof_libsmt;

use std::error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod implicit_parse; // synthesized by LALRPOP

#[macro_use]
mod common;
mod typed;
mod tok;
mod implicit;
mod explicit;
//mod refined;
mod hindley_milner;
mod env;
//mod liquid;
//mod eval;
mod lambdal;

use common::{LiquidError, Result};

fn usage() -> ! {
    let argv0 = std::env::args().nth(0).unwrap_or("<mdl>".to_string());
    die!("Usage: {} [OPTION...] PATH\n\
          Type inference.\n\
          \n\
          Options:\n\
            -help:\tshow this message",
         argv0);
}

fn parse_args() -> String {

    for arg in std::env::args().skip(1) {
        if arg == "-help" {
            usage();
        } else if arg.chars().nth(0).unwrap_or(' ') == '-' {
            eprintln!("unknown arg '{}'", arg);
            usage();
        } else {
            return arg;
        }
    }
    // no args? reading from stdin then
    String::from("")
}

pub fn implicit_open<'a, R: Read>(file: &mut R) -> Result<Box<implicit::Expr>> {

    let s = {
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => {
                return err!("read: {}", error::Error::description(&why))
            }
            Ok(_) => {}
        }
        s
    };

    let tokenizer = tok::Tokenizer::new(&s);
    match implicit_parse::parse_Program(&s, tokenizer) {
        Ok(v) => Ok(v),
        Err(e) => err!("parse_Program: {:?}", e),
    }
}

fn main() {
    let parse_result = match parse_args().as_ref() {
        "" => {
            implicit_open(&mut std::io::stdin())
        }
        ref s => {
            let path = Path::new(s);
            let mut file = match File::open(path) {
                // The `description` method of `io::Error` returns a string that
                // describes the error
                Err(why) => {
                    die!("open({}): {}",
                         path.display(),
                         error::Error::description(&why))
                }
                Ok(file) => file,
            };

            implicit_open(&mut file)
        }
    };

    let i_expr = match parse_result {
        Ok(expr) => expr,
        Err(e) => die!("implicit_open: {}", error::Error::description(&e)),
    };

    let e_expr = match hindley_milner::infer(&i_expr) {
        Ok(expr) => expr,
        Err(e) => die!("infer_types: {}", error::Error::description(&e)),
    };

    println!("typed:\n\n{:?}\n", e_expr);

    let (α_expr, env) = env::extract(&e_expr);
    println!("α-implicit: {:?}\n", α_expr);
    println!("Γ         : {:?}\n", env);

    // alternatively:
    // Qbc (bounds checking)
    // X: 0, *, len *
    // ν < X
    // ν <= X
    // ν = X
    // ν >= X
    // ν > X

    // let q = {
    //     use implicit::Expr::*;
    //     use common::Op2::*;
    //     use common::Const::*;
    //     [
    //         Op2(LTE, box Const(Int(0)), box V),
    //         Op2(LTE, box Star, box V),
    //         Op2(LT, box V, box Star),
    //         //Op2(LT, box V, box App(box Var(String::from("len")), box Star)),
    //     ]
    // };

    // let refined = match liquid::infer(&α_expr, &env, &q) {
    //     Ok(expr) => expr,
    //     Err(e) => die!("infer: {}", error::Error::description(&e)),
    // };

    // println!("refined:\n\n{:?}\n", refined);

    //let val = eval::interpret(&e_expr);

    //println!("result:\n\n{:?}\n", val);
}
