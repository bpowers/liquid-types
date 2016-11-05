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
    //    MkArray(Box<Imm>, Box<Imm>),
    //    GetArray(Box<Imm>, Box<Imm>),
    //    SetArray(Box<Imm>, Box<Imm>, Box<Imm>),
    Imm(Imm),
}
//    WellFormed(Imm), // Var-only

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    //    If(Box<Imm>, Box<Imm>, Box<Imm>),
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

fn op(cenv: ConvEnv, e: &explicit::Expr) -> (ConvEnv, Op) {
    use typed::Expr as E;
    match *e {
        E::Const(Const::Bool(c)) => (cenv, Op::Imm(Imm::Bool(c))),
        E::Const(Const::Int(c)) => (cenv, Op::Imm(Imm::Int(c))),
        _ => {
            panic!("TODO: implement term for {:?}", e);
        }
    }
}

fn identity(cenv: ConvEnv, e: Expr) -> (ConvEnv, Expr) {
    (cenv, e)
}

fn imm(e: &Expr) -> Option<Imm> {
    if let &Expr::Op(Op::Imm(ref i)) = e {
        Some(i.clone())
    } else {
        None
    }
}

// 1:1 translation -- can't fail
fn expr(cenv: ConvEnv, e: &explicit::Expr, k: &Fn(ConvEnv, Expr) -> (ConvEnv, Expr)) -> (ConvEnv, Expr) {

    use self::Imm as I;
    use typed::Expr as E;
    use self::Op::*;
    use self::Expr::*;

    match *e {
        E::Const(_) => {
            let (cenv, eop) = op(cenv, e);
            k(cenv, Op(eop))
        }
        E::Op2(op, ref l, ref r) => {
            expr(cenv, l, &|cenv: ConvEnv, ll: Expr| -> (ConvEnv, Expr) {
                expr(cenv, r, &|cenv: ConvEnv, rr: Expr| -> (ConvEnv, Expr) {
                    // allocate vars
                    let mut cenv = cenv;

                    let (l_ref, l_bound) = match imm(&ll) {
                        Some(i) => (i, None),
                        None    => {
                            let (l_cenv, l_tmp) = cenv.tmp();
                            cenv = l_cenv;
                            (I::Var(l_tmp.clone()), Some(l_tmp))
                        }
                    };

                    let (r_ref, r_bound) = match imm(&rr) {
                        Some(i) => (i, None),
                        None    => {
                            let (r_cenv, r_tmp) = cenv.tmp();
                            cenv = r_cenv;
                            (I::Var(r_tmp.clone()), Some(r_tmp))
                        }
                    };

                    // value to pass to the continuation
                    let val = Op(Op2(op, box l_ref, box r_ref));

                    // our inner expression is whatever the
                    // continuation says it is.
                    let (cenv, mut result) = k(cenv, val);

                    if let Some(r_tmp) = r_bound {
                        result = Let(r_tmp, box rr, box result);
                    }
                    if let Some(l_tmp) = l_bound {
                        result = Let(l_tmp, box ll.clone(), box result);
                    }

                    (cenv, result)
                })
            })
        }
        _ => {
            panic!("TODO: implement expr for {:?}", e);
        }
    }
    // E::Op2(op, ref e1, ref e2) => {
    //     let (n, e1, _) = convert(n, env, renamed, e1);
    //     let (n, e2, _) = convert(n, env, renamed, e2);
    //     (n, I::Op2(op, box e1, box e2), explicit::opty(op))
    // }
    // E::If(ref e1, ref e2, ref e3) => {
    //     let (n, e1, _) = convert(n, env, renamed, e1);
    //     let (n, e2, t) = convert(n, env, renamed, e2);
    //     let (n, e3, _) = convert(n, env, renamed, e3);
    //     (n, I::If(box e1, box e2, box e3), t)
    // }
    // E::Var(ref x) => {
    //     let αx = renamed.get(x).unwrap();
    //     (n, I::Var(αx.clone()), env.get(αx).unwrap().clone())
    // }
    // E::Let(ref id, ref e1, ref e2) => {
    //     let mut renamed = renamed.clone();
    //     let (n, αid) = (n+1, format!("{}!a{}", id, n));

    //     let (n, e1, t1) = convert(n, env, &renamed, e1);

    //     renamed.insert(id.clone(), αid.clone());
    //     env.insert(αid.clone(), t1);

    //     let (n, e2, t2) = convert(n, env, &renamed, e2);
    //     (n, I::Let(αid, box e1, box e2), t2)
    // }
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
    // E::MkArray(ref sz, ref val) => {
    //     let (n, sz, _) = convert(n, env, renamed, sz);
    //     let (n, val, _) = convert(n, env, renamed, val);
    //     (n, I::MkArray(box sz, box val), Type::TIntArray)
    // }
    // E::GetArray(ref id, ref idx) => {
    //     let (n, id, _) = convert(n, env, renamed, id);
    //     let (n, idx, _) = convert(n, env, renamed, idx);
    //     (n, I::GetArray(box id, box idx), Type::TInt)
    // }
    // E::SetArray(ref id, ref idx, ref v) => {
    //     let (n, id, _) = convert(n, env, renamed, id);
    //     let (n, idx, _) = convert(n, env, renamed, idx);
    //     let (n, v, _) = convert(n, env, renamed, v);
    //     (n, I::SetArray(box id, box idx, box v), Type::TIntArray)
    // }
}


