use common::{self,Op2};
use explicit::Expr;
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

fn op2_eval(ctx: &Closure, op: Op2, l: &Expr, r: &Expr) -> Value {
    use common::Op2::*;
    use self::Value::*;

    // all binary ops operate on ints, and at this point have passed
    // typechecking
    match op {
        LT | GT | Eq | Add | Sub | Mul => {
            let vl = vint(eval(ctx, l));
            let vr = vint(eval(ctx, r));

            match op {
                LT => VBool(vl < vr),
                GT => VBool(vl > vr),
                Eq => VBool(vl == vr),
                Add => VInt(vl + vr),
                Sub => VInt(vl - vr),
                Mul => VInt(vl * vr),
                _ => panic!("unreachable numerical op {:?}", op),
            }
        }
        And | Or | Impl | Iff => {
            let vl = vbool(eval(ctx, l));
            let vr = vbool(eval(ctx, r));

            match op {
                And => VBool(vl < vr),
                Or => VBool(vl > vr),
                _ => panic!("unreachable logic op {:?}", op),
            }

        }
    }
}

// fixpoint substitution
fn subst(ctx: &Closure, id: &String, fix: &Expr, e: &Expr) -> Expr {
    use typed::Expr::*;
    match *e {
        Const(ref c) => Const(c.clone()),
        Op2(op, ref e1, ref e2) => {
            let e1 = box subst(ctx, id, fix, e1);
            let e2 = box subst(ctx, id, fix, e2);
            Op2(op.clone(), e1, e2)
        }
        If(ref e1, ref e2, ref e3) => {
            let e1 = box subst(ctx, id, fix, e1);
            let e2 = box subst(ctx, id, fix, e2);
            let e3 = box subst(ctx, id, fix, e3);
            If(e1, e2, e3)
        }
        Var(ref x) => {
            if x == id {
                fix.clone()
            } else {
                Var(x.clone())
            }
        }
        Let(ref vid, ref e1, ref e2) => {
            let e1 = box subst(ctx, id, fix, e1);
            let e2 = box subst(ctx, id, fix, e2);
            Let(vid.clone(), e1, e2)
        }
        Fun(ref vid, ref ty, ref e) => {
            let e = box subst(ctx, id, fix, e);
            Fun(vid.clone(), ty.clone(), e)
        }
        Fix(ref vid, ref ty, ref e) => {
            let e = box subst(ctx, id, fix, e);
            Fix(vid.clone(), ty.clone(), e)
        }
        App(ref e1, ref e2) => {
            let e1 = box subst(ctx, id, fix, e1);
            let e2 = box subst(ctx, id, fix, e2);
            App(e1, e2)
        }
        MkArray(ref sz, ref n) => {
            let sz = box subst(ctx, id, fix, sz);
            let n = box subst(ctx, id, fix, n);
            MkArray(sz, n)
        }
        GetArray(ref iid, ref idx) => {
            let iid = box subst(ctx, id, fix, iid);
            let idx = box subst(ctx, id, fix, idx);
            GetArray(iid, idx)
        }
        SetArray(ref iid, ref idx, ref v) => {
            let iid = box subst(ctx, id, fix, iid);
            let idx = box subst(ctx, id, fix, idx);
            let v = box subst(ctx, id, fix, v);
            SetArray(iid, idx, v)
        }
        Star => panic!("star found when it shouldn't be"),
        V => panic!("v found when it shouldn't be"),
        WellFormed(_) => panic!("wellformed found when it shouldn't be"),
    }
}

fn eval(ctx: &Closure, expr: &Expr) -> Value {
    use self::Value::*;
    use typed::Expr::*;
    match *expr {
        Const(common::Const::Int(i)) => VInt(i),
        Const(common::Const::Bool(b)) => VBool(b),
        Op2(op, ref e1, ref e2) => op2_eval(ctx, op, e1, e2),
        If(ref e1, ref e2, ref e3) => {
            if vbool(eval(ctx, e1)) {
                eval(ctx, e2)
            } else {
                eval(ctx, e3)
            }
        }
        Var(ref x) => {
            match ctx.get(x) {
                Some(v) => v.clone(),
                None => {
                    panic!("variable not in our environment? {:?}", x);
                }
            }
        }
        Let(ref id, ref e1, ref e2) => {
            let v1 = eval(ctx, e1);
            let mut new_ctx = ctx.clone();
            new_ctx.insert(id.clone(), v1);
            eval(&new_ctx, e2)
        }
        Fun(ref id, _, ref e) => {
            VClosure(box ctx.clone(), id.clone(), e.clone())
        }
        Fix(ref id, _, ref e1) => {
            let inner = eval(ctx, e1);
            let (_, iid, ie) = vclosure(inner);
            let substituted_exp = box subst(ctx, id, expr, &ie);
            VClosure(box ctx.clone(), iid, substituted_exp)
        }
        App(ref e1, ref e2) => {
            let v = eval(ctx, e2);
            let (ctx, id, e) = vclosure(eval(ctx, e1));
            let mut new_ctx = ctx.clone();
            new_ctx.insert(id, v);
            eval(&new_ctx, &e)
        }
        MkArray(ref sz, ref n) => {
            let sz = vint(eval(ctx, sz));
            let n = vint(eval(ctx, n));
            let mut vec = Vec::with_capacity(sz as usize);
            vec.resize(sz as usize, n);
            VIntArray(box vec)
        }
        GetArray(ref iid, ref idx) => {
            let arr = vintarray(eval(ctx, iid));
            let idx = vint(eval(ctx, idx));
            VInt(arr[idx as usize])
        }
        SetArray(ref iid, ref idx, ref v) => {
            let mut arr = vintarray(eval(ctx, iid)).clone();
            let idx = vint(eval(ctx, idx));
            let v = vint(eval(ctx, v));
            arr[idx as usize] = v;
            VIntArray(arr)
        }
        Star => panic!("star found when it shouldn't be"),
        V => panic!("v found when it shouldn't be"),
        WellFormed(_) => panic!("wellformed found when it shouldn't be"),
    }
}

pub fn interpret(expr: &Expr) -> Value {
    let ctx: Closure = HashMap::new();

    eval(&ctx, expr)
}
