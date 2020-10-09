use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fmt::{Debug, Error, Formatter};
use std::result;

use crate::common::{Id, Result};
use crate::explicit;
use crate::explicit::Type::TInt;
use crate::hindley_milner;
use crate::implicit;
use crate::implicit_parse::ProgramParser;
use crate::lambdal::{self, Expr, Imm, Op};
use crate::refined::{Base, T};
use crate::tok::Tokenizer;

use z3::ast::{Ast, Bool, Dynamic, Int};
use z3::{Config, Context, Solver};
// core, integer, LIA, LIA_Sorts

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum C {
    WellFormed(Box<Type>),
    Subtype(Box<Type>, Box<Type>),
}

#[derive(PartialEq, Eq, Clone)]
pub enum Liquid {
    E(Expr),
    K(Id, Box<LinkedList<(Imm, Id)>>), // list of pending substitutions [imm/id]
}

impl Debug for Liquid {
    fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), Error> {
        use self::Liquid::*;
        match self {
            E(expr) => write!(fmt, "{:?}", expr),
            K(id, subs) => write!(fmt, "{} {:?}", id, subs),
        }
    }
}

pub type Type = T<Liquid>;
// if we have a type T1 with two subtype constraints: T2 <: T1 and T3
// <: T1, that would be a single STConstraint with 2 antedents.
pub type STConstraints = (Vec<(HashSet<Id>, LinkedList<Expr>, Box<Type>)>, Id);

pub type Idx = i32; // constraint index

pub type Constraint = ((HashSet<Id>, LinkedList<Expr>), C); // Boolean valued expressions & their environments

#[derive(Debug, Clone)]
pub struct KInfo {
    base: Base,
    all_qs: HashSet<lambdal::Expr>,
    curr_qs: Vec<lambdal::Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KEnv {
    shape: HashMap<Id, explicit::Type>,
    refined: HashMap<Id, Type>,
    next_id: i32,
}

impl KEnv {
    fn new(shape: &HashMap<Id, explicit::Type>) -> KEnv {
        KEnv {
            shape: shape.clone(),
            refined: HashMap::new(),
            next_id: 0,
        }
    }

    fn fresh_ty(&mut self, ty: explicit::Type) -> Type {
        let id = self.next_id;
        self.next_id += 1;

        let base = match ty {
            explicit::Type::TInt => Base::Int,
            explicit::Type::TBool => Base::Bool,
            explicit::Type::TFun(id, a, b) => {
                let fx = self.fresh_ty(*a);
                let f = self.fresh_ty(*b);
                return T::Fun(id, Box::new(fx), Box::new(f));
            }
            _ => panic!("FIXME: handle {:?}", ty),
        };
        let kid = format!("_k{}", id);
        let k = Liquid::K(kid, Box::new(LinkedList::new()));
        T::Ref(self.in_scope(), base, Box::new(k))
    }

    fn fresh(&mut self, e: &Expr) -> Type {
        let hm_type = hm_shape_expr(&self.shape, e);
        self.fresh_ty(hm_type)
    }

    fn get(&self, s: &str) -> Type {
        match self.refined.get(s) {
            Some(ty) => ty.clone(),
            None => panic!("env.get('{}' missing ({:?})", s, self.in_scope()),
        }
    }

    fn insert(&mut self, id: &str, ty: &Type) {
        self.refined.insert(id.to_string(), ty.clone());
    }