pub fn anf(implicit_expr: &implicit::Expr) -> Result<(Expr, HashMap<Id, explicit::Type>)> {

    let explicit_expr = hindley_milner::infer(implicit_expr)?;
    // alpha-renaming
    let (alpha_expr, env) = env::extract(&explicit_expr);

    // println!("α-implicit: {:?}\n", α_expr);
    // println!("Γ         : {:?}\n", env);


    let cenv = ConvEnv::new();
    let (_, expr) = expr(cenv, &alpha_expr, &identity);

    Ok((expr, env))
    // step 1 -- arithmatic + let bindings
    // step 2 -- conds + boolean vals
    // step 3 -- arrays
    // step 4 -- closures

    // (define (Value? M)
    //   (match M
    //     [`(quote ,_)         #t]
    //     [(? number?)         #t]
    //     [(? boolean?)        #t]
    //     [(? string?)         #t]
    //     [(? char?)           #t]
    //     [(? symbol?)         #t]
    //     [(or '+ '- '* '/ '=) #t]
    //     [else                #f]))


    // (define (normalize-term M) (normalize M (λ (x) x)))

    // (define (normalize M k)
    //   (match M
    //     [`(λ ,params ,body)
    //       (k `(λ ,params ,(normalize-term body)))]

    //     [`(let ([,x ,M1]) ,M2)
    //       (normalize M1 (λ (N1)
    //        `(let ([,x ,N1])
    //          ,(normalize M2 k))))]

    //     [`(if ,M1 ,M2 ,M3)
    //       (normalize-name M1 (λ (t)
    //        (k `(if ,t ,(normalize-term M2)
    //                   ,(normalize-term M3)))))]

    //     [`(,Fn . ,M*)
    //       (normalize-name Fn (λ (t)
    //        (normalize-name* M* (λ (t*)
    //         (k `(,t . ,t*))))))]

    //     [(? Value?)             (k M)]))

    // (define (normalize-name M k)
    //   (normalize M (λ (N)
    //     (if (Value? N) (k N)
    //         (let ([t (gensym)])
    //          `(let ([,t ,N]) ,(k t)))))))

    // (define (normalize-name* M* k)
    //   (if (null? M*)
    //       (k '())
    //       (normalize-name (car M*) (λ (t)
    //        (normalize-name* (cdr M*) (λ (t*)
    //         (k `(,t . ,t*))))))))

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
        Op(Op2(O::Add, box I::Int(2), box I::Int(3))));

    test_anf!(
        "2+(3 - 2)",
        Let(String::from("!tmp!0"), box Op(Op2(O::Sub, box I::Int(3), box I::Int(2))),
            box Op(Op2(O::Add, box I::Int(2), box I::Var(String::from("!tmp!0"))))));
}
