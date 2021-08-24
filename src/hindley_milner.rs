use std::collections::HashMap;

use crate::common::{Id, Result};
use crate::explicit::{self, Metavar, Type};
use crate::implicit;

#[cfg(test)]
use crate::implicit_parse;

#[cfg(test)]
use crate::tok::Tokenizer;

struct MVEnv<'a> {
    env_id: &'a str,
    next_id: i32,
}

impl<'a> MVEnv<'a> {
    fn new(env: &'a str) -> MVEnv<'a> {
        MVEnv {
            env_id: env,
            next_id: 0,
        }
    }

    fn alloc(&mut self, s: &str) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        Type::TMetavar((id, s.to_string()))
    }

    fn alloc_empty(&mut self) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        Type::TMetavar((id, String::from(self.env_id)))
    }
}

fn add_metavars_in(env: &mut MVEnv, expr: &implicit::Expr) -> explicit::Expr {
    use crate::implicit::Expr as I;
    use crate::typed::Expr as E;

    match expr {
        I::Var(id) => E::Var(id.clone()),
        I::Const(c) => E::Const(*c),
        I::Op2(op, l, r) => {
            let el = Box::new(add_metavars_in(env, l));
            let er = Box::new(add_metavars_in(env, r));
            E::Op2(*op, el, er)
        }
        I::Fun(id, e) => {
            let ee = Box::new(add_metavars_in(env, e));
            let mv = env.alloc(id);
            E::Fun(id.clone(), mv, ee)
        }
        I::App(e1, e2) => {
            let ee1 = Box::new(add_metavars_in(env, e1));
            let ee2 = Box::new(add_metavars_in(env, e2));
            E::App(ee1, ee2)
        }
        I::If(e1, e2, e3) => {
            let ee1 = Box::new(add_metavars_in(env, e1));
            let ee2 = Box::new(add_metavars_in(env, e2));
            let ee3 = Box::new(add_metavars_in(env, e3));
            E::If(ee1, ee2, ee3)
        }
        I::Let(id, e1, e2) => {
            let ee1 = Box::new(add_metavars_in(env, e1));
            let ee2 = Box::new(add_metavars_in(env, e2));
            E::Let(id.clone(), ee1, ee2)
        }
        I::Fix(id, e) => {
            let ee = Box::new(add_metavars_in(env, e));
            let mv = env.alloc(id);
            E::Fix(id.clone(), mv, ee)
        }
        I::MkArray(sz, n) => {
            let sz = Box::new(add_metavars_in(env, sz));
            let n = Box::new(add_metavars_in(env, n));
            E::MkArray(sz, n)
        }
        I::GetArray(id, idx) => {
            let id = Box::new(add_metavars_in(env, id));
            let idx = Box::new(add_metavars_in(env, idx));
            E::GetArray(id, idx)
        }
        I::SetArray(id, idx, v) => {
            let id = Box::new(add_metavars_in(env, id));
            let idx = Box::new(add_metavars_in(env, idx));
            let v = Box::new(add_metavars_in(env, v));
            E::SetArray(id, idx, v)
        }
        I::Star => E::Star,
        I::V => E::V,
    }
}

pub fn add_metavars(expr: &implicit::Expr) -> explicit::Expr {
    let mut env = MVEnv::new("α");
    add_metavars_in(&mut env, expr)
}

