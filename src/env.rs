use implicit;
use explicit;
use typed;

use std::collections::HashMap;
use explicit::Type;

fn convert(n: i32, env: &mut HashMap<String, Type>, renamed: &HashMap<String, String>, expr: &explicit::Expr) -> (i32, implicit::Expr, Type) {
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
        E::Empty(ref t1) => (n, I::Empty, t1.clone()),
        // E::Cons(ref e1, ref e2) => {
        //     let mv = m.alloc_empty();
        //     let (mut c1, t1) = gen_constraints(m, env, e1)?;
        //     let (mut c2, t2) = gen_constraints(m, env, e2)?;
        //     c1.append(&mut c2);
        //     c1.push((t1, mv.clone()));
        //     c1.push((t2, Type::TList(box mv.clone())));
        //     (c1, Type::TList(box mv))
        // }
        // E::Head(ref e) => {
        //     let mv = m.alloc_empty();
        //     let (mut c, t) = gen_constraints(m, env, e)?;
        //     c.push((t, Type::TList(box mv.clone())));
        //     (c, mv)
        // }
        // E::Tail(ref e) => {
        //     let mv = m.alloc_empty();
        //     let (mut c, t) = gen_constraints(m, env, e)?;
        //     c.push((t, Type::TList(box mv.clone())));
        //     (c, Type::TList(box mv))
        // }
        // E::IsEmpty(ref e) => {
        //     let mv = m.alloc_empty();
        //     let (mut c, t) = gen_constraints(m, env, e)?;
        //     c.push((t, Type::TList(box mv.clone())));
        //     (c, Type::TBool)
        // }
        // E::MkArray(ref sz, ref n) => {
        //     let (mut c1, t1) = gen_constraints(m, env, sz)?;
        //     let (mut c2, t2) = gen_constraints(m, env, n)?;
        //     c1.append(&mut c2);
        //     c1.push((t1, Type::TInt));
        //     c1.push((t2, Type::TInt));
        //     (c1, Type::TIntArray)
        // }
        // E::GetArray(ref id, ref idx) => {
        //     let (mut c1, t1) = gen_constraints(m, env, id)?;
        //     let (mut c2, t2) = gen_constraints(m, env, idx)?;
        //     c1.append(&mut c2);
        //     c1.push((t1, Type::TIntArray));
        //     c1.push((t2, Type::TInt));
        //     (c1, Type::TInt)
        // }
        // E::SetArray(ref id, ref idx, ref v) => {
        //     let (mut c1, t1) = gen_constraints(m, env, id)?;
        //     let (mut c2, t2) = gen_constraints(m, env, idx)?;
        //     let (mut c3, t3) = gen_constraints(m, env, v)?;
        //     c1.append(&mut c2);
        //     c1.append(&mut c3);
        //     c1.push((t1, Type::TIntArray));
        //     c1.push((t2, Type::TInt));
        //     c1.push((t3, Type::TInt));
        //     (c1, Type::TIntArray)
        // }
        E::Star => panic!("star found when it shouldn't be"),
        E::V => panic!("v found when it shouldn't be"),
        E::WellFormed(_) => panic!("wellformed found when it shouldn't be"),
        _ => panic!("! fixme"),
    }
}

pub fn extract(expr: &explicit::Expr) -> (implicit::Expr, HashMap<String, Type>) {
    let mut env: HashMap<String, Type> = HashMap::new();
    // keep track of α-renaming
    let mut renamed: HashMap<String, String> = HashMap::new();

    let (_, iexpr, _) = convert(1, &mut env, &renamed, expr);

    (iexpr, env)
}
