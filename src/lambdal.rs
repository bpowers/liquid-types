use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt::{Debug, Formatter, Error};

use common;
use common::{Id, Op2, Const};
use explicit;
use explicit::{Type};
use implicit;
use env;
use hindley_milner;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Imm {
    Bool(bool),
    Int(i64),
    Var(Id),
    Fun(Id, Box<Expr>),
    Fix(Id, Box<Expr>),
    Star,
    V,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Op {
    // should only be used with Imms, but Ops to make liquid type
    // constraints expressable without creating new temporaries
    Op2(Op2, Box<Op>, Box<Op>),
    MkArray(Box<Imm>, Box<Imm>),
    GetArray(Box<Imm>, Box<Imm>),
    SetArray(Box<Imm>, Box<Imm>, Box<Imm>),
    Imm(Imm),
}
//    WellFormed(Imm), // Var-only

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Expr {
    If(Box<Imm>, Box<Expr>, Box<Expr>),
    App(Box<Imm>, Box<Imm>),
    Let(Id, Box<Expr>, Box<Expr>),
    Op(Op),
}

impl Debug for Imm {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Imm::*;
        match *self {
            Bool(b) => write!(fmt, "{}", b),
            Int(n) => write!(fmt, "{}", n),
            Var(ref id) => write!(fmt, "{}", id),
            Fun(ref id, ref e) => write!(fmt, "(fun {} -> {:?})", id, e),
            Fix(ref id, ref e) => write!(fmt, "(fix {} -> {:?})", id, e),
            Star => write!(fmt, "★"),
            V => write!(fmt, "ν"),
        }
    }
}

impl Debug for Op {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Op::*;
        match *self {
            Op2(op, ref l, ref r) => write!(fmt, "{:?} {:?} {:?}", l, op, r),
            MkArray(ref sz, ref n) => write!(fmt, "array({:?}, {:?})", sz, n),
            GetArray(ref a, ref idx) => write!(fmt, "{:?}[{:?}]", a, idx),
            SetArray(ref a, ref idx, ref n) => write!(fmt, "{:?}[{:?}] = {:?}", a, idx, n),
            Imm(ref imm) => write!(fmt, "{:?}", imm),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            If(ref cond, ref l, ref r) => write!(fmt, "if ({:?}) then ({:?}) else ({:?})", cond, l, r),
            App(ref e1, ref e2) => write!(fmt, "{:?} {:?}", e1, e2),
            Let(ref id, ref e1, ref e2) => write!(fmt, "let {} = ({:?}) in ({:?})", id, e1, e2),
            Op(ref op) => write!(fmt, "{:?}", op),
        }
    }
}

static ENV_ID: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ConvEnv {
    env_id: usize,
    next_id: i32,
}

impl ConvEnv {
    fn new() -> ConvEnv {
        ConvEnv {
            env_id: ENV_ID.fetch_add(1, Ordering::SeqCst),
            next_id: 0,
        }
    }

