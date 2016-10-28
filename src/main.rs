#![feature(box_syntax,box_patterns)]

#[macro_use]
extern crate lazy_static;
extern crate unicode_xid;

use std::{env, error};
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
mod type_inference;
mod eval;

use common::{LiquidError, Result};
use type_inference::infer_types;

fn usage() -> ! {
    let argv0 = env::args().nth(0).unwrap_or("<mdl>".to_string());
    die!("Usage: {} [OPTION...] PATH\n\
          Type inference.\n\
          \n\
          Options:\n\
            -help:\tshow this message",
         argv0);
}

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

    let expr = match infer_types(&i_expr) {
        Ok(expr) => expr,
        Err(e) => die!("infer_types: {}", error::Error::description(&e)),
    };

    println!("typed:\n\n{:?}\n", expr);

    let val = eval::interpret(&expr);

    println!("result:\n\n{:?}\n", val);
}