    fn in_scope(&self) -> HashSet<Id> {
        let keys: HashSet<_> = self.refined.keys().cloned().collect();
        keys
    }
}

fn hm_shape_imm(env: &HashMap<Id, explicit::Type>, imm: &Imm) -> explicit::Type {
    use crate::explicit::Type::*;
    use crate::lambdal::Imm::*;

    match imm {
        Bool(_) => TBool,
        Int(_) => TInt,
        Var(id) => env[id].clone(),
        Fun(id, e) => {
            let argty = env[id].clone();
            TFun(id.clone(), Box::new(argty), Box::new(hm_shape_expr(env, e)))
        }
        Fix(id, _) => env[id].clone(),
        V => env["!v"].clone(),
        Star => unreachable!("shape of star"),
    }
}

fn hm_shape_op(env: &HashMap<Id, explicit::Type>, op: &Op) -> explicit::Type {
    use explicit::Type::*;
    use lambdal::Op::*;

    match op {
        Op2(op, _, _) => explicit::opty(*op),
        MkArray(_, _) => TIntArray,
        GetArray(_, _) => TInt,
        SetArray(_, _, _) => TIntArray,
        Imm(imm) => hm_shape_imm(env, imm),
    }
}

fn hm_shape_expr(env: &HashMap<Id, explicit::Type>, expr: &Expr) -> explicit::Type {
    use explicit::Type::TFun;
    use lambdal::Expr::*;

    match expr {
        If(_, e2, _) => hm_shape_expr(env, e2),
        Let(_, _, e2) => hm_shape_expr(env, e2),
        App(e1, _) => {
            if let TFun(_, _, rtype) = hm_shape_imm(env, e1) {
                *rtype
            } else {
                panic!("app for non-fun: {:?}", e1);
            }
        }
        Op(op) => hm_shape_op(env, op),
    }
}

fn ty(_: &mut KEnv, c: &Imm) -> Type {
    use self::Liquid::E;
    use crate::common::Op2;

    let base = match *c {
        Imm::Int(_) => Base::Int,
        Imm::Bool(_) => Base::Bool,
        _ => unreachable!("only called for constants."),
    };

    // something like {ν : int | ν = 3 }
    let eq = E(Expr::Op(Op::Op2(
        Op2::Eq,
        Box::new(Op::Imm(Imm::V)),
        Box::new(Op::Imm(c.clone())),
    )));
    T::Ref(HashSet::new(), base, Box::new(eq))
}

fn base(ty: &Type) -> Option<Base> {
    match *ty {
        T::Ref(_, b, _) => Some(b),
        _ => None,
    }
}

fn subst(sid: &str, imm: &Imm, ty: &Type) -> Type {
    if let T::Ref(scope, base, iref) = ty {
        if let Liquid::K(id, pending) = &**iref {
            let mut pending = pending.clone();
            pending.push_back((imm.clone(), sid.to_string()));
            T::Ref(
                scope.clone(),
                *base,
                Box::new(Liquid::K(id.clone(), pending)),
            )
        } else {
            panic!("Expected liquid type, not fun or full dep type: {:?}", ty);
        }
    } else if let T::Fun(id, tx, t) = ty {
        T::Fun(
            id.clone(),
            Box::new(subst(sid, imm, tx)),
            Box::new(subst(sid, imm, t)),
        )
    } else {
        panic!("Expected liquid type, not fun or full dep type: {:?}", ty);
    }
}

pub fn cons_imm(
    k_env: &mut KEnv,
    pathc: &LinkedList<Expr>,
    imm: &Imm,
) -> (Type, LinkedList<Constraint>) {
    use crate::common::Op2::Eq;
    use crate::lambdal::Imm::*;
    use crate::lambdal::Op::Imm as I;

    match imm {
        Var(id) => {
            let ty: Type = if let Some(b) = base(&k_env.get(id)) {
                let eq = Expr::Op(Op::Op2(Eq, Box::new(I(V)), Box::new(I(Var(id.clone())))));
                T::Ref(k_env.in_scope(), b, Box::new(Liquid::E(eq)))
            } else {
                k_env.get(id)
            };
            (ty, LinkedList::new())
        }
        Bool(_) => (ty(k_env, imm), LinkedList::new()),
        Int(_) => (ty(k_env, imm), LinkedList::new()),
        Fun(x, e) => {
            let fx = k_env.fresh(&Expr::Op(Op::Imm(Var(x.clone()))));
            let ffun_env = (k_env.in_scope(), pathc.clone());

            k_env.insert(x, &fx);
            let f = k_env.fresh(e);
            let (fe, mut c) = cons_expr(k_env, pathc, e);

            let ffun = T::Fun(x.clone(), Box::new(fx.clone()), Box::new(f.clone()));
            // Γ ⊢ (x:fx → f)
            c.push_back((ffun_env.clone(), C::WellFormed(Box::new(fx))));
            c.push_back((ffun_env.clone(), C::WellFormed(Box::new(ffun.clone()))));
            // Γ,x:fx ⊢ (fe <: f)
            c.push_back((ffun_env, C::Subtype(Box::new(fe), Box::new(f))));

            (ffun, c)
        }
        Fix(x, e) => {
            // const w/ ∀α.(α→α)→α
            let f = k_env.fresh(e);
            k_env.insert(x, &f);
            let ffix_env = (k_env.in_scope(), pathc.clone());

            let (fe, mut c) = cons_expr(k_env, &pathc, e);
            c.push_back((ffix_env, C::Subtype(Box::new(fe), Box::new(f.clone()))));

            (f, c)
        }
        _ => {
            panic!("TODO: cons_imm for {:?}", imm);
        }
    }
}

pub fn cons_op(
    k_env: &mut KEnv,
    pathc: &LinkedList<Expr>,
    e: &Op,
) -> (Type, LinkedList<Constraint>) {
    use crate::common::Op2::Eq;

    match e {
        Op::Imm(imm) => cons_imm(k_env, pathc, imm),
        Op::Op2(op, l, r) => {
            let (_, mut c1) = cons_op(k_env, pathc, l);
            let (_, mut c2) = cons_op(k_env, pathc, r);
            c1.append(&mut c2);

            let ty = explicit::opty(*op);
            let base = match ty {
                explicit::Type::TInt => Base::Int,
                explicit::Type::TBool => Base::Bool,
                _ => panic!("FIXME: handle {:?}", ty),
            };

            let eq = Expr::Op(Op::Op2(Eq, Box::new(Op::Imm(Imm::V)), Box::new(e.clone())));
            let f = T::Ref(k_env.in_scope(), base, Box::new(Liquid::E(eq)));

            (f, c1)
        }
        _ => {
            panic!("TODO: cons_op for {:?}", e);
        }
    }
}

pub fn cons_expr(
    k_env: &mut KEnv,
    pathc: &LinkedList<Expr>,
    expr: &Expr,
) -> (Type, LinkedList<Constraint>) {
    use lambdal::Expr::*;
    use lambdal::Op as LOp;

    match expr {
        If(e1, e2, e3) => {
            let mut pathc_t = pathc.clone();
            let mut pathc_f = pathc.clone();
            pathc_t.push_back(Expr::Op(LOp::Imm(*e1.clone())));
            pathc_f.push_back(Expr::App(
                Box::new(Imm::Var(String::from("not"))),
                (*e1).clone(),
            ));

            let f = k_env.fresh(expr);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons_imm(k_env, &pathc, e1);
            let (f2, mut c2) = cons_expr(k_env, &pathc_t, e2);
            let (f3, mut c3) = cons_expr(k_env, &pathc_f, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // Γ ⊢ (f)
            c1.push_back((
                (k_env.in_scope(), pathc.clone()),
                C::WellFormed(Box::new(f.clone())),
            ));
            // Γ,e1 ⊢ (f2 <: f)
            c1.push_back((
                (k_env.in_scope(), pathc_t),
                C::Subtype(Box::new(f2), Box::new(f.clone())),
            ));
            // Γ,¬e1 ⊢ (f3 <: f)
            c1.push_back((
                (k_env.in_scope(), pathc_f),
                C::Subtype(Box::new(f3), Box::new(f.clone())),
            ));

            (f, c1)
        }
        Let(id, e1, e2) => {
            let f = k_env.fresh(expr);
            let mut in_scope = k_env.in_scope();
            let (f1, mut c1) = cons_expr(k_env, pathc, e1);
            k_env.insert(id, &f1);
            in_scope.insert(id.clone());

            let let_env = (in_scope, pathc.clone());

            let (f2, mut c2) = cons_expr(k_env, pathc, e2);
            c1.append(&mut c2);
            // Γ ⊢ (f)
            c1.push_back((let_env.clone(), C::WellFormed(Box::new(f.clone()))));
            // Γ,x:f1 ⊢ (f2 <: f)
            c1.push_back((let_env, C::Subtype(Box::new(f2), Box::new(f.clone()))));

            (f, c1)
        }
        App(e1, e2) => {
            let (f1, mut c1) = cons_imm(k_env, pathc, e1);
            let (f2, mut c2) = cons_imm(k_env, pathc, e2);
            c1.append(&mut c2);
            if let T::Fun(x, fx, f) = &f1 {
                let f = subst(x, e2, f);
                // Γ ⊢ (f2 <: fx)
                c1.push_back(((HashSet::new(), pathc.clone()), C::WellFormed(fx.clone())));
                c1.push_back((
                    (k_env.in_scope(), pathc.clone()),
                    C::Subtype(Box::new(f2), fx.clone()),
                ));
                (f, c1)
            } else {
                panic!("expected TFun, not {:?}", f1);
            }
        }
        Op(op) => cons_op(k_env, &pathc, op),
    }
}

fn split(map: &mut HashMap<Idx, Constraint>, constraints: &LinkedList<Constraint>) {
    let mut idx = (map.len() as i32) + 1;

    for c in constraints.iter() {
        if let ((scope, pathc), C::Subtype(bt1, bt2)) = c {
            if let T::Fun(_, tx1, t1) = &**bt1 {
                if let T::Fun(x2, tx2, t2) = &**bt2 {
                    let mut contra_cs: LinkedList<Constraint> = LinkedList::new();
                    contra_cs.push_back((
                        (scope.clone(), pathc.clone()),
                        C::Subtype(tx2.clone(), tx1.clone()),
                    ));

                    let mut rscope = scope.clone();
                    rscope.insert(x2.clone());
                    contra_cs
                        .push_back(((rscope, pathc.clone()), C::Subtype(t1.clone(), t2.clone())));

                    // recurse
                    split(map, &contra_cs);
                    idx = (map.len() as i32) + 1;
                }
            }
        } else if let ((scope, pathc), C::WellFormed(bwf)) = c {
            if let T::Fun(id, _, t) = &**bwf {
                let mut wf_cs: LinkedList<Constraint> = LinkedList::new();

                let mut scope = scope.clone();
                scope.insert(id.clone());
                wf_cs.push_back(((scope, pathc.clone()), C::WellFormed(t.clone())));

                // recurse
                split(map, &wf_cs);
                idx = (map.len() as i32) + 1;
            }
        } else {
            map.insert(idx, c.clone());
            idx += 1;
        }
    }
}

fn replace_imm(imm: &Imm, from: &Imm, to: &Imm) -> Imm {
    if imm == from {
        return to.clone();
    }

    imm.clone()
}

fn replace_op(op: &Op, from: &Imm, to: &Imm) -> Op {
    use lambdal::Op::*;
    match op {
        Op2(op, l, r) => Op2(
            *op,
            Box::new(replace_op(l, from, to)),
            Box::new(replace_op(r, from, to)),
        ),
        MkArray(sz, n) => MkArray(
            Box::new(replace_imm(sz, from, to)),
            Box::new(replace_imm(n, from, to)),
        ),
        GetArray(a, i) => GetArray(
            Box::new(replace_imm(a, from, to)),
            Box::new(replace_imm(i, from, to)),
        ),
        SetArray(a, i, v) => SetArray(
            Box::new(replace_imm(a, from, to)),
            Box::new(replace_imm(i, from, to)),
            Box::new(replace_imm(v, from, to)),
        ),
        Imm(imm) => Imm(replace_imm(imm, from, to)),
    }
}

fn replace_expr(expr: &Expr, from: &Imm, to: &Imm) -> Expr {
    use lambdal::Expr::*;
    match expr {
        If(imm, e1, e2) => If(
            Box::new(replace_imm(imm, from, to)),
            Box::new(replace_expr(e1, from, to)),
            Box::new(replace_expr(e2, from, to)),
        ),
        Let(id, e1, e2) => Let(
            id.clone(),
            Box::new(replace_expr(e1, from, to)),
            Box::new(replace_expr(e2, from, to)),
        ),
        App(imm1, imm2) => App(
            Box::new(replace_imm(imm1, from, to)),
            Box::new(replace_imm(imm2, from, to)),
        ),
        Op(op) => Op(replace_op(op, from, to)),
    }
}

fn replace(v: &str, q: &implicit::Expr) -> Option<implicit::Expr> {
    use implicit::Expr as I;

    let r = match q {
        I::Var(id) => I::Var(id.clone()),
        I::Const(c) => I::Const(*c),
        I::Op2(op, l, r) => I::Op2(*op, Box::new(replace(v, l)?), Box::new(replace(v, r)?)),
        I::Fun(id, e) => I::Fun(id.clone(), Box::new(replace(v, e)?)),
        I::App(e1, e2) => I::App(Box::new(replace(v, e1)?), Box::new(replace(v, e2)?)),
        I::If(e1, e2, e3) => I::If(
            Box::new(replace(v, e1)?),
            Box::new(replace(v, e2)?),
            Box::new(replace(v, e3)?),
        ),
        I::Let(id, e1, e2) => I::Let(
            id.clone(),
            Box::new(replace(v, e1)?),
            Box::new(replace(v, e2)?),
        ),
        I::Fix(id, e) => I::Fix(id.clone(), Box::new(replace(v, e)?)),
        I::MkArray(sz, n) => I::MkArray(Box::new(replace(v, sz)?), Box::new(replace(v, n)?)),
        I::GetArray(id, idx) => I::GetArray(Box::new(replace(v, id)?), Box::new(replace(v, idx)?)),
        I::SetArray(id, idx, var) => I::SetArray(
            Box::new(replace(v, id)?),
            Box::new(replace(v, idx)?),
            Box::new(replace(v, var)?),
        ),
        I::V => I::V,
        I::Star => I::Var(v.to_string()),
    };

    Some(r)
}

// instantiate Q for k w/ alpha-renamed variables that are in-scope
// and of the right shape at the location of the well-formedness
// constraint
fn qstar(
    _: &str,
    in_scope: &HashSet<Id>,
    env: &HashMap<Id, explicit::Type>,
    qset: &[implicit::Expr],
) -> HashSet<lambdal::Expr> {
    use explicit::Type::TBool;

    let mut qstar: HashSet<lambdal::Expr> = HashSet::new();
    for tmpl in qset {
        if in_scope.is_empty() {
            let r = hindley_milner::infer_in(env.clone(), tmpl);
            if r.is_ok() {
                if let Ok(ee) = lambdal::q(tmpl) {
                    let ty = hm_shape_expr(env, &ee);
                    if let TBool = ty.clone() {
                        qstar.insert(ee);
                    } else {
                        println!("not TBool for {:?} ({:?})", ee, ty);
                    }
                }
            }
            continue;
        }
        for v in in_scope.iter() {
            if let Some(e) = replace(v, tmpl) {
                let r = hindley_milner::infer_in(env.clone(), &e);
                if r.is_ok() {
                    if let Ok(ee) = lambdal::q(&e) {
                        let ty = hm_shape_expr(env, &ee);
                        if let TBool = ty.clone() {
                            qstar.insert(ee);
                        } else {
                            println!("not TBool for {:?} ({:?})", ee, ty);
                        }
                    } else {
                        println!("lambdal conv failed for {:?}", e);
                    }
                }
            };
        }
    }

    qstar
}

fn build_a(
    constraints: &HashMap<Idx, Constraint>,
    env: &HashMap<Id, explicit::Type>,
    q: &[implicit::Expr],
) -> HashMap<Id, KInfo> {
    let mut a: HashMap<Id, KInfo> = HashMap::new();

    for (_, c) in constraints.iter() {
        if let ((in_scope, _), C::WellFormed(ty)) = c {
            if let T::Ref(_, base, bk) = &**ty {
                if let Liquid::K(id, _) = &**bk {
                    let all_qs = qstar(id, in_scope, env, q);
                    let curr_qs: Vec<_> = all_qs.iter().cloned().collect();
                    a.insert(
                        id.clone(),
                        KInfo {
                            base: *base,
                            all_qs,
                            curr_qs,
                        },
                    );
                } else {
                    unreachable!();
                }
            } else {
                panic!("WellFormed with E doesn't make sense: {:?}.", ty)
            }
        }
    }

    a
}

fn smt_from_imm<'ctx>(
    ctx: &'ctx Context,
    vars: &HashMap<String, Dynamic<'ctx>>,
    q: &lambdal::Imm,
) -> Dynamic<'ctx> {
    use lambdal::Imm as I;

