use common::{Op2};
use lambdal::{Expr,Op,Imm};
use std::collections::HashMap;

type Closure = HashMap<String, Value>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    VInt(i64),
    VBool(bool),
    VClosure(Box<Closure>, String, Box<Expr>),
    VIntArray(Box<Vec<i64>>),
}

fn vint(v: Value) -> i64 {
    match v {
        Value::VInt(i) => i,
        _ => panic!("unreachable -- expected int not {:?}", v)
    }
}

fn vbool(v: Value) -> bool {
    match v {
        Value::VBool(b) => b,
        _ => panic!("unreachable -- expected bool not {:?}", v)
    }
}

fn vclosure(v: Value) -> (Box<Closure>, String, Box<Expr>) {
    match v {
        Value::VClosure(ctx, id, e) => (ctx, id, e),
        _ => panic!("unreachable -- expected closure not {:?}", v)
    }
}

fn vintarray(v: Value) -> Box<Vec<i64>> {
    match v {
        Value::VIntArray(a) => a,
        _ => panic!("unreachable -- expected intarray not {:?}", v)
    }
}

fn eval_op2(ctx: &Closure, op: Op2, l: &Op, r: &Op) -> Value {
    use common::Op2::*;
    use self::Value::*;

    // all binary ops operate on ints, and at this point have passed
    // typechecking
    match op {
        LT | LTE | GT | GTE | Eq | Add | Sub | Mul => {
            let vl = vint(eval_op(ctx, l));
            let vr = vint(eval_op(ctx, r));

            match op {
                LT  => VBool(vl < vr),
                LTE => VBool(vl <= vr),
                GT  => VBool(vl > vr),
                GTE => VBool(vl >= vr),
                Eq  => VBool(vl == vr),
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
                And => VBool(vl < vr),
                Or => VBool(vl > vr),
                _ => panic!("unreachable logic op {:?}", op),
            }

        }
    }
}

fn subst_imm(ctx: &Closure, id: &String, fix: &Imm, i: &Imm) -> Imm {
    use lambdal::Imm::*;
    match *i {
        Bool(b) => Bool(b),
        Int(n) => Int(n),
        Var(ref x) => {
            if x == id {
                fix.clone()
            } else {
                Var(x.clone())
            }
        }
        Fun(ref vid, ref e) => {
            let e = box subst_expr(ctx, id, fix, e);
            Fun(vid.clone(), e)
        }
        Fix(ref vid, ref e) => {
            let e = box subst_expr(ctx, id, fix, e);
            Fix(vid.clone(), e)
        }
        V | Star => unreachable!("ν or ★ encountered during subst"),
    }
}

fn subst_op(ctx: &Closure, id: &String, fix: &Imm, o: &Op) -> Op {
    use lambdal::Op::*;
    match *o {
        Op2(op, ref e1, ref e2) => {
            let e1 = box subst_op(ctx, id, fix, e1);
            let e2 = box subst_op(ctx, id, fix, e2);
            Op2(op.clone(), e1, e2)
        }
        MkArray(ref sz, ref n) => {
            let sz = box subst_imm(ctx, id, fix, sz);
            let n = box subst_imm(ctx, id, fix, n);
            MkArray(sz, n)
        }
        GetArray(ref iid, ref idx) => {
            let iid = box subst_imm(ctx, id, fix, iid);
            let idx = box subst_imm(ctx, id, fix, idx);
            GetArray(iid, idx)
        }
        SetArray(ref iid, ref idx, ref v) => {
            let iid = box subst_imm(ctx, id, fix, iid);
            let idx = box subst_imm(ctx, id, fix, idx);
            let v = box subst_imm(ctx, id, fix, v);
            SetArray(iid, idx, v)
        }
        Imm(ref imm) => Imm(subst_imm(ctx, id, fix, imm)),
    }
}

