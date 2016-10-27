use std;
use std::collections::HashMap;
use std::error;

use implicit;
use explicit;

use explicit::{Type, Metavar};


#[cfg(test)]
use implicit_parse;

#[cfg(test)]
use tok::Tokenizer;

use common::{LiquidError, Result};

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

    fn alloc(&mut self, s: &String) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        Type::TMetavar((id, s.clone()))
    }

    fn alloc_empty(&mut self) -> Type {
        let id = self.next_id;
        self.next_id += 1;
        Type::TMetavar((id, String::from(self.env_id)))
    }
}

fn add_metavars_in(env: &mut MVEnv, exp: &implicit::Expr) -> explicit::Expr {
    use implicit::Expr as I;
    use explicit::Expr as E;

    match *exp {
        I::Var(ref id) => E::Var(id.clone()),
        I::Const(ref c) => E::Const(*c),
        I::Op2(ref op, ref l, ref r) => {
            let el = box add_metavars_in(env, l);
            let er = box add_metavars_in(env, r);
            E::Op2(*op, el, er)
        }
        I::Fun(ref id, ref e) => {
            let ee = box add_metavars_in(env, e);
            let mv = env.alloc(id);
            E::Fun(id.clone(), mv, ee)
        }
        I::App(ref e1, ref e2) => {
            let ee1 = box add_metavars_in(env, e1);
            let ee2 = box add_metavars_in(env, e2);
            E::App(ee1, ee2)
        }
        I::If(ref e1, ref e2, ref e3) => {
            let ee1 = box add_metavars_in(env, e1);
            let ee2 = box add_metavars_in(env, e2);
            let ee3 = box add_metavars_in(env, e3);
            E::If(ee1, ee2, ee3)
        }
        I::Let(ref id, ref e1, ref e2) => {
            let ee1 = box add_metavars_in(env, e1);
            let ee2 = box add_metavars_in(env, e2);
            E::Let(id.clone(), ee1, ee2)
        }
        I::Fix(ref id, ref e) => {
            let ee = box add_metavars_in(env, e);
            let mv = env.alloc(id);
            E::Fix(id.clone(), mv, ee)
        }
        I::Empty => E::Empty(env.alloc_empty()),
        I::Cons(ref hd, ref tl) => {
            let ehd = box add_metavars_in(env, hd);
            let etl = box add_metavars_in(env, tl);
            E::Cons(ehd, etl)
        }
        I::Head(ref e) => E::Head(box add_metavars_in(env, e)),
        I::Tail(ref e) => E::Tail(box add_metavars_in(env, e)),
        I::IsEmpty(ref e) => E::IsEmpty(box add_metavars_in(env, e)),
    }
}

fn add_metavars(exp: &implicit::Expr) -> Box<explicit::Expr> {
    let mut env = MVEnv::new("α");
    box add_metavars_in(&mut env, exp)
}

fn gen_constraints<'a>(m: &mut MVEnv,
                       env: &mut HashMap<&'a str, Type>,
                       exp: &'a explicit::Expr)
                       -> Result<(Vec<(Type, Type)>, Type)> {
    use common::{Const, Op2};
    use explicit::Expr as E;

    let result = match *exp {
        E::Const(Const::Int(_)) => (Vec::new(), Type::TInt),
        E::Const(Const::Bool(_)) => (Vec::new(), Type::TBool),
        E::Op2(op, ref e1, ref e2) => {
            let opty = match op {
                Op2::LT | Op2::GT | Op2::Eq => Type::TBool,
                Op2::Add | Op2::Sub | Op2::Mul => Type::TInt,
            };
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            c1.append(&mut c2);
            c1.push((t1, Type::TInt));
            c1.push((t2, Type::TInt));
            (c1, opty)
        }
        E::If(ref e1, ref e2, ref e3) => {
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            let (mut c3, t3) = gen_constraints(m, env, e3)?;
            c1.append(&mut c2);
            c1.append(&mut c3);
            c1.push((t1, Type::TBool));
            c1.push((t2, t3.clone()));
            (c1, t3)
        }
        E::Var(ref x) => {
            match env.get::<str>(&x) {
                Some(ty) => (Vec::new(), ty.clone()),
                None => {
                    return err!("unbound identifier: {}", x);
                }
            }
        }
        E::Let(ref id, ref e1, ref e2) => {
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            env.insert(&id, t1.clone());
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            env.remove::<str>(&id);
            c1.append(&mut c2);
            (c1, t2)
        }
        E::Fun(ref id, ref t1, ref e) => {
            env.insert(&id, t1.clone());
            let (c, t2) = gen_constraints(m, env, e)?;
            env.remove::<str>(&id);
            (c, Type::TFun(box t1.clone(), box t2))
        }
        E::Fix(ref id, ref t1, ref e) => {
            env.insert(&id, t1.clone());
            let (mut c, t2) = gen_constraints(m, env, e)?;
            env.remove::<str>(&id);
            c.push((t1.clone(), t2));
            (c, t1.clone())
        }
        E::App(ref e1, ref e2) => {
            let mv = m.alloc_empty();
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            c1.append(&mut c2);
            c1.push((t1.clone(), Type::TFun(box t2.clone(), box mv.clone())));
            (c1, mv)
        }
        E::Empty(ref t1) => (Vec::new(), Type::TList(box t1.clone())),
        E::Cons(ref e1, ref e2) => {
            let mv = m.alloc_empty();
            let (mut c1, t1) = gen_constraints(m, env, e1)?;
            let (mut c2, t2) = gen_constraints(m, env, e2)?;
            c1.append(&mut c2);
            c1.push((t1, mv.clone()));
            c1.push((t2, Type::TList(box mv.clone())));
            (c1, Type::TList(box mv))
        }
        E::Head(ref e) => {
            let mv = m.alloc_empty();
            let (mut c, t) = gen_constraints(m, env, e)?;
            c.push((t, Type::TList(box mv.clone())));
            (c, mv)
        }
        E::Tail(ref e) => {
            let mv = m.alloc_empty();
            let (mut c, t) = gen_constraints(m, env, e)?;
            c.push((t, Type::TList(box mv.clone())));
            (c, Type::TList(box mv))
        }
        E::IsEmpty(ref e) => {
            let mv = m.alloc_empty();
            let (mut c, t) = gen_constraints(m, env, e)?;
            c.push((t, Type::TList(box mv.clone())));
            (c, Type::TBool)
        }
    };

    Ok(result)
}

