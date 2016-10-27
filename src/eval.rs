use common::{self,Op2};
use explicit::Expr;
use std::collections::HashMap;

type Closure = HashMap<String, Value>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    VInt(i64),
    VBool(bool),
    VClosure(Box<Closure>, String, Box<Expr>),
    VCons(Box<Value>, Box<Value>),
    VEmpty,
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
        _ => panic!("unreachable -- expected bool not {:?}", v)
    }
}


fn op2_eval(ctx: &Closure, op: Op2, l: &Expr, r: &Expr) -> Value {
    use common::Op2::*;
    use self::Value::*;

    // all binary ops operate on ints, and at this point have passed
    // typechecking
    let vl = vint(eval(ctx, l));
    let vr = vint(eval(ctx, r));

    match op {
        LT => VBool(vl < vr),
        GT => VBool(vl > vr),
        Eq => VBool(vl == vr),
        Add => VInt(vl + vr),
        Sub => VInt(vl - vr),
        Mul => VInt(vl * vr),
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
        Let(ref id, ref e1, ref e2) => {
            let e1 = box subst(ctx, id, fix, e1);
            let e2 = box subst(ctx, id, fix, e2);
            Let(id.clone(), e1, e2)
        }
        Fun(ref id, ref ty, ref e) => {
            let e = box subst(ctx, id, fix, e);
            Fun(id.clone(), ty.clone(), e)
        }
        Fix(ref id, ref ty, ref e) => {
            let e = box subst(ctx, id, fix, e);
            Fix(id.clone(), ty.clone(), e)
        }
        App(ref e1, ref e2) => {
            let e1 = box subst(ctx, id, fix, e1);
            let e2 = box subst(ctx, id, fix, e2);
            App(e1, e2)
        }
        Empty(ref ty) => Empty(ty.clone()),
        Cons(ref e1, ref e2) => {
            let e1 = box subst(ctx, id, fix, e1);
            let e2 = box subst(ctx, id, fix, e2);
            Cons(e1, e2)
        }
        Head(ref e) => {
            let e = box subst(ctx, id, fix, e);
            Head(e)
        }
        Tail(ref e) => {
            let e = box subst(ctx, id, fix, e);
            Tail(e)
        }
        IsEmpty(ref e) => {
            let e = box subst(ctx, id, fix, e);
            IsEmpty(e)
        }
    }
}

fn is_empty(v: &Value) -> Value {
    use self::Value::*;
    match *v {
        VEmpty => VBool(true),
        VCons(_, _) => VBool(false),
        _ => panic!("is_empty unreachable"),
    }
}

// let rec eval' (ctx : env) (e : exp) : value =
//   match e with
//   | Id      (id)           -> lookup ctx id
//   | Const   (Int v)        -> VInt (v)
//   | Const   (Bool v)       -> VBool (v)
//   | Op2     (op2, e1, e2)  -> op2_eval op2 e1 e2
//   | If      (e1, e2, e3)   -> if vbool (eval' ctx e1) then eval' ctx e2
//                                                       else eval' ctx e3
//   | Let     (id, e1, e2)   -> eval' ((id, eval' ctx e1) :: ctx) e2
//   | Fun     (id, _, e2)    -> VClosure (ctx, id, e2)
//   | Fix     (id, _, e2)    -> let inner = eval' ctx e2 in
//                               let (_, iid, ie) = vclosure inner in
//                               let substituted_exp = subst id e ie in
//                               VClosure (ctx, iid, substituted_exp)
//   | App     (e1, e2)       -> let v = (eval' ctx e2) in
//                               let (ctx, id, e) = vclosure (eval' ctx e1) in
//                               eval' ((id, v) :: ctx) e
//   | Empty   (ty)           -> VEmpty ty
//   | Cons    (e1, e2)       -> VCons ((eval' ctx e1), (eval' ctx e2))
//   | Head    (Cons (hd, _)) -> eval' ctx hd
//   | Head    (_)            -> failwith "head on empty list"
//   | Tail    (Cons (_, tl)) -> eval' ctx tl
//   | Tail    (_)            -> failwith "tail on empty list"
//                     | IsEmpty (e)            -> is_empty (eval' ctx e)


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
        Var(ref x) => ctx.get(x).unwrap().clone(),
        Let(ref id, ref e1, ref e2) => {
            let v1 = eval(ctx, e1);
            let mut new_ctx = ctx.clone();
            new_ctx.insert(id.clone(), v1);
            eval(&new_ctx, e2)
        }
        Fun(ref id, _, ref e) => {
            VClosure(box ctx.clone(), id.clone(), e.clone())
        }
        Fix(ref id, _, ref e) => {
            let inner = eval(ctx, e);
            let (_, iid, ie) = vclosure(inner);
            let substituted_exp = box subst(ctx, id, e, &ie);
            VClosure(box ctx.clone(), iid, substituted_exp)
        }
        App(ref e1, ref e2) => {
            let v = eval(ctx, e2);
            let (ctx, id, e) = vclosure(eval(ctx, e1));
            let mut new_ctx = ctx.clone();
            new_ctx.insert(id, v);
            eval(&new_ctx, &e)
        }
        Empty(_) => VEmpty,
        Cons(ref e1, ref e2) => VCons(box eval(ctx, e1), box eval(ctx, e2)),
        Head(ref e) => {
            if let VCons(ref hd, _) = eval(ctx, e) {
                *hd.clone()
            } else {
                panic!("unreachable - head w/o cons: {:?}", e)
            }
        }
        Tail(ref e) => {
            if let VCons(_, ref tl) = eval(ctx, e) {
                *tl.clone()
            } else {
                panic!("unreachable - tail w/o cons")
            }
        }
        IsEmpty(ref e) => {
            is_empty(&eval(ctx, e))
        }
    }
}

pub fn interpret(expr: &Expr) -> Value {
    let ctx: Closure = HashMap::new();

    eval(&ctx, expr)
}
