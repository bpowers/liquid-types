use std::collections::HashMap;

use common::{Result, Id, Op2, Const};
use explicit;
use implicit;
use hindley_milner;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Imm {
    Bool(bool),
    Int(i64),
    Var(Id),
//    Fun(Id, Box<Expr>),
//    Fix(Id, Box<Expr>),
    Star,
    V,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Op {
    Op2(Op2, Box<Imm>, Box<Imm>),
//    MkArray(Box<Imm>, Box<Imm>),
//    GetArray(Box<Imm>, Box<Imm>),
//    SetArray(Box<Imm>, Box<Imm>, Box<Imm>),
    Imm(Imm),
}
//    WellFormed(Imm), // Var-only

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
//    If(Box<Imm>, Box<Imm>, Box<Imm>),
//    App(Box<Imm>, Box<Imm>),
    Let(Id, Box<Expr>, Box<Expr>),
    Op(Op),
}

struct Converter {
    env: HashMap<Id, explicit::Type>,
    next_id: i32,
}

impl Converter {
    fn new() -> Converter {
        Converter {
            env: HashMap::new(),
            next_id: 0,
        }
    }

    fn op(&mut self, e: &explicit::Expr) -> Op {
        use typed::Expr as E;
        match *e {
            E::Const(Const::Bool(c)) => Op::Imm(Imm::Bool(c)),
            E::Const(Const::Int(c)) => Op::Imm(Imm::Int(c)),
            _ => {
                panic!("TODO: implement term for {:?}", e);
            }
        }
    }

    fn tmp(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;

        format!("!tmp!{}", id)
    }

    // 1:1 translation -- can't fail
    fn expr<F>(&mut self, e: &explicit::Expr, k: F) -> Expr
        where F: Fn(Expr) -> Expr {

        use self::Imm as I;
        use common::Op2 as O;
        use typed::Expr as E;
        use self::Op::*;
        use self::Expr::*;

        match *e {
            E::Const(_) => Op(self.op(e)),
            // E::Op2(op, ref l, ref r) => {
            //     self.expr(l, |ll| {
            //         let tmp1 = self.tmp();
            //         Let(tmp1.clone(), box ll,
            //             box self.expr(r, |rr| {
            //                 let tmp2 = self.tmp();
            //                 k(Let(tmp2.clone(), box rr,
            //                     box Op(Op2(op,
            //                                box I::Var(tmp1.clone()),
            //                                box I::Var(tmp2.clone())))))
            //             }))
            //     })
            // }
            _ => {
                panic!("TODO: implement expr for {:?}", e);
            }
        }
        // E::Op2(op, ref e1, ref e2) => {
        //     let (n, e1, _) = convert(n, env, renamed, e1);
        //     let (n, e2, _) = convert(n, env, renamed, e2);
        //     (n, I::Op2(op, box e1, box e2), explicit::opty(op))
        // }
        // E::If(ref e1, ref e2, ref e3) => {
        //     let (n, e1, _) = convert(n, env, renamed, e1);
        //     let (n, e2, t) = convert(n, env, renamed, e2);
        //     let (n, e3, _) = convert(n, env, renamed, e3);
        //     (n, I::If(box e1, box e2, box e3), t)
        // }
        // E::Var(ref x) => {
        //     let αx = renamed.get(x).unwrap();
        //     (n, I::Var(αx.clone()), env.get(αx).unwrap().clone())
        // }
        // E::Let(ref id, ref e1, ref e2) => {
        //     let mut renamed = renamed.clone();
        //     let (n, αid) = (n+1, format!("{}!a{}", id, n));

        //     let (n, e1, t1) = convert(n, env, &renamed, e1);

        //     renamed.insert(id.clone(), αid.clone());
        //     env.insert(αid.clone(), t1);

        //     let (n, e2, t2) = convert(n, env, &renamed, e2);
        //     (n, I::Let(αid, box e1, box e2), t2)
        // }
        // E::Fun(ref id, ref t1, ref e) => {
        //     let mut renamed = renamed.clone();
        //     let (n, αid) = (n+1, format!("{}!a{}", id, n));

        //     renamed.insert(id.clone(), αid.clone());
        //     env.insert(αid.clone(), t1.clone());

        //     let (n, e, t2) = convert(n, env, &renamed, e);

        //     (n, I::Fun(αid, box e), Type::TFun(box t1.clone(), box t2))
        // }
        // E::Fix(ref id, ref t1, ref e) => {
        //     let mut renamed = renamed.clone();
        //     let (n, αid) = (n+1, format!("{}!a{}", id, n));

        //     renamed.insert(id.clone(), αid.clone());
        //     env.insert(αid.clone(), t1.clone());

        //     let (n, e, t2) = convert(n, env, &renamed, e);

        //     (n, I::Fix(αid, box e), t2)
        // }
        // E::App(ref e1, ref e2) => {
        //     let (n, e1, t1) = convert(n, env, renamed, e1);
        //     let (n, e2, _) = convert(n, env, renamed, e2);
        //     let t = match t1 {
        //         Type::TFun(_, t2) => *t2,
        //         _ => panic!("expected TFun, not {:?}", t1),
        //     };
        //     (n, I::App(box e1, box e2), t)
        // }
        // E::MkArray(ref sz, ref val) => {
        //     let (n, sz, _) = convert(n, env, renamed, sz);
        //     let (n, val, _) = convert(n, env, renamed, val);
        //     (n, I::MkArray(box sz, box val), Type::TIntArray)
        // }
        // E::GetArray(ref id, ref idx) => {
        //     let (n, id, _) = convert(n, env, renamed, id);
        //     let (n, idx, _) = convert(n, env, renamed, idx);
        //     (n, I::GetArray(box id, box idx), Type::TInt)
        // }
        // E::SetArray(ref id, ref idx, ref v) => {
        //     let (n, id, _) = convert(n, env, renamed, id);
        //     let (n, idx, _) = convert(n, env, renamed, idx);
        //     let (n, v, _) = convert(n, env, renamed, v);
        //     (n, I::SetArray(box id, box idx, box v), Type::TIntArray)
        // }
    }
}


