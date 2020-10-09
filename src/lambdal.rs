use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::common::{self, Const, Id, Op2};
use crate::env;
use crate::explicit::{self, Type};
use crate::hindley_milner;
use crate::implicit;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Imm {
    Bool(bool),
    Int(i64),
    Var(Id),
    Fun(Id, Box<Expr>),
    Fix(Id, Box<Expr>),
    Star,
    V,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Expr {
    If(Box<Imm>, Box<Expr>, Box<Expr>),
    App(Box<Imm>, Box<Imm>),
    Let(Id, Box<Expr>, Box<Expr>),
    Op(Op),
}

impl Debug for Imm {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Imm::*;
        match self {
            Bool(b) => write!(fmt, "{}", b),
            Int(n) => write!(fmt, "{}", n),
            Var(id) => write!(fmt, "{}", id),
            Fun(id, e) => write!(fmt, "fun {} -> {:?}", id, e),
            Fix(id, e) => write!(fmt, "fix {} -> {:?}", id, e),
            Star => write!(fmt, "★"),
            V => write!(fmt, "ν"),
        }
    }
}

impl Debug for Op {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Op::*;
        match self {
            Op2(op, l, r) => write!(fmt, "{:?} {:?} {:?}", l, op, r),
            MkArray(sz, n) => write!(fmt, "array({:?}, {:?})", sz, n),
            GetArray(a, idx) => write!(fmt, "{:?}[{:?}]", a, idx),
            SetArray(a, idx, n) => write!(fmt, "{:?}[{:?}] = {:?}", a, idx, n),
            Imm(imm) => write!(fmt, "{:?}", imm),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match self {
            If(cond, l, r) => write!(fmt, "if {:?} then {:?} else {:?}", cond, l, r),
            App(e1, e2) => write!(fmt, "{:?} {:?}", e1, e2),
            Let(id, e1, e2) => write!(fmt, "let {} = {:?} in {:?}", id, e1, e2),
            Op(op) => write!(fmt, "{:?}", op),
        }
    }
}