// fixpoint substitution
fn subst_expr(ctx: &Closure, id: &String, fix: &Imm, e: &Expr) -> Expr {
    use lambdal::Expr::*;
    match *e {
        If(ref e1, ref e2, ref e3) => {
            let e1 = box subst_imm(ctx, id, fix, e1);
            let e2 = box subst_expr(ctx, id, fix, e2);
            let e3 = box subst_expr(ctx, id, fix, e3);
            If(e1, e2, e3)
        }
        Let(ref vid, ref e1, ref e2) => {
            let e1 = box subst_expr(ctx, id, fix, e1);
            let e2 = box subst_expr(ctx, id, fix, e2);
            Let(vid.clone(), e1, e2)
        }
        App(ref e1, ref e2) => {
            let e1 = box subst_imm(ctx, id, fix, e1);
            let e2 = box subst_imm(ctx, id, fix, e2);
            App(e1, e2)
        }
        Op(ref op) => Op(subst_op(ctx, id, fix, op)),
    }
}

fn eval_imm(ctx: &Closure, i: &Imm) -> Value {
    use lambdal::Imm::*;
    use self::Value::*;
    match *i {
        Bool(b) => VBool(b),
        Int(i) => VInt(i),
        Var(ref id) => {
            match ctx.get(id) {
                Some(v) => v.clone(),
                None => panic!("lookup {} in ctx failed: {:?}", id, ctx),
            }
        }
        Fun(ref id, ref e) => VClosure(box ctx.clone(), id.clone(), e.clone()),
        Fix(ref id, ref e) => {
            let inner = eval(ctx, e);
            let (_, iid, ie) = vclosure(inner);
            let substituted_exp = box subst_expr(ctx, id, i, &ie);
            VClosure(box ctx.clone(), iid, substituted_exp)
        }
        V | Star => unreachable!("ν or ★ encountered during subst"),
    }
}

fn eval_op(ctx: &Closure, o: &Op) -> Value {
    use lambdal::Op::*;
    use self::Value::*;
    match *o {
        Op2(op, ref e1, ref e2) => eval_op2(ctx, op, e1, e2),
        MkArray(ref sz, ref n) => {
            let sz = vint(eval_imm(ctx, sz));
            let n = vint(eval_imm(ctx, n));
            let mut vec = Vec::with_capacity(sz as usize);
            vec.resize(sz as usize, n);
            VIntArray(box vec)
        }
        GetArray(ref iid, ref idx) => {
            let arr = vintarray(eval_imm(ctx, iid));
            let idx = vint(eval_imm(ctx, idx));
            VInt(arr[idx as usize])
        }
        SetArray(ref iid, ref idx, ref v) => {
            let mut arr = vintarray(eval_imm(ctx, iid)).clone();
            let idx = vint(eval_imm(ctx, idx));
            let v = vint(eval_imm(ctx, v));
            arr[idx as usize] = v;
            VIntArray(arr)
        }
        Imm(ref imm) => eval_imm(ctx, imm)
    }
}

fn eval(ctx: &Closure, expr: &Expr) -> Value {
    use lambdal::Expr::*;
    match *expr {
        If(ref cond, ref e1, ref e2) => {
            if vbool(eval_imm(ctx, cond)) {
                eval(ctx, e1)
            } else {
                eval(ctx, e2)
            }
        }
        App(ref e1, ref e2) => {
            let v = eval_imm(ctx, e2);
            let (ctx, id, e) = vclosure(eval_imm(ctx, e1));
            let mut new_ctx = ctx.clone();
            new_ctx.insert(id, v);
            eval(&new_ctx, &e)
        }
        Let(ref id, ref e1, ref e2) => {
            let v1 = eval(ctx, e1);
            let mut new_ctx = ctx.clone();
            new_ctx.insert(id.clone(), v1);
            eval(&new_ctx, e2)
        }
        Op(ref op) => {
            eval_op(ctx, op)
        }
    }
}

pub fn interpret(expr: &Expr) -> Value {
    let ctx: Closure = HashMap::new();

    eval(&ctx, expr)
}

#[cfg(test)]
macro_rules! test_eval(
    ($s:expr, $v:expr) => { {
        use implicit_parse;
        use tok::Tokenizer;
        use lambdal;
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        let iexpr = match implicit_parse::parse_Program(&s, tokenizer) {
            Ok(iexpr) => iexpr,
            Err(e) => {
                die!("parse_Program({}): {:?}", $s, e);
            }
        };
        let anf_expr = match lambdal::anf(&iexpr) {
            Ok((anf_expr, _)) => anf_expr,
            Err(e) => {
                die!("anf: {:?}", e);
            }
        };
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
    test_eval!("let rec factorial = fun x -> if x = 0 then 1 else x * (factorial (x - 1)) in factorial 5",
               VInt(120));
}