    match q {
        I::Var(id) => match vars.get(id) {
            Some(v) => (*v).clone(),
            None => panic!("smt_from_imm: {} not in {:?}", id, vars),
        },
        I::Bool(b) => Dynamic::from(Bool::from_bool(ctx, *b)),
        I::Int(n) => Dynamic::from(Int::from_i64(ctx, *n)),
        I::V => vars["!v"].clone(),
        I::Star => unreachable!("star in smt?"),
        I::Fun(_, _) => unreachable!("fun in smt?"),
        I::Fix(_, _) => unreachable!("fix in smt?"),
    }
}

fn smt_from_op<'ctx>(
    ctx: &'ctx Context,
    s: &Solver,
    vars: &HashMap<String, Dynamic<'ctx>>,
    q: &lambdal::Op,
) -> Dynamic<'ctx> {
    use crate::common::Op2;
    use crate::lambdal::Op as O;

    match q {
        O::Imm(imm) => smt_from_imm(ctx, vars, imm),
        O::Op2(op, l, r) => {
            let il = smt_from_op(ctx, s, vars, l);
            let ir = smt_from_op(ctx, s, vars, r);
            match op {
                Op2::And | Op2::Or | Op2::Impl | Op2::Iff => {
                    let il = il.as_bool().unwrap();
                    let ir = ir.as_bool().unwrap();
                    let opcode = match op {
                        Op2::And => Bool::and(ctx, &[&il, &ir]),
                        Op2::Or => Bool::or(ctx, &[&il, &ir]),
                        Op2::Impl => il.implies(&ir),
                        Op2::Iff => {
                            panic!("iff not implemented");
                        }
                        _ => unreachable!(),
                    };
                    // s.assert(&opcode)
                    Dynamic::from(opcode)
                }
                Op2::Add
                | Op2::Sub
                | Op2::Mul
                | Op2::LT
                | Op2::LTE
                | Op2::GT
                | Op2::GTE
                | Op2::Eq => {
                    let il = il.as_int().unwrap();
                    let ir = ir.as_int().unwrap();
                    let opcode = match op {
                        Op2::LT => Dynamic::from(il.lt(&ir)),
                        Op2::LTE => Dynamic::from(il.le(&ir)),
                        Op2::GT => Dynamic::from(il.gt(&ir)),
                        Op2::GTE => Dynamic::from(il.ge(&ir)),
                        Op2::Eq => Dynamic::from(il._eq(&ir)),
                        Op2::Add => Dynamic::from(Int::add(ctx, &[&ir, &il])),
                        Op2::Sub => Dynamic::from(Int::sub(ctx, &[&ir, &il])),
                        Op2::Mul => Dynamic::from(Int::mul(ctx, &[&ir, &il])),
                        _ => unreachable!(),
                    };
                    //s.assert(&opcode)
                    opcode
                }
            }
        }
        _ => {
            panic!("smt_from_op unimplemented {:?}", q);
        }
    }
}