fn gen_constraints<'a>(
    m: &mut MVEnv,
    env: &HashMap<Id, Type>,
    expr: &'a explicit::Expr,
) -> Result<(Vec<(Type, Type)>, Type)> {
    use crate::common::{Const, Op2};
    use crate::typed::Expr as E;

    let result = match expr {
        E::Const(Const::Int(_)) => (Vec::new(), Type::TInt),
        E::Const(Const::Bool(_)) => (Vec::new(), Type::TBool),
        E::Op2(op, e1, e2) => {
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            c1.append(&mut c2);
            let expected = match op {
                Op2::LT
                | Op2::LTE
                | Op2::GT
                | Op2::GTE
                | Op2::Eq
                | Op2::Add
                | Op2::Sub
                | Op2::Mul => Type::TInt,
                Op2::And | Op2::Or | Op2::Impl | Op2::Iff => Type::TBool,
            };
            c1.push((t1, expected.clone()));
            c1.push((t2, expected));
            (c1, explicit::opty(*op))
        }
        E::If(e1, e2, e3) => {
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            let (mut c3, t3) = gen_constraints(m, env, e3)?;
            c1.append(&mut c2);
            c1.append(&mut c3);
            c1.push((t1, Type::TBool));
            c1.push((t2, t3.clone()));
            (c1, t3)
        }
        E::Var(x) => match env.get(x) {
            Some(ty) => (Vec::new(), ty.clone()),
            None => {
                return err!("unbound identifier: {} in {:?}", x, env);
            }
        },
        E::Let(id, e1, e2) => {
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let mut new_env = env.clone();
            new_env.insert(id.clone(), t1);
            let (mut c2, t2) = gen_constraints(m, &new_env, e2)?;
            c1.append(&mut c2);
            (c1, t2)
        }
        E::Fun(id, t1, e) => {
            let mut new_env = env.clone();
            new_env.insert(id.clone(), t1.clone());
            let (c, t2) = gen_constraints(m, &new_env, e)?;
            (
                c,
                Type::TFun(id.clone(), Box::new(t1.clone()), Box::new(t2)),
            )
        }
        E::Fix(id, t1, e) => {
            let mut new_env = env.clone();
            new_env.insert(id.clone(), t1.clone());
            let (mut c, t2) = gen_constraints(m, &new_env, e)?;
            c.push((t1.clone(), t2));
            (c, t1.clone())
        }
        E::App(e1, e2) => {
            let mv = m.alloc_empty();
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            let id = match &**e1 {
                E::Fun(id, _, _) => id.clone(),
                E::Var(id) => {
                    if let Some(Type::TFun(xid, _, _)) = env.get(id) {
                        xid.clone()
                    } else {
                        // this is a problem, unless we're only
                        // running this to process Q, in which case it
                        // doesn't matter.
                        String::from("gonna-fail2")
                    }
                }
                _ => String::from("gonna-fail"),
            };
            c1.append(&mut c2);
            c1.push((t1, Type::TFun(id, Box::new(t2), Box::new(mv.clone()))));
            (c1, mv)
        }
        E::MkArray(sz, n) => {
            let (mut c1, t1) = gen_constraints(m, env, sz)?;
            let (mut c2, t2) = gen_constraints(m, env, n)?;
            c1.append(&mut c2);
            c1.push((t1, Type::TInt));
            c1.push((t2, Type::TInt));
            (c1, Type::TIntArray)
        }
        E::GetArray(id, idx) => {
            let (mut c1, t1) = gen_constraints(m, env, id)?;
            let (mut c2, t2) = gen_constraints(m, env, idx)?;
            c1.append(&mut c2);
            c1.push((t1, Type::TIntArray));
            c1.push((t2, Type::TInt));
            (c1, Type::TInt)
        }
        E::SetArray(id, idx, v) => {
            let (mut c1, t1) = gen_constraints(m, env, id)?;
            let (mut c2, t2) = gen_constraints(m, env, idx)?;
            let (mut c3, t3) = gen_constraints(m, env, v)?;
            c1.append(&mut c2);
            c1.append(&mut c3);
            c1.push((t1, Type::TIntArray));
            c1.push((t2, Type::TInt));
            c1.push((t3, Type::TInt));
            (c1, Type::TIntArray)
        }
        E::V => (Vec::new(), Type::TInt), // TODO: not quite right
        E::Star => (Vec::new(), Type::TInt), // TODO: not quite right
    };

    Ok(result)
}

fn apply(env: &HashMap<Metavar, Type>, typ_in: &Type) -> Type {
    use explicit::Type::*;

    match typ_in {
        TMetavar(mv) => {
            if let Some(styp) = env.get(mv) {
                // println!("$$\t  found {:?}", mv);
                styp.clone()
            } else {
                // println!("@@\t   notf {:?}", mv);
                typ_in.clone()
            }
        }
        TFun(id, a, b) => TFun(id.clone(), Box::new(apply(env, a)), Box::new(apply(env, b))),
        TIntArray | TInt | TBool => typ_in.clone(),
    }
}

fn compose(env1: &HashMap<Metavar, Type>, env2: &HashMap<Metavar, Type>) -> HashMap<Metavar, Type> {
    let mut r = HashMap::new();

    for (mv, typ) in env1.iter() {
        if env2.contains_key(mv) {
            if let Some(Type::TMetavar(mv2)) = env2.get(mv) {
                r.insert(mv2.clone(), typ.clone());
            }
        }
    }

    for (mv, typ) in env2.iter() {
        let typ = apply(env1, &apply(&r, typ));
        r.insert(mv.clone(), typ);
    }

    for (mv, typ) in env1.iter() {
        let typ = apply(&r, typ);
        r.insert(mv.clone(), typ);
    }

    r
}

