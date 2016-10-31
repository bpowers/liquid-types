let a = array(20, 0) in (*      a: IntArray *)
let idx = 30 in         (*    idx: {v: Int | v = 30 }*)
a[idx]                  (* a[idx]: *)


(*
let max x y =
  if x > y then x else y

// x in the then:
//   x:int; y:int; x > y |- {ν = x} <: {x ≤ ν ∧ y ≤ ν}
// holds as the following implication is valid in EUFA:
//   ((true ∧ true ∧ x > y) ∧ (ν = x)) ⇒ (x ≤ ν ∧ y ≤ ν)

let rec sum k =
  if k < 0 then 0 else
    let s = sum (k-1) in
    s + k

let foldn n b f =
  let rec loop i c =
    if i < n then loop (i+1) (f i c) else c in
  loop 0 b

let arraymax a =
  let am l m = max (sub a l) m in
  foldn (len a) 0 am *)