fn apply(env: &HashMap<Metavar, Type>, typ_in: &Type) -> Type {
    use explicit::Type::*;

    match *typ_in {
        TMetavar(ref mv) => {
            if let Some(styp) = env.get(mv) {
                // println!("$$\t  found {:?}", mv);
                styp.clone()
            } else {
                // println!("@@\t   notf {:?}", mv);
                typ_in.clone()
            }
        }
        TFun(ref a, ref b) => TFun(box apply(env, a), box apply(env, b)),
        TList(ref a) => TList(box apply(env, a)),
        TInt | TBool => typ_in.clone(),
    }
}

fn compose(env1: &HashMap<Metavar, Type>, env2: &HashMap<Metavar, Type>) -> HashMap<Metavar, Type> {
    let mut r = HashMap::new();

    for (mv, typ) in env1.iter() {
        if env2.contains_key(mv) {
            if let Some(&Type::TMetavar(ref mv2)) = env2.get(mv) {
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
        &TMetavar(ref mv2) => mv == mv2,
        &TFun(ref a, ref b) => occurs(mv, a) || occurs(mv, b),
        &TList(ref a) => occurs(mv, a),
        &TInt | &TBool => false,
    }
}

fn unify(t1: &Type, t2: &Type) -> Result<HashMap<Metavar, Type>> {
    use explicit::Type::*;

    let mut env: HashMap<Metavar, Type> = HashMap::new();

    match (t1, t2) {
        (&TInt, &TInt) => {}
        (&TBool, &TBool) => {}
        (&TMetavar(ref mv1), &TMetavar(ref mv2)) if mv1 == mv2 => {}
        (&TMetavar(ref mv1), &TMetavar(_)) => {
            env.insert(mv1.clone(), t2.clone());
        }
        (&TMetavar(ref mv1), _) if !occurs(mv1, t2) => {
            env.insert(mv1.clone(), t2.clone());
        }
        (&TMetavar(ref mv1), _) => {
            return err!("occurs check failed for mv1 '{:?}' in '{:?}'", mv1, t2)
        }
        (_, &TMetavar(ref mv2)) if !occurs(mv2, t1) => {
            env.insert(mv2.clone(), t1.clone());
        }
        (_, &TMetavar(ref mv2)) => {
            return err!("occurs check failed for mv2 '{:?}' in '{:?}'", mv2, t1)
        }
        (&TFun(ref s1, ref s2), &TFun(ref t1, ref t2)) => {
            let pi = unify(s1, t1)?;
            let s2 = apply(&pi, s2);
            let t2 = apply(&pi, t2);
            env = compose(&unify(&s2, &t2)?, &pi);
        }
        (&TList(ref s), &TList(ref t)) => {
            env = unify(s, t)?;
        }
        _ => return err!("couldn't unify {:?} with {:?}", t1, t2),
    };

    Ok(env)
}

fn unify_all(constraints: &[(Type, Type)]) -> Result<HashMap<Metavar, Type>> {
    let r: HashMap<Metavar, Type> = if let Some((&(ref t1, ref t2), rest)) =
        constraints.split_first() {
        let rst = unify_all(rest)?;
        let fst = unify(t1, t2)?;
        compose(&fst, &rst)
    } else {
        HashMap::new()
    };

    Ok(r)
}

fn remove_metavars(env: &HashMap<Metavar, Type>, exp: &explicit::Expr) -> Result<Box<explicit::Expr>> {
    use explicit::Expr as E;

    let result: E = match *exp {
        E::Const(ref c) => E::Const(c.clone()),
        E::Op2(op, ref e1, ref e2) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            E::Op2(op.clone(), e1, e2)
        }
        E::If(ref e1, ref e2, ref e3) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            let e3 = remove_metavars(env, e3)?;
            E::If(e1, e2, e3)
        }
        E::Var(ref x) => E::Var(x.clone()),
        E::Let(ref id, ref e1, ref e2) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            E::Let(id.clone(), e1, e2)
        }
        E::Fun(ref id, ref t1, ref e) => {
            let e = remove_metavars(env, e)?;
            E::Fun(id.clone(), apply(env, t1), e)
        }
        E::Fix(ref id, ref t1, ref e) => {
            let e = remove_metavars(env, e)?;
            E::Fix(id.clone(), apply(env, t1), e)
        }
        E::App(ref e1, ref e2) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            E::App(e1, e2)
        }
        E::Empty(ref t1) => E::Empty(apply(env, t1)),
        E::Cons(ref e1, ref e2) => {
            let e1 = remove_metavars(env, e1)?;
            let e2 = remove_metavars(env, e2)?;
            E::Cons(e1, e2)
        }
        E::Head(ref e) => {
            let e = remove_metavars(env, e)?;
            E::Head(e)
        }
        E::Tail(ref e) => {
            let e = remove_metavars(env, e)?;
            E::Tail(e)
        }
        E::IsEmpty(ref e) => {
            let e = remove_metavars(env, e)?;
            E::IsEmpty(e)
        }
    };

    Ok(box result)
}

