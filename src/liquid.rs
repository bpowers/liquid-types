use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use common;
use implicit;
use explicit;

use implicit::Expr;
use common::{Id, Result};
use refined::{Base, T};

use rustproof_libsmt::backends::smtlib2::*;
use rustproof_libsmt::backends::backend::*;
use rustproof_libsmt::backends::z3::Z3;

// Include the Core (bool) and Int theory and its functions
use rustproof_libsmt::theories::{integer};

// Include the LIA logic
use rustproof_libsmt::logics::lia::LIA;

// Qbc (bounds checking)
// X: 0, *, len *
// ν < X
// ν <= X
// ν = X
// ν >= X
// ν > X

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum C {
    WellFormed(Box<Type>),
    Subtype(Box<Type>, Box<Type>),
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Liquid {
    E(implicit::Expr),
    K(Id, Box<LinkedList<Expr>>), // list of pending substitutions
}

pub type Type = T<Liquid>;
pub type Implication = (LinkedList<Expr>, Box<Type>, Box<Type>);

pub type Idx = i32; // constraint index

pub type Constraint = ((HashSet<Id>, LinkedList<Expr>), C); // Boolean valued expressions & their environments

#[derive(Debug, Clone)]
pub struct KInfo {
    allq: HashSet<implicit::Expr>,
    currq: HashSet<implicit::Expr>,
    deps: Vec<Idx>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Env {
    shape: HashMap<Id, explicit::Type>,
    refined_env: HashMap<Id, Type>,
    path_constraints: LinkedList<Expr>,
}

impl Env {
    fn new(shape: &HashMap<Id, explicit::Type>) -> Env {
        Env {
            shape: shape.clone(),
            refined_env: HashMap::new(),
            path_constraints: LinkedList::new(),
        }
    }

    fn get(&self, s: &Id) -> Type {
        match self.refined_env.get(s) {
            Some(ty) => ty.clone(),
            None => panic!("env.get('{}' missing ({:?})", s, self.in_scope()),
        }
    }

    fn insert(&mut self, s: &Id, ty: &Type) {
        self.refined_env.insert(s.clone(), ty.clone());
    }

    fn add_constraint(&mut self, e: &Expr) {
        self.path_constraints.push_back(e.clone())
    }

    fn in_scope(&self) -> HashSet<Id> {
        let keys: HashSet<_> = self.refined_env.keys().cloned().collect();
        return keys
    }

    fn snapshot(&self) -> (HashSet<Id>, LinkedList<Expr>) {
        (self.in_scope(), self.path_constraints.clone())
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KEnv {
    shape: HashMap<Id, explicit::Type>,
    env_id: String,
    next_id: i32,
}

fn hm_shape(env: &HashMap<Id, explicit::Type>, expr: &Expr) -> explicit::Type {
    use implicit::Expr::*;
    use explicit::Type::*;

    match *expr {
        Var(ref id) => env.get(id).unwrap().clone(),
        Const(common::Const::Int(_)) => TInt,
        Const(common::Const::Bool(_)) => TBool,
        Op2(op, _, _) => explicit::opty(op),
        Fun(ref id, ref e) => TFun(box env.get(id).unwrap().clone(), box hm_shape(env, e)),
        App(ref e1, _) => {
            if let TFun(_, e2) = hm_shape(env, e1) {
                *e2
            } else {
                panic!("expected TFun, not {:?}", expr);
            }
        }
        If(_, ref e2, _) => hm_shape(env, e2),
        Let(_, _, ref e2) => hm_shape(env, e2),
        Fix(_, ref e) => hm_shape(env, e),
        MkArray(_, _) => TIntArray,
        GetArray(_, _) => TInt,
        SetArray(_, _, _) => TIntArray,
        Star => panic!("star found when it shouldn't be"),
        V => panic!("v found when it shouldn't be"),
    }
}

impl KEnv {
    fn new(shape: &HashMap<Id, explicit::Type>) -> KEnv {
        KEnv {
            shape: shape.clone(),
            env_id: String::from("κ"), // ν
            next_id: 0,
        }
    }

    fn fresh_ty(&mut self, env: &Env, ty: &explicit::Type) -> Type {
        let id = self.next_id;
        self.next_id += 1;

        let base = match *ty {
            explicit::Type::TInt => Base::Int,
            explicit::Type::TBool => Base::Bool,
            _ => panic!("FIXME: handle {:?}", ty),
        };
        let k = Liquid::K(format!("!k{}", id), box LinkedList::new());
        T::Ref(env.in_scope(), base, box k)
    }

    fn fresh(&mut self, env: &Env, expr: &Expr) -> Type {
        if let &Expr::Fun(ref id, ref e) = expr {
            let t1 = &self.shape.get(id).unwrap().clone();
            let fx = self.fresh_ty(env, t1);
            let f = self.fresh(env, e);
            T::Fun(id.clone(), box fx, box f)
        } else {
            let ty = hm_shape(&env.shape, expr);
            self.fresh_ty(env, &ty)
        }
    }
}

fn ty<'a>(_: &mut KEnv, _: &Env, c: &common::Const) -> Type {
    use common::Op2;
    use common::Const::*;
    use self::Liquid::E;

    let base = match *c {
        Int(_) => Base::Int,
        Bool(_) => Base::Bool,
    };

    println!("ty({:?})", base);

    // {ν : int | ν = 3 }
    let eq = E(Expr::Op2(Op2::Eq, box Expr::V, box Expr::Const(*c)));
    T::Ref(HashSet::new(), base, box eq)
}

fn base(ty: &Type) -> Option<Base> {
    match *ty {
        T::Ref(_, b, _) => Some(b),
        _ => None,
    }
}

fn subst(_: &Id, _: &Expr, ty: &Type) -> Type {
    println!("TODO: subst");
    ty.clone()
}

pub fn cons<'a>(k_env: &mut KEnv, env: &Env, expr: &Expr) -> (Type, LinkedList<Constraint>) {
    use implicit::Expr::*;
    use common::Op2::Eq;

    match *expr {
        Var(ref id) => {
            let ty: Type = if let Some(b) = base(&env.get(id)) {
                let eq = Op2(Eq, box V, box Var(id.clone()));
                T::Ref(env.in_scope(), b, box Liquid::E(eq))
            } else {
                println!("{} not base -- using just env ({:?})", id, env.get(id));
                env.get(id)
            };
            (ty, LinkedList::new())
        }
        Const(ref c) => {
            (ty(k_env, &env, c), LinkedList::new())
        }
        Op2(op, ref e1, ref e2) => {
            let (_, mut c1) = cons(k_env, env, e1);
            let (_, mut c2) = cons(k_env, env, e2);
            c1.append(&mut c2);

            let ty = explicit::opty(op);
            let base = match ty {
                explicit::Type::TInt => Base::Int,
                explicit::Type::TBool => Base::Bool,
                _ => panic!("FIXME: handle {:?}", ty),
            };

            let eq = Op2(Eq, box V, box expr.clone());
            let f = T::Ref(env.in_scope(), base, box Liquid::E(eq));

            (f, c1)
        }
        If(ref e1, ref e2, ref e3) => {
            let mut env_t = env.clone();
            let mut env_f = env.clone();
            env_t.add_constraint(&e1.clone());
            env_f.add_constraint(&App(box Var(String::from("not")), e1.clone()));

            let f = k_env.fresh(&env, expr);
            // type of e1 has already been verified to be a bool by HM
            let (_, mut c1) = cons(k_env, &env, e1);
            let (f2, mut c2) = cons(k_env, &env_t, e2);
            let (f3, mut c3) = cons(k_env, &env_f, e3);
            c1.append(&mut c2);
            c1.append(&mut c3);
            // Γ ⊢ (f)
            c1.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,e1 ⊢ (f2 <: f)
            c1.push_back((env_t.snapshot(), C::Subtype(box f2.clone(), box f.clone())));
            // Γ,¬e1 ⊢ (f3 <: f)
            c1.push_back((env_f.snapshot(), C::Subtype(box f3.clone(), box f.clone())));

            (f, c1)
        }
        Fun(ref x, ref e) => {
            let mut env = env.clone();
            let fx = k_env.fresh(&env, &Var(x.clone()));
            env.insert(x, &fx);
            let f = k_env.fresh(&env, e);
            let (fe, mut c) = cons(k_env, &env, e);
            // Γ ⊢ (x:fx → f)
            c.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,x:fx ⊢ (fe <: f)
            c.push_back((env.snapshot(), C::Subtype(box fe.clone(), box f.clone())));

            (f, c)
        }
        Fix(ref x, ref e) => {
            // const w/ ∀α.(α→α)→α
            let mut env = env.clone();
            let fx = k_env.fresh(&env, e);
            env.insert(x, &fx);

            // FIXME
            cons(k_env, &env, e)
        }
        Let(ref id, ref e1, ref e2) => {
            let mut env = env.clone();

            let f = k_env.fresh(&env, expr);
            let (f1, mut c1) = cons(k_env, &env, e1);
            env.insert(id, &f1);
            let (f2, mut c2) = cons(k_env, &env, e2);
            c1.append(&mut c2);
            // Γ ⊢ (f)
            c1.push_back((env.snapshot(), C::WellFormed(box f.clone())));
            // Γ,x:f1 ⊢ (f2 <: f)
            c1.push_back((env.snapshot(), C::Subtype(box f2.clone(), box f.clone())));

            (f, c1)
        }
        App(ref e1, ref e2) => {
            let (f1, mut c1) = cons(k_env, env, e1);
            println!("## {:?}\t:\t{:?}", e1, f1);
            let (f2, mut c2) = cons(k_env, env, e2);
            c1.append(&mut c2);
            if let T::Fun(ref x, ref fx, ref f) = f1 {
                let f = subst(x, e2, f);
                // Γ ⊢ (f2 <: fx)
                c1.push_back((env.snapshot(), C::Subtype(box f2.clone(), fx.clone())));
                return (f, c1);
            } else {
                panic!("expected TFun, not {:?}", f1);
            }
            // let (x:Fx → F, C1) = Cons(Γ, e1) in
            // let (F
            // 0
            // x, C2) = Cons(Γ, e2) in
            // ([e2/x]F, C1 ∪ C2 ∪ {Γ ` F
            // 0
            // x <: Fx})
        }
        _ => {
            println!("unhandled {:?}", expr);
            (T::Ref(env.in_scope(), Base::Bool, box Liquid::E(Const(common::Const::Bool(true)))), LinkedList::new())
        }
    }
}

fn split(map: &mut HashMap<Idx, Constraint>, constraints: &LinkedList<Constraint>) {
    let mut idx = 1;

    for c in constraints.iter() {
        if let &((ref scope, ref pathc), C::Subtype(box T::Fun(_, ref tx1, ref t1), box T::Fun(ref x2, ref tx2, ref t2))) = c {
            let mut contra_cs: LinkedList<Constraint> = LinkedList::new();

            contra_cs.push_back(((scope.clone(), pathc.clone()), C::Subtype(tx2.clone(), tx1.clone())));

            let mut rscope = scope.clone();
            rscope.insert(x2.clone());
            contra_cs.push_back(((rscope, pathc.clone()), C::Subtype(t1.clone(), t2.clone())));

            // recurse
            split(map, &contra_cs);
        } else {
            map.insert(idx, c.clone());
            idx += 1;
        }
    }
}

fn build_a(constraints: &HashMap<Idx, Constraint>) -> HashMap<Id, KInfo> {
    for (idx, c) in constraints.iter() {
        // if ! well formed, pass

    }

    HashMap::new()
}

pub fn infer(expr: &Expr, env: &HashMap<Id, explicit::Type>) -> Result<Expr> {
    let mut k_env = KEnv::new(env);
    let (f, constraint_list) = cons(&mut k_env, &Env::new(env), expr);

    let mut constraints: HashMap<Idx, Constraint> = HashMap::new();
    split(&mut constraints, &constraint_list);

    let a = build_a(&constraints);

    println!("CONS ({}):\n", k_env.next_id);
    println!("a\t{:?}", a);

    // for co in c.iter() {
    //     let ((_, pathc), constr) = co.clone();
    //     println!("\t{:?}\n\t\t{:?}", pathc, constr);
    // };

    let mut z3 = Z3::new_with_binary("./z3");
    // Defining an instance of Z3 solver
    let mut solver = SMTLib2::new(Some(LIA));
    solver.set_logic(&mut z3);

    // Defining the symbolic vars x & y
    let x = solver.new_var(Some("x"), integer::Sorts::Int);
    let y = solver.new_var(Some("y"), integer::Sorts::Int);

    // Defining the integer constants
    let int5 = solver.new_const(integer::OpCodes::Const(5));
    let int1 = solver.new_const(integer::OpCodes::Const(1));

    // Defining the assert conditions
    let cond1 = solver.assert(integer::OpCodes::Add, &[x, y]);
    let _  = solver.assert(integer::OpCodes::Gt, &[cond1, int5]);
    let _  = solver.assert(integer::OpCodes::Gt, &[x, int1]);
    let _  = solver.assert(integer::OpCodes::Gt, &[y, int1]);

    let (result, _) = solver.solve(&mut z3, false);
    match result {
        Ok(result) => {
            println!("x: {}; y: {}", result[&x], result[&y]);
        }
        Err(e) => println!("No solutions for x and y found for given set of constraints ({:?})", e),
    }

    // TODO:
    // let a = Solve(Split(c), λκ.Inst(Γ, e, Q)) in
    // a(f)

    Ok(Expr::Const(common::Const::Bool(false)))
}