fn occurs(mv: &Metavar, t: &Type) -> bool {
    use explicit::Type::*;

    match t {
        TMetavar(mv2) => mv == mv2,
        TFun(_, a, b) => occurs(mv, &a) || occurs(mv, &b),
        TIntArray | TInt | TBool => false,
    }
}

fn unify(t1: &Type, t2: &Type) -> Result<HashMap<Metavar, Type>> {
    use explicit::Type::*;

    let mut env: HashMap<Metavar, Type> = HashMap::new();

    match (t1, t2) {
        (TInt, TInt) => {}
        (TBool, TBool) => {}
        (TIntArray, TIntArray) => {}
        (TMetavar(mv1), TMetavar(mv2)) if mv1 == mv2 => {}
        (TMetavar(mv1), TMetavar(_)) => {
            env.insert(mv1.clone(), t2.clone());
        }
        (TMetavar(mv1), _) if !occurs(mv1, t2) => {
            env.insert(mv1.clone(), t2.clone());
        }
        (TMetavar(mv1), _) => return err!("occurs check failed for mv1 '{:?}' in '{:?}'", mv1, t2),
        (_, TMetavar(mv2)) if !occurs(mv2, t1) => {
            env.insert(mv2.clone(), t1.clone());
        }
        (_, TMetavar(mv2)) => return err!("occurs check failed for mv2 '{:?}' in '{:?}'", mv2, t1),
        (TFun(_, s1, s2), TFun(_, t1, t2)) => {
            let pi = unify(s1, t1)?;
            let s2 = apply(&pi, s2);
            let t2 = apply(&pi, t2);
            env = compose(&unify(&s2, &t2)?, &pi);
        }
        _ => return err!("couldn't unify {:?} with {:?}", t1, t2),
    };

    Ok(env)
}

fn unify_all(constraints: &[(Type, Type)]) -> Result<HashMap<Metavar, Type>> {
    let r: HashMap<Metavar, Type> = if let Some(((t1, t2), rest)) = constraints.split_first() {
        let rst = unify_all(rest)?;
        let fst = unify(t1, t2)?;
        compose(&fst, &rst)
    } else {
        HashMap::new()
    };

    Ok(r)
}

fn remove_metavars(
    env: &HashMap<Metavar, Type>,
    expr: &explicit::Expr,
) -> Result<Box<explicit::Expr>> {
    use crate::typed::Expr as E;

    let result: explicit::Expr = match expr {
        E::Const(c) => E::Const(*c),
        E::Op2(op, e1, e2) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            E::Op2(*op, e1, e2)
        }
        E::If(e1, e2, e3) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            let e3 = remove_metavars(env, e3)?;
            E::If(e1, e2, e3)
        }
        E::Var(x) => E::Var(x.clone()),
        E::Let(id, e1, e2) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            E::Let(id.clone(), e1, e2)
        }
        E::Fun(id, t1, e) => {
            let e = remove_metavars(env, e)?;
            E::Fun(id.clone(), apply(env, t1), e)
        }
        E::Fix(id, t1, e) => {
            let e = remove_metavars(env, e)?;
            E::Fix(id.clone(), apply(env, t1), e)
        }
        E::App(e1, e2) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            E::App(e1, e2)
        }
        E::MkArray(sz, n) => {
            let sz = remove_metavars(env, sz)?;
            let n = remove_metavars(env, n)?;
            E::MkArray(sz, n)
        }
        E::GetArray(id, idx) => {
            let id = remove_metavars(env, id)?;
            let idx = remove_metavars(env, idx)?;
            E::GetArray(id, idx)
        }
        E::SetArray(id, idx, v) => {
            let id = remove_metavars(env, id)?;
            let idx = remove_metavars(env, idx)?;
            let v = remove_metavars(env, v)?;
            E::SetArray(id, idx, v)
        }
        E::V => E::V,
        E::Star => E::Star,
    };

    Ok(Box::new(result))
}

pub fn infer_in(id_env: HashMap<Id, Type>, expr: &implicit::Expr) -> Result<explicit::Expr> {
    let typed_w_metavars = add_metavars(&expr);
    let mut cg_env = MVEnv::new("β");

    let constraints = gen_constraints(&mut cg_env, &id_env, &typed_w_metavars).map(|(constraints, _)| constraints)?;
    let mv_env = unify_all(&constraints)?;
    let typed = remove_metavars(&mv_env, &typed_w_metavars)?;

    Ok(*typed)
}

pub fn infer(expr: &implicit::Expr) -> Result<explicit::Expr> {
    let id_env = HashMap::new();
    infer_in(id_env, expr)
}

