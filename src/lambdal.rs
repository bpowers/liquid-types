use std::collections::HashMap;

use common::{Result, Id, Op2, Const};
use explicit;
use implicit;
use env;
use hindley_milner;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Imm {
    Bool(bool),
    Int(i64),
    Var(Id),
    //    Fun(Id, Box<Expr>),
    //    Fix(Id, Box<Expr>),
    //    Star,
    //    V,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Op {
    Op2(Op2, Box<Imm>, Box<Imm>),
    MkArray(Box<Imm>, Box<Imm>),
    GetArray(Box<Imm>, Box<Imm>),
    SetArray(Box<Imm>, Box<Imm>, Box<Imm>),
    Imm(Imm),
}
//    WellFormed(Imm), // Var-only

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    If(Box<Imm>, Box<Expr>, Box<Expr>),
    //    App(Box<Imm>, Box<Imm>),
    Let(Id, Box<Expr>, Box<Expr>),
    Op(Op),
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ConvEnv {
    //env: HashMap<Id, explicit::Type>,
    next_id: i32,
}

impl ConvEnv {
    fn new() -> ConvEnv {
        ConvEnv {
            //env: HashMap::new(),
            next_id: 0,
        }
    }

    fn tmp(&self) -> (ConvEnv, Id) {
        let id = self.next_id;
        let c = ConvEnv{
            //env: self.env.clone(),
            next_id: self.next_id+1,
        };

        (c, format!("!tmp!{}", id))
    }
}

fn constant(c: &Const) -> Imm {
    match *c {
        Const::Bool(b) => Imm::Bool(b),
        Const::Int(i) => Imm::Int(i),
    }
}

fn identity(cenv: ConvEnv, i: Imm) -> (ConvEnv, Expr) {
    (cenv, Expr::Op(Op::Imm(i)))
}

// 1:1 translation -- can't fail
fn expr(cenv: ConvEnv, e: &explicit::Expr, k: &Fn(ConvEnv, Imm) -> (ConvEnv, Expr)) -> (ConvEnv, Expr) {

    use self::Imm as I;
    use typed::Expr as E;
    use self::Op::*;
    use self::Expr::*;

    match *e {
        E::Const(ref c) => k(cenv, constant(c)),
        E::Op2(op, ref l, ref r) => {
            expr(cenv, l, &|cenv: ConvEnv, ll: I| -> (ConvEnv, Expr) {
                expr(cenv, r, &|cenv: ConvEnv, rr: I| -> (ConvEnv, Expr) {

                    let (cenv, op_tmp) = cenv.tmp();
                    let op_val = Op(Op2(op, box ll.clone(), box rr));
                    // value to pass to the continuation
                    let op_ref = I::Var(op_tmp.clone());

                    // our inner expression is whatever the
                    // continuation says it is.
                    let (cenv, result) = k(cenv, op_ref);

                    (cenv, Let(op_tmp, box op_val, box result))
                })
            })
        }
        E::Let(ref id, ref e1, ref e2) => {
            expr(cenv, e1, &|cenv: ConvEnv, e1l: I| -> (ConvEnv, Expr) {
                let (cenv, inner) = expr(cenv, e2, k);
                (cenv, Let(id.clone(), box Op(Imm(e1l.clone())), box inner))
            })
        }
        E::Var(ref x) => {
            k(cenv, I::Var(x.clone()))
        }
        E::If(ref e1, ref e2, ref e3) => {
            expr(cenv, e1, &|cenv: ConvEnv, cond_ref: I| -> (ConvEnv, Expr) {

                let (cenv, l_true) = expr(cenv, e2, &identity);
                let (cenv, l_false) = expr(cenv, e3, &identity);

                // value to pass to the continuation
                let val = If(box cond_ref, box l_true, box l_false);
                let (cenv, val_tmp) = cenv.tmp();

                // our inner expression is whatever the
                // continuation says it is.
                let (cenv, mut result) = k(cenv, I::Var(val_tmp.clone()));

                result = Let(val_tmp, box val, box result);

                (cenv, result)
            })
        }
        E::MkArray(ref sz, ref val) => {
            println!("O mkarray");
            expr(cenv, sz, &|cenv: ConvEnv, isz: I| -> (ConvEnv, Expr) {
                expr(cenv, val, &|cenv: ConvEnv, ival: I| -> (ConvEnv, Expr) {
                    println!("I mkarray");
                    let val = Op(MkArray(box isz.clone(), box ival));
                    let (cenv, val_ref) = cenv.tmp();
                    let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                    (cenv, Let(val_ref, box val, box result))
                })
            })
        }
        E::GetArray(ref id, ref idx) => {
            println!("O getarray");
            expr(cenv, id, &|cenv: ConvEnv, iid: I| -> (ConvEnv, Expr) {
                expr(cenv, idx, &|cenv: ConvEnv, iidx: I| -> (ConvEnv, Expr) {
                    println!("I getarray");
                    let val = Op(GetArray(box iid.clone(), box iidx));
                    let (cenv, val_ref) = cenv.tmp();
                    let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                    (cenv, Let(val_ref, box val, box result))
                })
            })
        }
        E::SetArray(ref id, ref idx, ref v) => {
            expr(cenv, id, &|cenv: ConvEnv, iid: I| -> (ConvEnv, Expr) {
                expr(cenv, idx, &|cenv: ConvEnv, iidx: I| -> (ConvEnv, Expr) {
                    expr(cenv, v, &|cenv: ConvEnv, iv: I| -> (ConvEnv, Expr) {
                        let val = Op(SetArray(box iid.clone(), box iidx.clone(), box iv));
                        let (cenv, val_ref) = cenv.tmp();
                        let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                        (cenv, Let(val_ref, box val, box result))
                    })
                })
            })
        }
        _ => {
            panic!("TODO: implement expr for {:?}", e);
        }
    }
    // E::Fun(ref id, ref t1, ref e) => {
    //     let mut renamed = renamed.clone();
    //     let (n, αid) = (n+1, format!("{}!a{}", id, n));

    //     renamed.insert(id.clone(), αid.clone());
    //     env.insert(αid.clone(), t1.clone());

    //     let (n, e, t2) = convert(n, env, &renamed, e);

    //     (n, I::Fun(αid, box e), Type::TFun(box t1.clone(), box t2))
    // }
    // E::Fix(ref id, ref t1, ref e) => {
    //     let mut renamed = renamed.clone();
    //     let (n, αid) = (n+1, format!("{}!a{}", id, n));

    //     renamed.insert(id.clone(), αid.clone());
    //     env.insert(αid.clone(), t1.clone());

    //     let (n, e, t2) = convert(n, env, &renamed, e);

    //     (n, I::Fix(αid, box e), t2)
    // }
    // E::App(ref e1, ref e2) => {
    //     let (n, e1, t1) = convert(n, env, renamed, e1);
    //     let (n, e2, _) = convert(n, env, renamed, e2);
    //     let t = match t1 {
    //         Type::TFun(_, t2) => *t2,
    //         _ => panic!("expected TFun, not {:?}", t1),
    //     };
    //     (n, I::App(box e1, box e2), t)
    // }
}


