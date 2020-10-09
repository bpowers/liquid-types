use std::collections::HashMap;

use crate::common::Op2;
use crate::lambdal::{Expr, Imm, Op};

type Closure = HashMap<String, Value>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    VInt(i64),
    VBool(bool),
    VClosure(Box<Closure>, String, Box<Expr>),
    VIntArray(Vec<i64>),
}

fn vint(v: Value) -> i64 {
    match v {
        Value::VInt(i) => i,
        _ => panic!("unreachable -- expected int not {:?}", v),
    }
}

fn vbool(v: Value) -> bool {
    match v {
        Value::VBool(b) => b,
        _ => panic!("unreachable -- expected bool not {:?}", v),
    }
}

fn vclosure(v: Value) -> (Box<Closure>, String, Box<Expr>) {
    match v {
        Value::VClosure(ctx, id, e) => (ctx, id, e),
        _ => panic!("unreachable -- expected closure not {:?}", v),
    }
}

fn vintarray(v: Value) -> Vec<i64> {
    match v {
        Value::VIntArray(a) => a,
        _ => panic!("unreachable -- expected intarray not {:?}", v),
    }
}

fn eval_op2(ctx: &Closure, op: Op2, l: &Op, r: &Op) -> Value {
    use self::Value::*;
    use crate::common::Op2::*;

    // all binary ops operate on ints, and at this point have passed
    // typechecking
    match op {
        LT | LTE | GT | GTE | Eq | Add | Sub | Mul => {
            let vl = vint(eval_op(ctx, l));
            let vr = vint(eval_op(ctx, r));

            match op {
                LT => VBool(vl < vr),
                LTE => VBool(vl <= vr),
                GT => VBool(vl > vr),
                GTE => VBool(vl >= vr),
                Eq => VBool(vl == vr),
                Add => VInt(vl + vr),
                Sub => VInt(vl - vr),
                Mul => VInt(vl * vr),
                _ => panic!("unreachable numerical op {:?}", op),
            }
        }
        And | Or | Impl | Iff => {
            let vl = vbool(eval_op(ctx, l));
            let vr = vbool(eval_op(ctx, r));

            match op {
                And => VBool(vl && vr),
                Or => VBool(vl || vr),
                _ => panic!("unreachable logic op {:?}", op),
            }
        }
    }
}

fn subst_imm(ctx: &Closure, id: &str, fix: &Imm, i: &Imm) -> Imm {
    use crate::lambdal::Imm::*;
    match i {
        Bool(b) => Bool(*b),
        Int(n) => Int(*n),
        Var(x) => {
            if x == id {
                fix.clone()
            } else {
                Var(x.clone())
            }
        }
        Fun(vid, e) => {
            let e = Box::new(subst_expr(ctx, id, fix, e));
            Fun(vid.clone(), e)
        }
        Fix(vid, e) => {
            let e = Box::new(subst_expr(ctx, id, fix, e));
            Fix(vid.clone(), e)
        }
        V | Star => unreachable!("ν or ★ encountered during subst"),
    }
}

fn subst_op(ctx: &Closure, id: &str, fix: &Imm, o: &Op) -> Op {
    use crate::lambdal::Op::*;
    match o {
        Op2(op, e1, e2) => {
            let e1 = Box::new(subst_op(ctx, id, fix, e1));
            let e2 = Box::new(subst_op(ctx, id, fix, e2));
            Op2(*op, e1, e2)
        }
        MkArray(sz, n) => {
            let sz = Box::new(subst_imm(ctx, id, fix, sz));
            let n = Box::new(subst_imm(ctx, id, fix, n));
            MkArray(sz, n)
        }
        GetArray(iid, idx) => {
            let iid = Box::new(subst_imm(ctx, id, fix, iid));
            let idx = Box::new(subst_imm(ctx, id, fix, idx));
            GetArray(iid, idx)
        }
        SetArray(iid, idx, v) => {
            let iid = Box::new(subst_imm(ctx, id, fix, iid));
            let idx = Box::new(subst_imm(ctx, id, fix, idx));
            let v = Box::new(subst_imm(ctx, id, fix, v));
            SetArray(iid, idx, v)
        }
        Imm(imm) => Imm(subst_imm(ctx, id, fix, imm)),
    }
}