fn anf(implicit_expr: &implicit::Expr) -> Result<(Expr, HashMap<Id, explicit::Type>)> {
    let mut env: HashMap<Id, explicit::Type> = HashMap::new();

    let explicit_expr = hindley_milner::infer(implicit_expr)?;

    let mut conv = Converter::new();
    let expr = conv.expr(&explicit_expr, |x| x);

    Ok((expr, env))
    // step 1 -- arithmatic + let bindings
    // step 2 -- conds + boolean vals
    // step 3 -- arrays
    // step 4 -- closures

// (define (Value? M)
//   (match M
//     [`(quote ,_)         #t]
//     [(? number?)         #t]
//     [(? boolean?)        #t]
//     [(? string?)         #t]
//     [(? char?)           #t]
//     [(? symbol?)         #t]
//     [(or '+ '- '* '/ '=) #t]
//     [else                #f]))


// (define (normalize-term M) (normalize M (λ (x) x)))

// (define (normalize M k)
//   (match M
//     [`(λ ,params ,body)
//       (k `(λ ,params ,(normalize-term body)))]

//     [`(let ([,x ,M1]) ,M2)
//       (normalize M1 (λ (N1)
//        `(let ([,x ,N1])
//          ,(normalize M2 k))))]

//     [`(if ,M1 ,M2 ,M3)
//       (normalize-name M1 (λ (t)
//        (k `(if ,t ,(normalize-term M2)
//                   ,(normalize-term M3)))))]

//     [`(,Fn . ,M*)
//       (normalize-name Fn (λ (t)
//        (normalize-name* M* (λ (t*)
//         (k `(,t . ,t*))))))]

//     [(? Value?)             (k M)]))

// (define (normalize-name M k)
//   (normalize M (λ (N)
//     (if (Value? N) (k N)
//         (let ([t (gensym)])
//          `(let ([,t ,N]) ,(k t)))))))

// (define (normalize-name* M* k)
//   (if (null? M*)
//       (k '())
//       (normalize-name (car M*) (λ (t)
//        (normalize-name* (cdr M*) (λ (t*)
//         (k `(,t . ,t*))))))))

}


macro_rules! test_anf(
    ($s:expr, $ae:expr) => { {
        use implicit_parse;
        use tok::Tokenizer;
        use std;
        let s = $s;
        let tokenizer = Tokenizer::new(&s);
        let iexpr = match implicit_parse::parse_Program(&s, tokenizer) {
            Ok(iexpr) => iexpr,
            Err(e) => {
                die!("parse_Program({}): {:?}", $s, e);
            }
        };
        let anf_expr = match anf(&iexpr) {
            Ok((anf_expr, _)) => anf_expr,
            Err(e) => {
                die!("anf: {:?}", e);
            }
        };
        if anf_expr != $ae {
            die!("mismatch {:?} != {:?}", anf_expr, $ae);
        }
    } }
);

#[test]
fn anf_transforms() {
    use common::Op2 as O;
    use self::Imm as I;
    use self::Op::*;
    use self::Expr::*;

    test_anf!(
        "-22",
        Op(Imm(I::Int(-22))));
    test_anf!(
        "2+3",
        Let(String::from("!tmp!0"),
            box Op(Op2(O::Add, box I::Int(2), box I::Int(3))),
            box Op(Imm(I::Var(String::from("!tmp!0"))))));
}