fn smt_from_expr<'ctx>(
    ctx: &'ctx Context,
    s: &Solver,
    vars: &HashMap<String, Dynamic<'ctx>>,
    q: &lambdal::Expr,
) -> Dynamic<'ctx> {
    use lambdal::Expr as E;

    match q {
        E::Let(id, e1, e2) => {
            let id_idx = match vars.get(id) {
                Some(idx) => (*idx).clone(),
                None => panic!("key {} not found in {:?}", id, vars),
            };
            let rhs = smt_from_expr(ctx, s, vars, e1);
            if let Some(rhs) = rhs.as_int() {
                s.assert(&id_idx.as_int().unwrap()._eq(&rhs));
            } else if let Some(rhs) = rhs.as_bool() {
                s.assert(&id_idx.as_bool().unwrap()._eq(&rhs));
            } else {
                unreachable!();
            }
            smt_from_expr(ctx, s, vars, e2)
        }
        E::App(e1, e2) => {
            if **e1 == Imm::Var(String::from("not")) {
                s.assert(&smt_from_imm(ctx, vars, e2).as_bool().unwrap().not());
                Dynamic::from(Bool::from_bool(ctx, false))
            } else {
                panic!("TODO: only supported app is not");
            }
        }
        E::Op(op) => smt_from_op(ctx, s, vars, op),
        _ => {
            panic!("smt_from_expr unimplemented {:?}", q);
        }
    }
}