pub fn anf(implicit_expr: &implicit::Expr) -> Result<(Expr, HashMap<Id, explicit::Type>)> {

    let explicit_expr = hindley_milner::infer(implicit_expr)?;
    // alpha-renaming
    let (alpha_expr, env) = env::extract(&explicit_expr);

    let cenv = ConvEnv::new();
    let (_, expr) = expr(cenv, &alpha_expr, &identity);

    Ok((expr, env))
}


macro_rules! test_anf(
    ($s:expr, $ae:expr) => { {
        use implicit_parse;
        use tok::Tokenizer;
        use std;
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        let iexpr = match implicit_parse::parse_Program(&s, tokenizer) {
            Ok(iexpr) => iexpr,
            Err(e) => {
                die!("parse_Program({}): {:?}", $s, e);
            }
        };
        let anf_expr = match anf(&iexpr) {
            Ok((anf_expr, _)) => anf_expr,
            Err(e) => {
                die!("anf: {:?}", e);
            }
        };
        if anf_expr != $ae {
            die!("mismatch {:?} != {:?}", anf_expr, $ae);
        }
    } }
);

#[test]
fn anf_transforms() {
    use common::Op2 as O;
    use self::Imm as I;
    use self::Op::*;
    use self::Expr::*;

    test_anf!(
        "-22",
        Op(Imm(I::Int(-22))));

    test_anf!(
        "2+3",
        Let(String::from("!tmp!0"), box Op(Op2(O::Add, box I::Int(2), box I::Int(3))),
            box Op(Imm(I::Var(String::from("!tmp!0"))))));

    test_anf!(
        "2+(3 - 2)",
        Let(String::from("!tmp!0"), box Op(Op2(O::Sub, box I::Int(3), box I::Int(2))),
            box Let(String::from("!tmp!1"), box Op(Op2(O::Add, box I::Int(2), box I::Var(String::from("!tmp!0")))),
                    box Op(Imm(I::Var(String::from("!tmp!1")))))));

    test_anf!(
        "2 + 3 - 2",
        Let(String::from("!tmp!0"), box Op(Op2(O::Add, box I::Int(2), box I::Int(3))),
            box Let(String::from("!tmp!1"), box Op(Op2(O::Sub, box I::Var(String::from("!tmp!0")), box I::Int(2))),
                    box Op(Imm(I::Var(String::from("!tmp!1")))))));

    test_anf!(
        "let x = 1 in x",
        Let(String::from("x!a1"), box Op(Imm(I::Int(1))),
            box Op(Imm(I::Var(String::from("x!a1"))))));

    test_anf!(
        "let x = (if true then 1 else 2) in x",
        Let(String::from("!tmp!0"), box If(box I::Bool(true), box Op(Imm(I::Int(1))), box Op(Imm(I::Int(2)))),
            box Let(String::from("x!a1"), box Op(Imm(I::Var(String::from("!tmp!0")))),
                    box Op(Imm(I::Var(String::from("x!a1")))))));

    test_anf!(
        "let a = array(3, 5) in a[0]",
        Let(String::from("!tmp!0"), box Op(MkArray(box I::Int(3), box I::Int(5))),
            box Let(String::from("a!a1"), box Op(Imm(I::Var(String::from("!tmp!0")))),
                    box Let(String::from("!tmp!1"), box Op(GetArray(box I::Var(String::from("a!a1")), box I::Int(0))),
                            box Op(Imm(I::Var(String::from("!tmp!1"))))))));
}
