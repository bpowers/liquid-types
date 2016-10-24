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
mod tok;
mod implicit;
mod explicit;
mod type_inference;

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
    // no args? die
    usage();
}

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

    let tokenizer = tok::Tokenizer::new(&s);
    match implicit_parse::parse_Program(&s, tokenizer) {
        Ok(v) => Ok(v),
        Err(e) => err!("parse_Program: {:?}", e),
    }
}

fn main() {
    let path_s = parse_args();
    let path = Path::new(&path_s);

    let i_expr = match implicit_open(&path) {
        Ok(expr) => expr,
        Err(e) => die!("implicit_open: {}", error::Error::description(&e)),
    };

    let expr = match infer_types(&i_expr) {
        Ok(expr) => expr,
        Err(e) => die!("infer_types: {}", error::Error::description(&e))
    };

    println!("typed:\n\n{:?}\n", expr);
}
