
use explicit;

use std::collections::HashMap;

type Closure = HashMap<String, Value>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    VInt(i32),
    VBool(bool),
    VClosure(Box<Closure>, Box<explicit::Expr>),
    VCons(Box<Value>, Box<Value>),
    VEmpty,
}


// pub fn eval(exp: &explicit::Expr) ->

// let rec eval' (ctx : env) (e : exp) : value =
//   (* All of our binary operators work on ints *)
//   let op2_eval (op2) (e1) (e2) : value =
//     let v1 = vint (eval' ctx e1) in
//     let v2 = vint (eval' ctx e2) in
//     match op2 with
//     | LT  -> VBool (v1 < v2)
//     | GT  -> VBool (v1 > v2)
//     | Eq  -> VBool (v1 = v2)
//     | Add -> VInt (v1 + v2)
//     | Sub -> VInt (v1 - v2)
//     | Mul -> VInt (v1 * v2)
//   in
//   (* used when evaluating fixpoint functions *)
//   let rec subst (id: id) (fix: exp) (e: exp) : exp = match e with
//     | Id curr           -> if curr = id then fix else e
//     | Const _           -> e
//     | Op2 (op, e1, e2)  -> Op2 (op, subst id fix e1, subst id fix e2)
//     | If (e1, e2, e3)   -> If (subst id fix e1, subst id fix e2, subst id fix e3)
//     | Let (iid, e1, e2) -> Let (iid, subst id fix e1, subst id fix e2)
//     | Fun (iid, ty, e)  -> Fun (iid, ty, subst id fix e)
//     | Fix (iid, ty, e)  -> Fix (iid, ty, subst id fix e)
//     | App (e1, e2)      -> App (subst id fix e1, subst id fix e2)
//     | Empty _           -> e
//     | Cons (e1, e2)     -> Cons (subst id fix e1, subst id fix e2)
//     | Head e            -> Head (subst id fix e)
//     | Tail e            -> Tail (subst id fix e)
//     | IsEmpty e         -> IsEmpty (subst id fix e)
//   in
//   let is_empty (v) : value = match v with
//     | VEmpty _     -> VBool true
//     | VCons (_, _) -> VBool false
//     | _ -> failwith "is_empty unreachable"
//   in
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


pub fn interpret(expr: &explicit::Expr) -> Value {
    Value::VBool(false)
}