pub fn infer_types(exp: &implicit::Expr) -> Result<explicit::Expr> {
    let typed_w_metavars = add_metavars(&exp);
    let mut cg_env = MVEnv::new("β");
    let mut id_env = HashMap::new();
    let constraints = match gen_constraints(&mut cg_env, &mut id_env, &typed_w_metavars) {
        Ok((c, _)) => c,
        Err(e) => die!("gen_constraints: {}", error::Error::description(&e)),
    };

    let mv_env = match unify_all(&constraints) {
        Ok(e) => e,
        Err(e) => die!("unification failed: {}", error::Error::description(&e)),
    };

    let typed = match remove_metavars(&mv_env, &typed_w_metavars) {
        Ok(exp) => exp,
        Err(e) => die!("remove_metavars failed: {}", error::Error::description(&e)),
    };

    Ok(*typed)
}


macro_rules! test_parse(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        if let Err(e) = implicit_parse::parse_Program(&s, tokenizer) {
            die!("parse_Program({}): {:?}", $s, e)
        }
    } }
);

macro_rules! test_parse_fails(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        if let Ok(_) = implicit_parse::parse_Program(&s, tokenizer) {
            die!("parse_Program({}) should have failed", $s)
        }
    } }
);

macro_rules! test_unbound_ident(
    ($s:expr) => { {
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        let exp = match implicit_parse::parse_Program(&s, tokenizer) {
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

    test_parse!("1 :: 2 :: empty");
    test_parse!("true :: false :: empty");
    test_parse!("true :: true :: empty");
    test_parse!("(2 + 3 - 2) + 3");
    test_parse!("fun x -> x + 1");
    test_parse!("let inc = fun x -> x + 1 in inc 3");
    test_parse!("let factorial = fix fac -> fun n -> if n = 0 then 1 else n * (fac (n - 1)) in factorial 5");
    test_parse!("let b = true in if b then true else false");
    test_parse!("let double = fun x -> x * 2 in double 3");
    test_parse!("let h = (fun l -> head l) in let l = 1 :: 2 :: empty in (h l) :: l");

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

    match apply(&s, &TFun(box TMetavar(x.clone()), box TBool)) {
        TFun(box TInt, box TBool) => {}
        _ => die!("apply should recur into type constructors"),
    }

    // TEST "Subst.compose should distribute over Subst.apply (1)" =
    let mut s1 = HashMap::new();
    let mut s2 = HashMap::new();
    s1.insert(x.clone(), TInt);
    s2.insert(y.clone(), TBool);
    let app1 = apply(&compose(&s1, &s2),
                     &TFun(box TMetavar(x.clone()), box TMetavar(y.clone())));
    let app2 = apply(&s1,
                     &apply(&s2, &TFun(box TMetavar(x.clone()), box TMetavar(y.clone()))));
    if app1 != app2 {
        die!("expected equal: {:?} == {:?}", app1, app2)
    }

    // TEST "Subst.compose should distribute over Subst.apply (2)" =
    s1.insert(x.clone(), TBool);
    s2.insert(y.clone(), TMetavar(x.clone()));
    let app1 = apply(&compose(&s1, &s2),
                     &TFun(box TMetavar(x.clone()), box TMetavar(y.clone())));
    let app2 = apply(&s1,
                     &apply(&s2, &TFun(box TMetavar(x.clone()), box TMetavar(y.clone()))));
    if app1 != app2 {
        die!("expected equal2: {:?} == {:?}", app1, app2)
    }

    // (* An incomplete suite of tests for unification *)
    // TEST "unifying identical base types should return the empty substitution" =
    //   Subst.to_list (unify TInt TInt) = []

}