fn expr_from_var(a: &HashMap<Id, KInfo>, var: &str, ty: &Type) -> Vec<lambdal::Expr> {
    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));
    let exprs = match ty {
        T::Ref(_, _, inner) => match &**inner {
            Liquid::E(expr) => vec![expr.clone()],
            Liquid::K(p_id, substs) => {
                let qs = match a.get(p_id) {
                    Some(ps) => ps.clone().curr_qs,
                    None => vec![const_true],
                };
                let mut qs_replaced = vec![];
                for mut q in qs {
                    for (sub, var) in substs.iter() {
                        q = replace_expr(&q, &Imm::Var(var.clone()), sub);
                    }
                    qs_replaced.push(q);
                }
                qs_replaced
            }
        },
        T::Fun(_, _, _) => {
            // functions are uninterpreted.
            vec![const_true]
        }
    };
    let mut instantiated = vec![];
    for e in exprs {
        instantiated.push(replace_expr(&e, &Imm::V, &Imm::Var(var.to_string())));
    }
    instantiated
}

fn const_for_type<'ctx>(
    ctx: &'ctx Context,
    var: &str,
    ty: &explicit::Type,
) -> Option<Dynamic<'ctx>> {
    let sort: Dynamic<'ctx> = match ty {
        explicit::Type::TInt => Dynamic::from(Int::new_const(ctx, var)),
        explicit::Type::TBool => Dynamic::from(Bool::new_const(ctx, var)),
        _ => {
            //println!("TODO: v'{}' more sorts than int ({:?})", var, ty);
            return None;
        }
    };
    Some(sort)
}