static ENV_ID: AtomicUsize = AtomicUsize::new(0);

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
        let c = ConvEnv {
            env_id: self.env_id,
            next_id: self.next_id + 1,
        };

        (c, format!("tmp{}_{}", self.env_id, id))
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
fn expr(
    cenv: ConvEnv,
    e: &explicit::Expr,
    k: &dyn Fn(ConvEnv, Imm) -> (ConvEnv, Expr),
) -> (ConvEnv, Expr) {
    use self::Expr::*;
    use self::Imm as I;
    use self::Op::*;
    use crate::typed::Expr as E;

    match e {
        E::Const(c) => k(cenv, constant(c)),
        E::Op2(op, l, r) => {
            expr(cenv, l, &|cenv, ll| {
                expr(cenv, r, &|cenv, rr| {
                    let (cenv, op_tmp) = cenv.tmp();
                    let op_val = Op(Op2(*op, Box::new(Imm(ll.clone())), Box::new(Imm(rr))));
                    // value to pass to the continuation
                    let op_ref = I::Var(op_tmp.clone());

                    // our inner expression is whatever the
                    // continuation says it is.
                    let (cenv, result) = k(cenv, op_ref);

                    (cenv, Let(op_tmp, Box::new(op_val), Box::new(result)))
                })
            })
        }
        E::Let(id, e1, e2) => expr(cenv, e1, &|cenv, e1l| {
            let (cenv, inner) = expr(cenv, e2, k);
            (
                cenv,
                Let(id.clone(), Box::new(Op(Imm(e1l.clone()))), Box::new(inner)),
            )
        }),
        E::Var(x) => k(cenv, I::Var(x.clone())),
        E::If(e1, e2, e3) => {
            expr(cenv, e1, &|cenv, cond_ref| {
                let (cenv, l_true) = expr(cenv, e2, &identity);
                let (cenv, l_false) = expr(cenv, e3, &identity);

                // value to pass to the continuation
                let val = If(Box::new(cond_ref), Box::new(l_true), Box::new(l_false));
                let (cenv, val_tmp) = cenv.tmp();

                // our inner expression is whatever the
                // continuation says it is.
                let (cenv, mut result) = k(cenv, I::Var(val_tmp.clone()));

                result = Let(val_tmp, Box::new(val), Box::new(result));

                (cenv, result)
            })
        }
        E::Fun(id, _, e) => {
            let (cenv, fun) = expr(cenv, e, &identity);
            let (cenv, fun_ref) = cenv.tmp();
            let (cenv, result) = k(cenv, I::Var(fun_ref.clone()));

            (
                cenv,
                Let(
                    fun_ref,
                    Box::new(Op(Imm(I::Fun(id.clone(), Box::new(fun))))),
                    Box::new(result),
                ),
            )
        }
        E::Fix(id, _, e) => {
            let (cenv, fix) = expr(cenv, e, &identity);
            let (cenv, fix_ref) = cenv.tmp();
            let (cenv, result) = k(cenv, I::Var(fix_ref.clone()));

            (
                cenv,
                Let(
                    fix_ref,
                    Box::new(Op(Imm(I::Fix(id.clone(), Box::new(fix))))),
                    Box::new(result),
                ),
            )
        }
        E::App(e1, e2) => {
            expr(cenv, e1, &|cenv, ie1| {
                expr(cenv, e2, &|cenv, ie2| {
                    let (cenv, app_tmp) = cenv.tmp();
                    let app_val = App(Box::new(ie1.clone()), Box::new(ie2.clone()));
                    // value to pass to the continuation
                    let app_ref = I::Var(app_tmp.clone());

                    // our inner expression is whatever the
                    // continuation says it is.
                    let (cenv, result) = k(cenv, app_ref);

                    (cenv, Let(app_tmp, Box::new(app_val), Box::new(result)))
                })
            })
        }
        E::MkArray(sz, val) => expr(cenv, sz, &|cenv, isz| {
            expr(cenv, val, &|cenv, ival| {
                let val = Op(MkArray(Box::new(isz.clone()), Box::new(ival)));
                let (cenv, val_ref) = cenv.tmp();
                let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                (cenv, Let(val_ref, Box::new(val), Box::new(result)))
            })
        }),
        E::GetArray(id, idx) => expr(cenv, id, &|cenv, iid| {
            expr(cenv, idx, &|cenv, iidx| {
                let val = Op(GetArray(Box::new(iid.clone()), Box::new(iidx)));
                let (cenv, val_ref) = cenv.tmp();
                let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                (cenv, Let(val_ref, Box::new(val), Box::new(result)))
            })
        }),
        E::SetArray(id, idx, v) => expr(cenv, id, &|cenv, iid| {
            expr(cenv, idx, &|cenv, iidx| {
                expr(cenv, v, &|cenv, iv| {
                    let val = Op(SetArray(
                        Box::new(iid.clone()),
                        Box::new(iidx.clone()),
                        Box::new(iv),
                    ));
                    let (cenv, val_ref) = cenv.tmp();
                    let (cenv, result) = k(cenv, I::Var(val_ref.clone()));
                    (cenv, Let(val_ref, Box::new(val), Box::new(result)))
                })
            })
        }),
        E::V => k(cenv, I::V),
        E::Star => k(cenv, I::Star),
    }
}

fn build_env_imm(env: HashMap<Id, Type>, i: &Imm) -> (HashMap<Id, Type>, Type) {
    use self::Imm::*;
    match i {
        Bool(_) => (env, Type::TBool),
        Int(_) => (env, Type::TInt),
        Var(id) => {
            let ty = match env.get(id) {
                Some(ty) => ty.clone(),
                None => panic!("no key '{}' in {:?}", id, env),
            };
            (env, ty)
        }
        Fun(id, e) => {
            let (env, return_type) = build_env_expr(env, e);
            let arg_type = env[id].clone();
            (
                env,
                Type::TFun(id.clone(), Box::new(arg_type), Box::new(return_type)),
            )
        }
        Fix(_, e) => build_env_expr(env, e),
        V | Star => unreachable!("ν or ★ encountered during build_env"),
    }
}