#[cfg(test)]
macro_rules! test_parse(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        if let Err(e) = implicit_parse::ProgramParser::new().parse(&s, tokenizer) {
            die!("parse_Program({}): {:?}", $s, e)
        }
    } }
);

#[cfg(test)]
macro_rules! test_parse_fails(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        if let Ok(_) = implicit_parse::ProgramParser::new().parse(&s, tokenizer) {
            die!("parse_Program({}) should have failed", $s)
        }
    } }
);

#[cfg(test)]
macro_rules! test_unbound_ident(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        let exp = match implicit_parse::ProgramParser::new().parse(&s, tokenizer) {
            Ok(v) => v,
            Err(e) => die!("parse_Program: {:?}", e),
        };
        let typed_w_metavars = add_metavars(&exp);
        let mut cg_env = MVEnv::new("β");
        let mut id_env = HashMap::new();
        if let Ok(_) = gen_constraints(&mut cg_env, &mut id_env, &typed_w_metavars) {
            die!("expected gen_constraints to fail for: {}", $s)
        }
    } }
);

#[test]
fn implicit_parse() {
    test_parse!("-22");
    test_parse!("(22)");
    test_parse!("((((-22))))");
    test_parse!("((((-22))))");

    // test_parse!("1 :: 2 :: empty");
    // test_parse!("true :: false :: empty");
    // test_parse!("true :: true :: empty");
    test_parse!("(2 + 3 - 2) + 3");
    test_parse!("fun x -> x + 1");
    test_parse!("let inc = fun x -> x + 1 in inc 3");
    test_parse!(
        "let factorial = fix fac -> fun n -> if n = 0 then 1 else n * (fac (n - 1)) in factorial 5"
    );
    test_parse!("let b = true in if b then true else false");
    test_parse!("let double = fun x -> x * 2 in double 3");
    // test_parse!("let h = (fun l -> head l) in let l = 1 :: 2 :: empty in (h l) :: l");

    test_parse_fails!("((22)");
    test_parse_fails!("let x = 1"); // only let-in supported

    test_unbound_ident!("let i = 3 in z");
}

#[test]
fn unification() {
    use explicit::Metavar;
    use explicit::Type::*;

    let x: Metavar = (0, String::from("x"));
    let y: Metavar = (1, String::from("y"));

    let mut s = HashMap::new();
    s.insert(x.clone(), TInt);
    match apply(&s, &TMetavar(x.clone())) {
        TInt => {}
        _ => die!("apply should replace x with TInt"),
    }

    match apply(
        &s,
        &TFun(
            String::from("_"),
            Box::new(TMetavar(x.clone())),
            Box::new(TBool),
        ),
    ) {
        TFun(_, l, r) if *l == TInt && *r == TBool => {}
        _ => die!("apply should recur into type constructors"),
    }

    // TEST "Subst.compose should distribute over Subst.apply (1)" =
    let mut s1 = HashMap::new();
    let mut s2 = HashMap::new();
    s1.insert(x.clone(), TInt);
    s2.insert(y.clone(), TBool);
    let app1 = apply(
        &compose(&s1, &s2),
        &TFun(
            String::from("_"),
            Box::new(TMetavar(x.clone())),
            Box::new(TMetavar(y.clone())),
        ),
    );
    let app2 = apply(
        &s1,
        &apply(
            &s2,
            &TFun(
                String::from("_"),
                Box::new(TMetavar(x.clone())),
                Box::new(TMetavar(y.clone())),
            ),
        ),
    );
    if app1 != app2 {
        die!("expected equal: {:?} == {:?}", app1, app2)
    }

    // TEST "Subst.compose should distribute over Subst.apply (2)" =
    s1.insert(x.clone(), TBool);
    s2.insert(y.clone(), TMetavar(x.clone()));
    let app1 = apply(
        &compose(&s1, &s2),
        &TFun(
            String::from("___"),
            Box::new(TMetavar(x.clone())),
            Box::new(TMetavar(y.clone())),
        ),
    );
    let app2 = apply(
        &s1,
        &apply(
            &s2,
            &TFun(
                String::from("___"),
                Box::new(TMetavar(x.clone())),
                Box::new(TMetavar(y.clone())),
            ),
        ),
    );
    if app1 != app2 {
        die!("expected equal2: {:?} == {:?}", app1, app2)
    }

    // (* An incomplete suite of tests for unification *)
    // TEST "unifying identical base types should return the empty substitution" =
    //   Subst.to_list (unify TInt TInt) = []
}