// whether the conjunction of all p implies the conjunction of all q
fn implication_holds(
    env: &HashMap<Id, explicit::Type>,
    v_ty: explicit::Type,
    p: &[lambdal::Expr],
    q: &[lambdal::Expr],
) -> bool {
    let cfg = Config::default();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let mut senv: HashMap<Id, Dynamic> = HashMap::new();

    // Defining the symbolic vars x & y
    for (var, ty) in env {
        if let Some(c) = const_for_type(&ctx, var, ty) {
            senv.insert(var.clone(), c);
        }
    }

    senv.insert(
        String::from("!v"),
        const_for_type(&ctx, &"!v".to_string(), &v_ty).unwrap(),
    );

    let mut ps: Vec<Dynamic> = Vec::new();
    for t in p {
        let pred = smt_from_expr(&ctx, &solver, &senv, t);
        ps.push(pred);
    }

    let mut qs: Vec<_> = Vec::new();
    for t in q {
        let pred = smt_from_expr(&ctx, &solver, &senv, t);
        qs.push(pred);
    }

    let p_all = match ps.len() {
        0 => Bool::from_bool(&ctx, true),
        1 => ps[0].as_bool().unwrap(),
        _ => {
            let ps: Vec<_> = ps.iter().map(|p| p.as_bool().unwrap()).collect();
            let ps_refs: Vec<_> = ps.iter().collect();
            Bool::and(&ctx, &ps_refs)
        }
    };
    let q_all = match qs.len() {
        0 => Bool::from_bool(&ctx, true),
        1 => qs[0].as_bool().unwrap(),
        _ => {
            let qs: Vec<_> = qs.iter().map(|q| q.as_bool().unwrap()).collect();
            let qs_refs: Vec<_> = qs.iter().collect();
            Bool::and(&ctx, &qs_refs)
        }
    };
    let imply = p_all.implies(&q_all);
    solver.assert(&imply.not());

    let result = solver.check();
    matches!(result, z3::SatResult::Unsat)
}

// whether the conjunction of all p implies the conjunction of all q
fn weaken(
    env: &HashMap<Id, explicit::Type>,
    renv: &HashMap<Id, Type>,
    a: &HashMap<Id, KInfo>,
    all_p: &[(HashSet<Id>, LinkedList<Expr>, Box<Type>)],
    qs: &HashSet<lambdal::Expr>,
) -> Option<Vec<lambdal::Expr>> {
    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));

    let mut curr_qs: Vec<lambdal::Expr> = Vec::new();
    'outer: for q in qs {
        for (in_scope, path_constraints, p) in all_p {
            let mut p = match &**p {
                T::Ref(_, _, inner) => match &**inner {
                    Liquid::E(expr) => vec![expr.clone()],
                    Liquid::K(p_id, _) => match a.get(p_id) {
                        Some(ps) => ps.clone().curr_qs,
                        None => vec![const_true.clone()],
                    },
                },
                T::Fun(_, _, _) => panic!("unexpected {:?} -- should all be split() by now", p),
            };
            for var in in_scope {
                p.extend(expr_from_var(a, var, &renv[var]));
            }
            for pc in path_constraints {
                p.push(pc.clone());
            }

            if !implication_holds(env, TInt, &p, &[q.clone()]) {
                continue 'outer;
            }
        }
        curr_qs.push(q.clone());
    }

    curr_qs.sort();
    Some(curr_qs)
}