// fixpoint substitution
fn subst_expr(ctx: &Closure, id: &str, fix: &Imm, e: &Expr) -> Expr {
    use crate::lambdal::Expr::*;
    match e {
        If(e1, e2, e3) => {
            let e1 = Box::new(subst_imm(ctx, id, fix, e1));
            let e2 = Box::new(subst_expr(ctx, id, fix, e2));
            let e3 = Box::new(subst_expr(ctx, id, fix, e3));
            If(e1, e2, e3)
        }
        Let(vid, e1, e2) => {
            let e1 = Box::new(subst_expr(ctx, id, fix, e1));
            let e2 = Box::new(subst_expr(ctx, id, fix, e2));
            Let(vid.clone(), e1, e2)
        }
        App(e1, e2) => {
            let e1 = Box::new(subst_imm(ctx, id, fix, e1));
            let e2 = Box::new(subst_imm(ctx, id, fix, e2));
            App(e1, e2)
        }
        Op(op) => Op(subst_op(ctx, id, fix, op)),
    }
}

fn eval_imm(ctx: &Closure, i: &Imm) -> Value {
    use self::Value::*;
    use crate::lambdal::Imm::*;
    match i {
        Bool(b) => VBool(*b),
        Int(i) => VInt(*i),
        Var(id) => match ctx.get(id) {
            Some(v) => v.clone(),
            None => panic!("lookup {} in ctx failed: {:?}", id, ctx),
        },
        Fun(id, e) => VClosure(Box::new(ctx.clone()), id.clone(), e.clone()),
        Fix(id, e) => {
            let inner = eval(ctx, e);
            let (_, iid, ie) = vclosure(inner);
            let substituted_exp = Box::new(subst_expr(ctx, id, i, &ie));
            VClosure(Box::new(ctx.clone()), iid, substituted_exp)
        }
        V | Star => unreachable!("ν or ★ encountered during subst"),
    }
}

fn eval_op(ctx: &Closure, o: &Op) -> Value {
    use self::Value::*;
    use crate::lambdal::Op::*;
    match o {
        Op2(op, e1, e2) => eval_op2(ctx, *op, e1, e2),
        MkArray(sz, n) => {
            let sz = vint(eval_imm(ctx, sz));
            let n = vint(eval_imm(ctx, n));
            let mut vec = Vec::with_capacity(sz as usize);
            vec.resize(sz as usize, n);
            VIntArray(vec)
        }
        GetArray(iid, idx) => {
            let arr = vintarray(eval_imm(ctx, iid));
            let idx = vint(eval_imm(ctx, idx));
            VInt(arr[idx as usize])
        }
        SetArray(iid, idx, v) => {
            let mut arr = vintarray(eval_imm(ctx, iid));
            let idx = vint(eval_imm(ctx, idx));
            let v = vint(eval_imm(ctx, v));
            arr[idx as usize] = v;
            VIntArray(arr)
        }
        Imm(imm) => eval_imm(ctx, imm),
    }
}

fn eval(ctx: &Closure, expr: &Expr) -> Value {
    use crate::lambdal::Expr::*;
    match expr {
        If(cond, e1, e2) => {
            if vbool(eval_imm(ctx, cond)) {
                eval(ctx, e1)
            } else {
                eval(ctx, e2)
            }
        }
        App(e1, e2) => {
            let v = eval_imm(ctx, e2);
            let (ctx, id, e) = vclosure(eval_imm(ctx, e1));
            let mut new_ctx = ctx;
            new_ctx.insert(id, v);
            eval(&new_ctx, &e)
        }
        Let(id, e1, e2) => {
            let v1 = eval(ctx, e1);
            let mut new_ctx = ctx.clone();
            new_ctx.insert(id.clone(), v1);
            eval(&new_ctx, e2)
        }
        Op(op) => eval_op(ctx, op),
    }
}

pub fn interpret(expr: &Expr) -> Value {
    let ctx: Closure = HashMap::new();

    eval(&ctx, expr)
}

#[cfg(test)]
macro_rules! test_eval(
    ($s:expr, $v:expr) => { {
        use crate::implicit_parse::ProgramParser;
        use crate::tok::Tokenizer;
        use crate::lambdal;
        let input = $s;
        let lexer = Tokenizer::new(&input);

        let iexpr = ProgramParser::new().parse(input, lexer).unwrap();
        let (anf_expr, _) = lambdal::anf(&iexpr).unwrap();
        let r = interpret(&anf_expr);
        if r != $v {
            die!("mismatch {:?} != {:?}", r, $v);
        }
    } }
);

#[test]
fn eval_results() {
    use self::Value::*;

    test_eval!("-22", VInt(-22));
    test_eval!("let double = (fun n -> n*2) in double 8", VInt(16));
    test_eval!(
        "let rec factorial = fun x -> if x = 0 then 1 else x * (factorial (x - 1)) in factorial 5",
        VInt(120)
    );
}