    fn tmp(&self) -> (ConvEnv, Id) {
        let id = self.next_id;
        let c = ConvEnv{
            env_id: self.env_id,
            next_id: self.next_id+1,
        };

        (c, format!("!tmp-{}!{}", self.env_id, id))
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
            expr(cenv, l, &|cenv, ll| {
                expr(cenv, r, &|cenv, rr| {
                    let (cenv, op_tmp) = cenv.tmp();
                    let op_val = Op(Op2(op, box Imm(ll.clone()), box Imm(rr)));
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
            expr(cenv, e1, &|cenv, e1l| {
                let (cenv, inner) = expr(cenv, e2, k);
                (cenv, Let(id.clone(), box Op(Imm(e1l.clone())), box inner))
            })
        }
        E::Var(ref x) => {
            k(cenv, I::Var(x.clone()))
        }
        E::If(ref e1, ref e2, ref e3) => {
            expr(cenv, e1, &|cenv, cond_ref| {

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
        E::Fun(ref id, _, ref e) => {
            let (cenv, fun) = expr(cenv, e, &identity);
            let (cenv, fun_ref) = cenv.tmp();
            let (cenv, result) = k(cenv, I::Var(fun_ref.clone()));

            (cenv, Let(fun_ref, box Op(Imm(I::Fun(id.clone(), box fun))), box result))
        }
        E::Fix(ref id, _, ref e) => {
            let (cenv, fix) = expr(cenv, e, &identity);
            let (cenv, fix_ref) = cenv.tmp();
            let (cenv, result) = k(cenv, I::Var(fix_ref.clone()));

            (cenv, Let(fix_ref, box Op(Imm(I::Fix(id.clone(), box fix))), box result))
        }
        E::App(ref e1, ref e2) => {
            expr(cenv, e1, &|cenv, ie1| {
                expr(cenv, e2, &|cenv, ie2| {

                    let (cenv, app_tmp) = cenv.tmp();
                    let app_val = App(box ie1.clone(), box ie2.clone());
                    // value to pass to the continuation
                    let app_ref = I::Var(app_tmp.clone());

                    // our inner expression is whatever the
                    // continuation says it is.
                    let (cenv, result) = k(cenv, app_ref);

                    (cenv, Let(app_tmp, box app_val, box result))
                })
            })
        }
        E::MkArray(ref sz, ref val) => {
            expr(cenv, sz, &|cenv, isz| {
                expr(cenv, val, &|cenv, ival| {
                    let val = Op(MkArray(box isz.clone(), box ival));
                    let (cenv, val_ref) = cenv.tmp();
                    let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                    (cenv, Let(val_ref, box val, box result))
                })
            })
        }
        E::GetArray(ref id, ref idx) => {
            expr(cenv, id, &|cenv, iid| {
                expr(cenv, idx, &|cenv, iidx| {
                    let val = Op(GetArray(box iid.clone(), box iidx));
                    let (cenv, val_ref) = cenv.tmp();
                    let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                    (cenv, Let(val_ref, box val, box result))
                })
            })
        }
        E::SetArray(ref id, ref idx, ref v) => {
            expr(cenv, id, &|cenv, iid| {
                expr(cenv, idx, &|cenv, iidx| {
                    expr(cenv, v, &|cenv, iv| {
                        let val = Op(SetArray(box iid.clone(), box iidx.clone(), box iv));
                        let (cenv, val_ref) = cenv.tmp();
                        let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                        (cenv, Let(val_ref, box val, box result))
                    })
                })
            })
        }
        E::V => k(cenv, I::V),
        E::Star => k(cenv, I::Star),
    }
}

fn build_env_imm(env: HashMap<Id, Type>, i: &Imm) -> (HashMap<Id, Type>, Type) {
    use self::Imm::*;
    match *i {
        Bool(_) => (env, Type::TBool),
        Int(_) => (env, Type::TInt),
        Var(ref id) => {
            let ty = match env.get(id) {
                Some(ty) => ty.clone(),
                None => panic!("no key '{}' in {:?}", id, env),
            };
            (env, ty)
        }
        Fun(ref id, ref e) => {
            let (env, return_type) = build_env_expr(env, e);
            let arg_type = env[id].clone();
            (env, Type::TFun(id.clone(), box arg_type, box return_type))
        }
        Fix(_, ref e) => build_env_expr(env, e),
        V | Star => unreachable!("ν or ★ encountered during build_env"),
    }
}

fn build_env_op(env: HashMap<Id, Type>, o: &Op) -> (HashMap<Id, Type>, Type) {
    use self::Op::*;
    match *o {
        Op2(op, ref e1, ref e2) => {
            let (env, _) = build_env_op(env, e1);
            let (env, _) = build_env_op(env, e2);
            (env, explicit::opty(op))
        }
        MkArray(ref e1, ref e2) => {
            let (env, _) = build_env_imm(env, e1);
            let (env, _) = build_env_imm(env, e2);
            (env, Type::TIntArray)
        }
        GetArray(ref e1, ref e2) => {
            let (env, _) = build_env_imm(env, e1);
            let (env, _) = build_env_imm(env, e2);
            (env, Type::TInt)
        }
        SetArray(ref e1, ref e2, ref e3) => {
            let (env, _) = build_env_imm(env, e1);
            let (env, _) = build_env_imm(env, e2);
            let (env, _) = build_env_imm(env, e3);
            (env, Type::TIntArray)
        }
        Imm(ref i) => build_env_imm(env, i),
    }
}

fn build_env_expr(env: HashMap<Id, Type>, e: &Expr) -> (HashMap<Id, Type>, Type) {
    use self::Expr::*;
    match *e {
        If(ref cond, ref e1, ref e2) => {
            let (env, _) = build_env_imm(env, cond);
            let (env, ty) = build_env_expr(env, e1);
            let (env, _) = build_env_expr(env, e2);
            (env, ty)
        }
        App(ref e1, _) => {
            let (env, e1_type) = build_env_imm(env, e1);
            if let Type::TFun(_,
                              _, ret_type) = e1_type {
                (env, *ret_type)
            } else {
                unreachable!("app of non-fun should have been caught by HM")
            }
        }
        Let(ref id, ref e1, ref e2) => {
            let (env, id_ty) = build_env_expr(env, e1);
            let mut env = env.clone();
            env.insert(id.clone(), id_ty);

            build_env_expr(env, e2)
        }
        Op(ref op) => build_env_op(env, op),
    }
}

pub fn anf(implicit_expr: &implicit::Expr) -> common::Result<(Expr, HashMap<Id, Type>)> {

    let explicit_expr = hindley_milner::infer(implicit_expr)?;
    // alpha-renaming
    let (alpha_expr, env) = env::extract(&explicit_expr);

    let cenv = ConvEnv::new();
    let (_, expr) = expr(cenv, &alpha_expr, &identity);

    let (env, _) = build_env_expr(env, &expr);

    Ok((expr, env))
}

pub fn q_op(e: &implicit::Expr) -> common::Result<Op> {
    use implicit::Expr as I;
    use self::Imm::*;
    use common::Const;

    let imm = match *e {
        I::Op2(op, ref l, ref r) => {
            let l = q_op(l)?;
            let r = q_op(r)?;
            return Ok(Op::Op2(op, box l, box r));
        }
        I::Var(ref id) => Var(id.clone()),
        I::Const(Const::Int(b)) => Int(b),
        I::Const(Const::Bool(b)) => Bool(b),
        I::Star => Star,
        I::V => V,
        _ => {
            panic!("unexpected imm in q: {:?}", e);
        }
    };
    Ok(Op::Imm(imm))
}

#[allow(dead_code)]
pub fn q(implicit_expr: &implicit::Expr) -> common::Result<Expr> {
    use common::LiquidError;
    if let Ok(op) = q_op(implicit_expr) {
        return Ok(Expr::Op(op));
    }

    err!("expected simple Q")
}

#[test]
fn test_q() {
    use common::Op2::*;

    let q1 = implicit::Expr::Op2(LT, box implicit::Expr::V, box implicit::Expr::Star);
    let ql = match q(&q1) {
        Ok(expr) => expr,
        Err(e) => die!("q: {:?}", e),
    };

    let expected = Expr::Op(Op::Op2(LT, box Op::Imm(Imm::V), box Op::Imm(Imm::Star)));

    if !cmp(&ql, &expected) {
        die!("conversion of q failed: {:?}", ql)
    };
}

fn cmp_imm(vmap: HashMap<Id, Id>, imm1: &Imm, imm2: &Imm) -> bool {
    let mut vmap = vmap;
    use self::Imm::*;

    match (imm1, imm2) {
        (&Bool(b1), &Bool(b2)) => b1 == b2,
        (&Int(n1), &Int(n2)) => n1 == n2,
        (&Var(ref v1), &Var(ref v2)) => {
            let expected_v2 = match vmap.get(v1) {
                Some(v) => v,
                None => {
                    return false;
                }
            };
            expected_v2 == v2
        }
        (&Fun(ref id1, ref e1), &Fun(ref id2, ref e2)) => {
            vmap.insert(id1.clone(), id2.clone());
            cmp_expr(vmap, e1, e2)
        }
        (&Fix(ref id1, ref e1), &Fun(ref id2, ref e2)) => {
            vmap.insert(id1.clone(), id2.clone());
            cmp_expr(vmap, e1, e2)
        }
        (&Star, &Star) => true,
        (&V, &V) => true,
        _ => false,
    }
}

fn cmp_op(vmap: HashMap<Id, Id>, e1: &Op, e2: &Op) -> bool {
    use self::Op::*;

    match (e1, e2) {
        (&Op2(ref op1, ref l1, ref r1), &Op2(ref op2, ref l2, ref r2)) => {
            op1 == op2 && cmp_op(vmap.clone(), l1, l2) && cmp_op(vmap, r1, r2)
        }
        (&MkArray(ref sz1, ref n1), &MkArray(ref sz2, ref n2)) => {
            cmp_imm(vmap.clone(), sz1, sz2) && cmp_imm(vmap, n1, n2)
        }
        (&GetArray(ref a1, ref n1), &GetArray(ref a2, ref n2)) => {
            cmp_imm(vmap.clone(), a1, a2) && cmp_imm(vmap, n1, n2)
        }
        (&SetArray(ref a1, ref n1, ref v1), &SetArray(ref a2, ref n2, ref v2)) => {
            cmp_imm(vmap.clone(), a1, a2) && cmp_imm(vmap.clone(), n1, n2) && cmp_imm(vmap, v1, v2)
        }
        (&Imm(ref i1), &Imm(ref i2)) => cmp_imm(vmap, i1, i2),
        _ => false,
    }
}

fn cmp_expr(vmap: HashMap<Id, Id>, e1: &Expr, e2: &Expr) -> bool {
    let mut vmap = vmap;
    use self::Expr::*;

    match (e1, e2) {
        (&If(ref cond1, ref l1, ref r1), &If(ref cond2, ref l2, ref r2)) => {
            cmp_imm(vmap.clone(), cond1, cond2) && cmp_expr(vmap.clone(), l1, l2) && cmp_expr(vmap, r1, r2)
        }
        (&App(ref l1, ref r1), &App(ref l2, ref r2)) => {
            cmp_imm(vmap.clone(), l1, l2) && cmp_imm(vmap, r1, r2)
        }
        (&Let(ref v1, ref l1, ref r1), &Let(ref v2, ref l2, ref r2)) => {
            if !cmp_expr(vmap.clone(), l1, l2) {
                return false;
            }
            vmap.insert(v1.clone(), v2.clone());
            cmp_expr(vmap, r1, r2)
        }
        (&Op(ref op1), &Op(ref op2)) => cmp_op(vmap, op1, op2),
        _ => false,
    }
}

#[allow(dead_code)]
pub fn cmp(e1: &Expr, e2: &Expr) -> bool {
    let vmap = HashMap::new();
    cmp_expr(vmap, e1, e2)
}

macro_rules! test_anf(
    ($s:expr, $ae:expr) => { {
        use implicit_parse;
        use tok::Tokenizer;
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
        let expected = $ae;
        if !cmp(&anf_expr, &expected) {
            die!("cmp mismatch {:?} != {:?}", anf_expr, expected);
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
        Let(String::from("!tmp!0"), box Op(Op2(O::Add, box Imm(I::Int(2)), box Imm(I::Int(3)))),
            box Op(Imm(I::Var(String::from("!tmp!0"))))));

    test_anf!(
        "2+(3 - 2)",
        Let(String::from("!tmp!0"), box Op(Op2(O::Sub, box Imm(I::Int(3)), box Imm(I::Int(2)))),
            box Let(String::from("!tmp!1"), box Op(Op2(O::Add, box Imm(I::Int(2)), box Imm(I::Var(String::from("!tmp!0"))))),
                    box Op(Imm(I::Var(String::from("!tmp!1")))))));

    test_anf!(
        "2 + 3 - 2",
        Let(String::from("!tmp!0"), box Op(Op2(O::Add, box Imm(I::Int(2)), box Imm(I::Int(3)))),
            box Let(String::from("!tmp!1"), box Op(Op2(O::Sub, box Imm(I::Var(String::from("!tmp!0"))), box Imm(I::Int(2)))),
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

    test_anf!(
        "let f = (fun n -> n + 1) in f (2+3)",
        Let(String::from("!tmp!1"), box Op(Imm(I::Fun(String::from("n!a2"),
                                                      box Let(String::from("!tmp!0"), box Op(Op2(O::Add, box Imm(I::Var(String::from("n!a2"))), box Imm(I::Int(1)))),
                                                              box Op(Imm(I::Var(String::from("!tmp!0")))))))),
            box Let(String::from("f!a1"), box Op(Imm(I::Var(String::from("!tmp!1")))),
                    box Let(String::from("!tmp!2"), box Op(Op2(O::Add, box Imm(I::Int(2)), box Imm(I::Int(3)))),
                            box Let(String::from("!tmp!3"), box App(box I::Var(String::from("f!a1")), box I::Var(String::from("!tmp!2"))),
                                    box Op(Imm(I::Var(String::from("!tmp!3")))))))));
}
