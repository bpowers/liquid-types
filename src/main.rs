extern crate lazy_static;
extern crate unicode_xid;
extern crate z3;

use std::fs::File;
use std::io::Read;
use std::path::Path;

mod implicit_parse {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/implicit_parse.rs"));
}

#[macro_use]
mod common;
mod env;
mod explicit;
mod hindley_milner;
mod implicit;
mod refined;
mod tok;
mod typed;
#[macro_use]
mod liquid;
mod eval;
mod lambdal;

use crate::common::Result;
use crate::liquid::q;

fn usage() -> ! {
    let argv0 = std::env::args()
        .next()
        .unwrap_or_else(|| "<mdl>".to_string());
    die!(
        "Usage: {} [OPTION...] PATH\n\
          Type inference.\n\
          \n\
          Options:\n\
            -help:\tshow this message",
        argv0
    );
}

fn parse_args() -> String {
    for arg in std::env::args().skip(1) {
        if arg == "-help" {
            usage();
        } else if arg.chars().next().unwrap_or(' ') == '-' {
            eprintln!("unknown arg '{}'", arg);
            usage();
        } else {
            return arg;
        }
    }
    // no args? reading from stdin then
    String::from("")
}

pub fn implicit_open<R: Read>(file: &mut R) -> Result<Box<implicit::Expr>> {
    let input = {
        let mut s = String::new();
        if let Err(why) = file.read_to_string(&mut s) {
            return err!("read: {}", why);
        }
        s
    };

    let lexer = tok::Tokenizer::new(&input);
    let parser = crate::implicit_parse::ProgramParser::new();
    match parser.parse(&input, lexer) {
        Ok(v) => Ok(Box::new(v)),
        Err(e) => err!("parse_Program: {:?}", e),
    }
}

fn main() {
    let parse_result = match parse_args().as_ref() {
        "" => implicit_open(&mut std::io::stdin()),
        ref s => {
            let path = Path::new(s);
            let mut file = match File::open(path) {
                // The `description` method of `io::Error` returns a string that
                // describes the error
                Err(why) => die!("open({}): {}", path.display(), why,),
                Ok(file) => file,
            };

            implicit_open(&mut file)
        }
    };

    let i_expr = match parse_result {
        Ok(expr) => expr,
        Err(e) => die!("implicit_open: {}", e),
    };

    let (anf_expr, type_env) = match lambdal::anf(&i_expr) {
        Ok(tuple) => tuple,
        Err(e) => die!("anf: {}", e),
    };

    // define the liquid type templates
    let qs = [
        q("0 <= ν").unwrap(),
        q("★ <= ν").unwrap(),
        q("ν < ★").unwrap(),
        //q("ν < len ★").unwrap(),
    ];

    let refined_env = match liquid::infer(&anf_expr, &type_env, &qs[..]) {
        Ok(env) => env,
        Err(e) => die!("infer: {}", e),
    };

    // println!("refined:\n\n{:?}\n", refined);

    println!("\nrefined Γ:");
    let mut ids: Vec<_> = refined_env.keys().clone().collect();
    ids.sort();
    for id in ids {
        println!("{}:\t{:?}", id, refined_env[id]);
    }
    println!();

    let val = eval::interpret(&anf_expr);

    println!("result:\n\n{:?}\n", val);
}