fn solve(
    env: &HashMap<Id, explicit::Type>,
    renv: &HashMap<Id, Type>,
    constraints: &LinkedList<STConstraints>,
    a: &HashMap<Id, KInfo>,
) -> Result<HashMap<Id, KInfo>> {
    let const_true = Expr::Op(Op::Imm(Imm::Bool(true)));

    for (all_p, id) in constraints.iter() {
        // if we don't find the ID in our environment, it means we are
        // looking at unbound function parameters -- which means we can just look
        let qs = match a.get(id) {
            Some(q) => q.clone(),
            None => {
                let mut all_qs = HashSet::new();
                all_qs.insert(const_true.clone());
                // FIXME: shouldn't default to Int
                KInfo {
                    base: Base::Int,
                    all_qs,
                    curr_qs: vec![const_true.clone()],
                }
            }
        };

        for (in_scope, path_constraints, p) in all_p.iter() {
            let mut p = match &**p {
                T::Ref(_, _, inner) => match &**inner {
                    Liquid::E(expr) => vec![expr.clone()],
                    Liquid::K(p_id, _) => {
                        let qs = match a.get(p_id) {
                            Some(ps) => ps.clone().curr_qs,
                            None => vec![const_true.clone()],
                        };
                        qs
                    }
                },
                T::Fun(_, _, _) => panic!("unexpected {:?} -- should all be split() by now", p),
            };

            for var in in_scope {
                p.extend(expr_from_var(a, var, &renv[var]));
            }
            for pc in path_constraints {
                p.push(pc.clone());
            }

            let implication = implication_holds(env, TInt, &p, &qs.curr_qs);
            if !implication {
                if qs.curr_qs[0] == const_true {
                    return err!("implication failure for -> true");
                }
                match weaken(env, renv, a, all_p, &qs.all_qs) {
                    Some(new_qs) => {
                        let mut new_a = a.clone();
                        new_a.insert(
                            id.clone(),
                            KInfo {
                                base: qs.base,
                                all_qs: qs.all_qs.clone(),
                                curr_qs: new_qs,
                            },
                        );
                        return solve(env, renv, constraints, &new_a);
                    }
                    None => {
                        return err!("Weaken failed for {:?}", p);
                    }
                }
            }
        }
    }

    Ok(a.clone())
}

fn conjoin(qs: &[Expr]) -> Op {
    use crate::common::Op2;
    if let Expr::Op(op) = qs[0].clone() {
        match qs[1..].len() {
            0 => op,
            _ => Op::Op2(Op2::And, Box::new(op), Box::new(conjoin(&qs[1..]))),
        }
    } else {
        panic!("expected q to be an Op, not {:?}", qs)
    }
}

fn concretize_liquid(_: &HashMap<Id, Type>, a: &HashMap<Id, KInfo>, lqdt: &Liquid) -> Liquid {
    match lqdt {
        Liquid::E(expr) => Liquid::E(expr.clone()),
        Liquid::K(id, _) => {
            // TODO: substitutions
            if !a.contains_key(id) {
                //panic!("NOT FOUND {}", id);
                return Liquid::E(Expr::Op(Op::Imm(Imm::Bool(true))));
            }
            let qs = &a[id].curr_qs;
            match qs.len() {
                0 => Liquid::E(Expr::Op(Op::Imm(Imm::Bool(true)))),
                _ => Liquid::E(Expr::Op(conjoin(&qs))),
            }
        }
    }
}

fn concretize_ty(renv: &HashMap<Id, Type>, a: &HashMap<Id, KInfo>, ty: &Type) -> Type {
    match ty {
        T::Ref(in_scope, base, predicate) => T::Ref(
            in_scope.clone(),
            *base,
            Box::new(concretize_liquid(renv, a, predicate)),
        ),
        T::Fun(id, xty, yty) => T::Fun(
            id.clone(),
            Box::new(concretize_ty(renv, a, xty)),
            Box::new(concretize_ty(renv, a, yty)),
        ),
    }
}

// turn types that have k in them into dependent types w/o ks
fn concretize(renv: &HashMap<Id, Type>, a: &HashMap<Id, KInfo>) -> HashMap<Id, Type> {
    let mut result = HashMap::new();

    for (id, ty) in renv {
        result.insert(id.clone(), concretize_ty(renv, a, ty));
    }

    result
}

