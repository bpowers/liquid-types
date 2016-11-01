#![feature(box_syntax,box_patterns)]
#![feature(non_ascii_idents)]

#[macro_use]
extern crate lazy_static;
extern crate unicode_xid;
extern crate rustproof_libsmt;

use rustproof_libsmt::backends::smtlib2::*;
use rustproof_libsmt::backends::backend::*;
use rustproof_libsmt::backends::z3;

// Include the Core (bool) and Int theory and its functions
use rustproof_libsmt::theories::{core,integer};

// Include the LIA logic
use rustproof_libsmt::logics::lia::LIA;


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
mod refined;
mod hindley_milner;
mod env;
mod liquid;
mod eval;

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

    let expr = match hindley_milner::infer(&i_expr) {
        Ok(expr) => expr,
        Err(e) => die!("infer_types: {}", error::Error::description(&e)),
    };

    println!("typed:\n\n{:?}\n", expr);

    let (αexpr, env) = env::extract(&expr);
    println!("α-implicit: {:?}\n", αexpr);
    println!("Γ         : {:?}\n", env);


    let refined = match liquid::infer(&αexpr, &env) {
        Ok(expr) => expr,
        Err(e) => die!("infer: {}", error::Error::description(&e)),
    };

    println!("refined:\n\n{:?}\n", refined);

    let val = eval::interpret(&expr);

    println!("result:\n\n{:?}\n", val);

        let mut z3: z3::Z3 = Default::default();
    // Defining an instance of Z3 solver
    let mut solver = SMTLib2::new(Some(LIA));
    solver.set_logic(&mut z3);

    // Defining the symbolic vars x & y
    let x = solver.new_var(Some("x"),integer::Sorts::Int);
    let y = solver.new_var(Some("y"),integer::Sorts::Int);

    // Defining the integer constants
    let int5 = solver.new_const(integer::OpCodes::Const(5));
    let int1 = solver.new_const(integer::OpCodes::Const(1));

    // Defining the assert conditions
    let cond1 = solver.assert(integer::OpCodes::Add, &[x, y]);
    let _  = solver.assert(integer::OpCodes::Gt, &[cond1, int5]);
    let _  = solver.assert(integer::OpCodes::Gt, &[x, int1]);
    let _  = solver.assert(integer::OpCodes::Gt, &[y, int1]);

    let (result, response) = solver.solve(&mut z3, false);
    println!("result: {:?}", result);
    println!("response: {:?}", response);
    match result {
        Ok(result) => {
            println!("found answers.");
            println!("x: {}; y: {}", result[&x], result[&y]);
        }
        Err(e) => println!("No solutions for x and y found for given set of constraints ({:?})", e),
    }
}