fn build_env_op(env: HashMap<Id, Type>, o: &Op) -> (HashMap<Id, Type>, Type) {
    use self::Op::*;
    match o {
        Op2(op, e1, e2) => {
            let (env, _) = build_env_op(env, e1);
            let (env, _) = build_env_op(env, e2);
            (env, explicit::opty(*op))
        }
        MkArray(e1, e2) => {
            let (env, _) = build_env_imm(env, e1);
            let (env, _) = build_env_imm(env, e2);
            (env, Type::TIntArray)
        }
        GetArray(e1, e2) => {
            let (env, _) = build_env_imm(env, e1);
            let (env, _) = build_env_imm(env, e2);
            (env, Type::TInt)
        }
        SetArray(e1, e2, e3) => {
            let (env, _) = build_env_imm(env, e1);
            let (env, _) = build_env_imm(env, e2);
            let (env, _) = build_env_imm(env, e3);
            (env, Type::TIntArray)
        }
        Imm(i) => build_env_imm(env, i),
    }
}

fn build_env_expr(env: HashMap<Id, Type>, e: &Expr) -> (HashMap<Id, Type>, Type) {
    use self::Expr::*;
    match e {
        If(cond, e1, e2) => {
            let (env, _) = build_env_imm(env, cond);
            let (env, ty) = build_env_expr(env, e1);
            let (env, _) = build_env_expr(env, e2);
            (env, ty)
        }
        App(e1, _) => {
            let (env, e1_type) = build_env_imm(env, e1);
            if let Type::TFun(_, _, ret_type) = e1_type {
                (env, *ret_type)
            } else {
                unreachable!("app of non-fun should have been caught by HM")
            }
        }
        Let(id, e1, e2) => {
            let (env, id_ty) = build_env_expr(env, e1);
            let mut env = env.clone();
            env.insert(id.clone(), id_ty);

            build_env_expr(env, e2)
        }
        Op(op) => build_env_op(env, op),
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
    use self::Imm::*;
    use implicit::Expr as I;

    let imm = match e {
        I::Op2(op, l, r) => {
            let l = q_op(l)?;
            let r = q_op(r)?;
            return Ok(Op::Op2(*op, Box::new(l), Box::new(r)));
        }
        I::Var(id) => Var(id.clone()),
        I::Const(Const::Int(b)) => Int(*b),
        I::Const(Const::Bool(b)) => Bool(*b),
        I::Star => {
            return err!("found star.");
        }
        I::V => V,
        _ => {
            panic!("unexpected imm in q: {:?}", e);
        }
    };
    Ok(Op::Imm(imm))
}

#[allow(dead_code)]
pub fn q(implicit_expr: &implicit::Expr) -> common::Result<Expr> {
    if let Ok(op) = q_op(implicit_expr) {
        return Ok(Expr::Op(op));
    }

    err!("expected simple Q")
}

#[test]
fn test_q() {
    use common::Op2::*;

    let q1 = implicit::Expr::Op2(
        LT,
        Box::new(implicit::Expr::V),
        Box::new(implicit::Expr::Const(common::Const::Int(3))),
    );
    let ql = match q(&q1) {
        Ok(expr) => expr,
        Err(e) => die!("q: {:?}", e),
    };

    let expected = Expr::Op(Op::Op2(
        LT,
        Box::new(Op::Imm(Imm::V)),
        Box::new(Op::Imm(Imm::Int(3))),
    ));

    if !cmp(&ql, &expected) {
        die!("conversion of q failed: {:?}", ql)
    };
}

fn cmp_imm(vmap: HashMap<Id, Id>, imm1: &Imm, imm2: &Imm) -> bool {
    let mut vmap = vmap;
    use self::Imm::*;

    match (imm1, imm2) {
        (Bool(b1), Bool(b2)) => b1 == b2,
        (Int(n1), Int(n2)) => n1 == n2,
        (Var(v1), Var(v2)) => {
            let expected_v2 = match vmap.get(v1) {
                Some(v) => v,
                None => {
                    return false;
                }
            };
            expected_v2 == v2
        }
        (Fun(id1, e1), Fun(id2, e2)) => {
            vmap.insert(id1.clone(), id2.clone());
            cmp_expr(vmap, e1, e2)
        }
        (Fix(id1, e1), Fun(id2, e2)) => {
            vmap.insert(id1.clone(), id2.clone());
            cmp_expr(vmap, e1, e2)
        }
        (Star, Star) => true,
        (V, V) => true,
        _ => false,
    }
}

fn cmp_op(vmap: HashMap<Id, Id>, e1: &Op, e2: &Op) -> bool {
    use self::Op::*;

    match (e1, e2) {
        (Op2(op1, l1, r1), Op2(op2, l2, r2)) => {
            op1 == op2 && cmp_op(vmap.clone(), l1, l2) && cmp_op(vmap, r1, r2)
        }
        (MkArray(sz1, n1), MkArray(sz2, n2)) => {
            cmp_imm(vmap.clone(), sz1, sz2) && cmp_imm(vmap, n1, n2)
        }
        (GetArray(a1, n1), GetArray(a2, n2)) => {
            cmp_imm(vmap.clone(), a1, a2) && cmp_imm(vmap, n1, n2)
        }
        (SetArray(a1, n1, v1), SetArray(a2, n2, v2)) => {
            cmp_imm(vmap.clone(), a1, a2) && cmp_imm(vmap.clone(), n1, n2) && cmp_imm(vmap, v1, v2)
        }
        (Imm(i1), Imm(i2)) => cmp_imm(vmap, i1, i2),
        _ => false,
    }
}

fn cmp_expr(vmap: HashMap<Id, Id>, e1: &Expr, e2: &Expr) -> bool {
    let mut vmap = vmap;
    use self::Expr::*;

    match (e1, e2) {
        (If(cond1, l1, r1), If(cond2, l2, r2)) => {
            cmp_imm(vmap.clone(), cond1, cond2)
                && cmp_expr(vmap.clone(), l1, l2)
                && cmp_expr(vmap, r1, r2)
        }
        (App(l1, r1), App(l2, r2)) => cmp_imm(vmap.clone(), l1, l2) && cmp_imm(vmap, r1, r2),
        (Let(v1, l1, r1), Let(v2, l2, r2)) => {
            if !cmp_expr(vmap.clone(), l1, l2) {
                return false;
            }
            vmap.insert(v1.clone(), v2.clone());
            cmp_expr(vmap, r1, r2)
        }
        (Op(op1), Op(op2)) => cmp_op(vmap, op1, op2),
        _ => false,
    }
}

#[allow(dead_code)]
pub fn cmp(e1: &Expr, e2: &Expr) -> bool {
    let vmap = HashMap::new();
    cmp_expr(vmap, e1, e2)
}

#[cfg(test)]
macro_rules! test_anf(
    ($s:expr, $ae:expr) => { {
        use crate::implicit_parse::ProgramParser;
        use crate::tok::Tokenizer;
        let input = $s;
        let lexer = Tokenizer::new(&input);
        let iexpr = ProgramParser::new().parse(input, lexer).unwrap();
        let (anf_expr, _) = anf(&iexpr).unwrap();
        let expected = $ae;
        if !cmp(&anf_expr, &expected) {
            die!("cmp mismatch {:?} != {:?}", anf_expr, expected);
        }
    } }
);

#[test]
fn anf_transforms() {
    use self::Expr::*;
    use self::Imm as I;
    use self::Op::*;
    use crate::common::Op2 as O;

    test_anf!("-22", Op(Imm(I::Int(-22))));

    test_anf!(
        "2+3",
        Let(
            String::from("!tmp!0"),
            Box::new(Op(Op2(
                O::Add,
                Box::new(Imm(I::Int(2))),
                Box::new(Imm(I::Int(3)))
            ))),
            Box::new(Op(Imm(I::Var(String::from("!tmp!0")))))
        )
    );

    test_anf!(
        "2+(3 - 2)",
        Let(
            String::from("!tmp!0"),
            Box::new(Op(Op2(
                O::Sub,
                Box::new(Imm(I::Int(3))),
                Box::new(Imm(I::Int(2)))
            ))),
            Box::new(Let(
                String::from("!tmp!1"),
                Box::new(Op(Op2(
                    O::Add,
                    Box::new(Imm(I::Int(2))),
                    Box::new(Imm(I::Var(String::from("!tmp!0"))))
                ))),
                Box::new(Op(Imm(I::Var(String::from("!tmp!1")))))
            ))
        )
    );

    test_anf!(
        "2 + 3 - 2",
        Let(
            String::from("!tmp!0"),
            Box::new(Op(Op2(
                O::Add,
                Box::new(Imm(I::Int(2))),
                Box::new(Imm(I::Int(3)))
            ))),
            Box::new(Let(
                String::from("!tmp!1"),
                Box::new(Op(Op2(
                    O::Sub,
                    Box::new(Imm(I::Var(String::from("!tmp!0")))),
                    Box::new(Imm(I::Int(2)))
                ))),
                Box::new(Op(Imm(I::Var(String::from("!tmp!1")))))
            ))
        )
    );

    test_anf!(
        "let x = 1 in x",
        Let(
            String::from("x!a1"),
            Box::new(Op(Imm(I::Int(1)))),
            Box::new(Op(Imm(I::Var(String::from("x!a1")))))
        )
    );

    test_anf!(
        "let x = (if true then 1 else 2) in x",
        Let(
            String::from("!tmp!0"),
            Box::new(If(
                Box::new(I::Bool(true)),
                Box::new(Op(Imm(I::Int(1)))),
                Box::new(Op(Imm(I::Int(2))))
            )),
            Box::new(Let(
                String::from("x!a1"),
                Box::new(Op(Imm(I::Var(String::from("!tmp!0"))))),
                Box::new(Op(Imm(I::Var(String::from("x!a1")))))
            ))
        )
    );

    test_anf!(
        "let a = array(3, 5) in a[0]",
        Let(
            String::from("!tmp!0"),
            Box::new(Op(MkArray(Box::new(I::Int(3)), Box::new(I::Int(5))))),
            Box::new(Let(
                String::from("a!a1"),
                Box::new(Op(Imm(I::Var(String::from("!tmp!0"))))),
                Box::new(Let(
                    String::from("!tmp!1"),
                    Box::new(Op(GetArray(
                        Box::new(I::Var(String::from("a!a1"))),
                        Box::new(I::Int(0))
                    ))),
                    Box::new(Op(Imm(I::Var(String::from("!tmp!1")))))
                ))
            ))
        )
    );

    test_anf!(
        "let f = (fun n -> n + 1) in f (2+3)",
        Let(
            String::from("!tmp!1"),
            Box::new(Op(Imm(I::Fun(
                String::from("n!a2"),
                Box::new(Let(
                    String::from("!tmp!0"),
                    Box::new(Op(Op2(
                        O::Add,
                        Box::new(Imm(I::Var(String::from("n!a2")))),
                        Box::new(Imm(I::Int(1)))
                    ))),
                    Box::new(Op(Imm(I::Var(String::from("!tmp!0")))))
                ))
            )))),
            Box::new(Let(
                String::from("f!a1"),
                Box::new(Op(Imm(I::Var(String::from("!tmp!1"))))),
                Box::new(Let(
                    String::from("!tmp!2"),
                    Box::new(Op(Op2(
                        O::Add,
                        Box::new(Imm(I::Int(2))),
                        Box::new(Imm(I::Int(3)))
                    ))),
                    Box::new(Let(
                        String::from("!tmp!3"),
                        Box::new(App(
                            Box::new(I::Var(String::from("f!a1"))),
                            Box::new(I::Var(String::from("!tmp!2")))
                        )),
                        Box::new(Op(Imm(I::Var(String::from("!tmp!3")))))
                    ))
                ))
            ))
        )
    );
}