pub fn infer(
    expr: &Expr,
    env: &HashMap<Id, explicit::Type>,
    q: &[implicit::Expr],
) -> Result<HashMap<Id, Type>> {
    let mut k_env = KEnv::new(env);
    println!("infer:\t{:?}\n", expr);
    let (top, constraint_list) = cons_expr(&mut k_env, &LinkedList::new(), expr);

    {
        println!("Liquid Γ:");
        let mut ids: Vec<_> = k_env.refined.keys().clone().collect();
        ids.sort();
        for id in ids {
            println!("{}:\t{:?}", id, k_env.refined[id]);
        }
    }
    println!();

    let mut constraints: HashMap<Idx, Constraint> = HashMap::new();
    split(&mut constraints, &constraint_list);

    println!("Subtype Constraints");
    #[allow(clippy::type_complexity)]
    let mut by_id: HashMap<Id, Vec<(HashSet<Id>, LinkedList<Expr>, Box<Type>)>> = HashMap::new();
    let mut ckeys: Vec<Idx> = constraints.keys().copied().collect();
    // sort constrains to make output easier to understand
    ckeys.sort_by_key(|k| {
        if let ((_, _), C::Subtype(_, bref)) = &constraints[k] {
            if let T::Ref(_, _, bk) = &**bref {
                if let Liquid::K(id, _) = &**bk {
                    i32::from_str(&id[2..]).unwrap_or(-1)
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        } else {
            -2
        }
    });

    // group subtyping constraints by supertype
    for ckey in ckeys.iter() {
        let c = &constraints[&ckey];
        if let ((in_scope, path), C::Subtype(p, e)) = c {
            //println!("Γ ⊢ {:?}   \t<: {:?} (PATH: {:?})", p, e, path);
            println!(
                "Γ ⊢ {:?}   \t<: {:?} (PATH: {:?}) (IN_SCOPE: {:?})",
                p, e, path, in_scope
            );
            if let T::Ref(_, _, bk) = &**e {
                if let Liquid::K(id, _) = &**bk {
                    let mut antecedent = vec![(in_scope.clone(), path.clone(), p.clone())];
                    if by_id.contains_key(id) {
                        let mut others = by_id[id].clone();
                        others.append(&mut antecedent);
                        by_id.insert(id.clone(), others);
                    } else {
                        by_id.insert(id.clone(), antecedent);
                    }
                }
            } else {
                println!("TODO: subtype constraint for non-liquid type {:?}", e);
            }
        }
    }
    println!();

    let mut all_constraints: LinkedList<STConstraints> = LinkedList::new();
    for (id, v) in by_id {
        //println!("->{}:\t{:?}", id, v);
        all_constraints.push_back((v.clone(), id.clone()));
    }

    let a = build_a(&constraints, env, q);
    let min_a = solve(env, &k_env.refined, &all_constraints, &a)?;

    use std::str::FromStr;
    {
        println!("a:");
        let mut ids: Vec<_> = a.keys().clone().collect();
        ids.sort_by_key(|id| i32::from_str(&id[2..]).unwrap_or(-1));
        for id in ids {
            println!("{}\t{:?}", id, a[id].all_qs);
        }
    }
    {
        println!("min_a:");
        let mut ids: Vec<_> = min_a.keys().clone().collect();
        ids.sort_by_key(|id| i32::from_str(&id[2..]).unwrap_or(-1));
        for id in ids {
            println!("{}\t{:?}", id, min_a[id].curr_qs);
        }
    }

    println!(
        "\nRESULT TYPE: {:?}",
        concretize_ty(&k_env.refined, &min_a, &top)
    );
    Ok(concretize(&k_env.refined, &min_a))
}

pub(crate) fn q(input: &str) -> Result<crate::implicit::Expr> {
    use crate::common::LiquidError;
    let lexer = Tokenizer::new(input);
    ProgramParser::new()
        .parse(&input, lexer)
        .map_err(|e| LiquidError::new(format!("Parser: {:?}", e)))
}

#[cfg(test)]
pub(crate) fn expr(input: &str) -> Result<crate::lambdal::Expr> {
    use crate::common::LiquidError;
    let lexer = Tokenizer::new(input);
    match ProgramParser::new().parse(&input, lexer) {
        Ok(iexpr) => lambdal::q(&iexpr),
        Err(err) => Err(LiquidError::new(format!("Parser: {:?}", err))),
    }
}

#[test]
fn test_implication() {
    let mut env: HashMap<Id, explicit::Type> = HashMap::new();
    env.insert(String::from("x"), explicit::Type::TInt);
    env.insert(String::from("y"), explicit::Type::TInt);
    env.insert(String::from("!tmp-0!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-0!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-0!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-1!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-1!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-1!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-2!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-2!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-2!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!0"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!1"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!2"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!3"), explicit::Type::TBool);
    env.insert(String::from("!tmp-3!4"), explicit::Type::TBool);

    let p = [expr("x <= y ∧ ν = y").unwrap()];
    let q = [expr("ν >= x ∧ ν >= y").unwrap()];

    // expect this to hold
    if !implication_holds(&env, TInt, &p, &q) {
        die!("1 expected {:?} => {:?}", p, q);
    }

    let p = [expr("x <= y ∧ ν = y").unwrap()];
    let q = [expr("ν < 0 ∧ ν >= x ∧ ν >= y").unwrap()];

    // but this shouldn't
    if implication_holds(&env, TInt, &p, &q) {
        die!("2 expected {:?} => {:?}", p, q);
    }
}

#[test]
fn z3_works() {
    let cfg = Config::new();
    // TODO: set the logic to LIA?
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Defining the symbolic vars x & y
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let v = Int::new_const(&ctx, "v");

    let p = Bool::and(&ctx, &[&x.le(&y), &v._eq(&y)]);
    let k = Bool::and(&ctx, &[&v.ge(&x), &v.ge(&y)]);
    solver.assert(&p.implies(&k).not());

    let result = solver.check();
    assert!(matches!(result, z3::SatResult::Unsat));
}
