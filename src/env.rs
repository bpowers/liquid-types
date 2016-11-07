use explicit;

use common::Id;
use std::collections::HashMap;
use explicit::Type;

fn convert(n: i32, env: &mut HashMap<Id, Type>, renamed: &HashMap<Id, Id>, expr: &explicit::Expr) -> (i32, explicit::Expr, Type) {
    use common::Const;
    use typed::Expr as E;

    match *expr {
        E::Const(Const::Int(c)) => (n, E::Const(Const::Int(c)), Type::TInt),
        E::Const(Const::Bool(c)) => (n, E::Const(Const::Bool(c)), Type::TBool),
        E::Op2(op, ref e1, ref e2) => {
            let (n, e1, _) = convert(n, env, renamed, e1);
            let (n, e2, _) = convert(n, env, renamed, e2);
            (n, E::Op2(op, box e1, box e2), explicit::opty(op))
        }
        E::If(ref e1, ref e2, ref e3) => {
            let (n, e1, _) = convert(n, env, renamed, e1);
            let (n, e2, t) = convert(n, env, renamed, e2);
            let (n, e3, _) = convert(n, env, renamed, e3);
            (n, E::If(box e1, box e2, box e3), t)
        }
        E::Var(ref x) => {
            let alpha_x = renamed.get(x).unwrap();
            (n, E::Var(alpha_x.clone()), env.get(alpha_x).unwrap().clone())
        }
        E::Let(ref id, ref e1, ref e2) => {
            let mut renamed = renamed.clone();
            let (n, alpha_id) = (n+1, format!("{}!a{}", id, n));

            let (n, e1, t1) = convert(n, env, &renamed, e1);

            renamed.insert(id.clone(), alpha_id.clone());
            env.insert(alpha_id.clone(), t1);

            let (n, e2, t2) = convert(n, env, &renamed, e2);
            (n, E::Let(alpha_id, box e1, box e2), t2)
        }
        E::Fun(ref id, ref t1, ref e) => {
            let mut renamed = renamed.clone();
            let (n, alpha_id) = (n+1, format!("{}!a{}", id, n));

            renamed.insert(id.clone(), alpha_id.clone());
            env.insert(alpha_id.clone(), t1.clone());

            let (n, e, t2) = convert(n, env, &renamed, e);

            (n, E::Fun(alpha_id, t1.clone(), box e), Type::TFun(box t1.clone(), box t2))
        }
        E::Fix(ref id, ref t1, ref e) => {
            let mut renamed = renamed.clone();
            let (n, alpha_id) = (n+1, format!("{}!a{}", id, n));

            renamed.insert(id.clone(), alpha_id.clone());
            env.insert(alpha_id.clone(), t1.clone());

            let (n, e, t2) = convert(n, env, &renamed, e);

            (n, E::Fix(alpha_id, t1.clone(), box e), t2)
        }
        E::App(ref e1, ref e2) => {
            let (n, e1, t1) = convert(n, env, renamed, e1);
            let (n, e2, _) = convert(n, env, renamed, e2);
            let t = match t1 {
                Type::TFun(_, t2) => *t2,
                _ => panic!("expected TFun, not {:?}", t1),
            };
            (n, E::App(box e1, box e2), t)
        }
        E::MkArray(ref sz, ref val) => {
            let (n, sz, _) = convert(n, env, renamed, sz);
            let (n, val, _) = convert(n, env, renamed, val);
            (n, E::MkArray(box sz, box val), Type::TIntArray)
        }
        E::GetArray(ref id, ref idx) => {
            let (n, id, _) = convert(n, env, renamed, id);
            let (n, idx, _) = convert(n, env, renamed, idx);
            (n, E::GetArray(box id, box idx), Type::TInt)
        }
        E::SetArray(ref id, ref idx, ref v) => {
            let (n, id, _) = convert(n, env, renamed, id);
            let (n, idx, _) = convert(n, env, renamed, idx);
            let (n, v, _) = convert(n, env, renamed, v);
            (n, E::SetArray(box id, box idx, box v), Type::TIntArray)
        }
        E::V => (n, E::V, Type::TInt), // TODO: not quite right
        E::Star => (n, E::Star, Type::TInt), // TODO: not quite right
    }
}

pub fn extract(expr: &explicit::Expr) -> (explicit::Expr, HashMap<Id, Type>) {
    let mut env: HashMap<Id, Type> = HashMap::new();
    // keep track of α-renaming
    let renamed: HashMap<Id, Id> = HashMap::new();

    let (_, iexpr, _) = convert(1, &mut env, &renamed, expr);

    (iexpr, env)
}
