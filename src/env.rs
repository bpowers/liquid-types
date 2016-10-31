use implicit;
use explicit;
use typed;

use common::Id;
use std::collections::HashMap;
use explicit::Type;

fn convert(n: i32, env: &mut HashMap<Id, Type>, renamed: &HashMap<Id, Id>, expr: &explicit::Expr) -> (i32, implicit::Expr, Type) {
    use common::Const;
    use implicit::Expr as I;
    use typed::Expr as E;
    match *expr {
        E::Const(Const::Int(c)) => (n, I::Const(Const::Int(c)), Type::TInt),
        E::Const(Const::Bool(c)) => (n, I::Const(Const::Bool(c)), Type::TBool),
        E::Op2(op, ref e1, ref e2) => {
            let (n, e1, _) = convert(n, env, renamed, e1);
            let (n, e2, _) = convert(n, env, renamed, e2);
            (n, I::Op2(op, box e1, box e2), explicit::opty(op))
        }
        E::If(ref e1, ref e2, ref e3) => {
            let (n, e1, _) = convert(n, env, renamed, e1);
            let (n, e2, t) = convert(n, env, renamed, e2);
            let (n, e3, _) = convert(n, env, renamed, e3);
            (n, I::If(box e1, box e2, box e3), t)
        }
        E::Var(ref x) => {
            let αx = renamed.get(x).unwrap();
            (n, I::Var(αx.clone()), env.get(αx).unwrap().clone())
        }
        E::Let(ref id, ref e1, ref e2) => {
            let mut renamed = renamed.clone();
            let (n, αid) = (n+1, format!("{}-α{}", id, n));

            let (n, e1, t1) = convert(n, env, &renamed, e1);

            renamed.insert(id.clone(), αid.clone());
            env.insert(αid.clone(), t1);

            let (n, e2, t2) = convert(n, env, &renamed, e2);
            (n, I::Let(αid, box e1, box e2), t2)
        }
        E::Fun(ref id, ref t1, ref e) => {
            let mut renamed = renamed.clone();
            let (n, αid) = (n+1, format!("{}-α{}", id, n));

            renamed.insert(id.clone(), αid.clone());
            env.insert(αid.clone(), t1.clone());

            let (n, e, t2) = convert(n, env, &renamed, e);

            (n, I::Fun(αid, box e), Type::TFun(box t1.clone(), box t2))
        }
        E::Fix(ref id, ref t1, ref e) => {
            let mut renamed = renamed.clone();
            let (n, αid) = (n+1, format!("{}-α{}", id, n));

            renamed.insert(id.clone(), αid.clone());
            env.insert(αid.clone(), t1.clone());

            let (n, e, t2) = convert(n, env, &renamed, e);

            (n, I::Fix(αid, box e), t2)
        }
        E::App(ref e1, ref e2) => {
            let (n, e1, t1) = convert(n, env, renamed, e1);
            let (n, e2, _) = convert(n, env, renamed, e2);
            let t = match t1 {
                Type::TFun(_, t2) => *t2,
                _ => panic!("expected TFun, not {:?}", t1),
            };
            (n, I::App(box e1, box e2), t)
        }
        E::MkArray(ref sz, ref val) => {
            let (n, sz, _) = convert(n, env, renamed, sz);
            let (n, val, _) = convert(n, env, renamed, val);
            (n, I::MkArray(box sz, box val), Type::TIntArray)
        }
        E::GetArray(ref id, ref idx) => {
            let (n, id, _) = convert(n, env, renamed, id);
            let (n, idx, _) = convert(n, env, renamed, idx);
            (n, I::GetArray(box id, box idx), Type::TInt)
        }
        E::SetArray(ref id, ref idx, ref v) => {
            let (n, id, _) = convert(n, env, renamed, id);
            let (n, idx, _) = convert(n, env, renamed, idx);
            let (n, v, _) = convert(n, env, renamed, v);
            (n, I::SetArray(box id, box idx, box v), Type::TIntArray)
        }
    }
}

pub fn extract(expr: &explicit::Expr) -> (implicit::Expr, HashMap<Id, Type>) {
    let mut env: HashMap<Id, Type> = HashMap::new();
    // keep track of α-renaming
    let mut renamed: HashMap<Id, Id> = HashMap::new();

    let (_, iexpr, _) = convert(1, &mut env, &renamed, expr);

    (iexpr, env)
}
