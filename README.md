Logically Qualified Data Types
==============================

This repository contains an implementation in Rust of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on an
implicitly-typed variant of ML.


BUILD
-----

This projects requires having the [z3 theorem
prover](https://github.com/Z3Prover/z3/wiki) library in your library search path.

RUN
---

As an example, `examples/max3.ml` defines a function max and applies it
to two arguments:

```ML
let max = fun x -> fun y -> if x > y then x else y in max 1 -3
```


You can infer liquid types for this program by
running:

```sh
$ target/debug/liquid-types examples/max3.ml
```

This will spit out debugging information about the liquid environment
(Γ) and subtyping constraints, the liquid type templates we start with
(a), and the liquid type templates after we've iteratively weakened
them (min_a), along with the refined types produced by running liquid
type inference.

For example, the type inferred for max above is:

```
max_a1:	F(x_a2: {ν: Int | 0 ≤ ν} → F(y_a3: {ν: Int | true} → {ν: Int | 0 ≤ ν ∧ x_a2 ≤ ν ∧ y_a3 ≤ ν}))
```

(the `_a$N` suffix on variables can be ignored, it is added during A-normalization)

There are additional programs in the `examples/` directory, and feel
free to try your own!  If you hit any problems, please open an
[issue](/../../issues/new).



IMPLEMENTED
-----------

Most of the paper is implemented, including:

- The liquid type inference algorithm, including constraint generation and iterative weakening.
- lexing/parsing/interpreting a subset of an implicitly-typed ML-like language
- Hindley-Milner type inference
- A-normalization
- Alpha renaming


NOT IMPLEMENTED
---------------

- Specifying Q (the set of liquid type templates) on the command line - to change Q edit the definition in main.rs.
- Higher order polymorphic functions.
- Types -- instantiation or polymorphism, only built-in `bool` and `int` types are available.
- Pending substitutions in liquid type parameters.
- Liquid type inference for arrays, array operations like `sub`, `set` and the built-in function `len`.


QUIRKS
------

Subtraction: - make sure there are spaces around `-` like in `k -
1`. `k-1` doesn't currently work (it is lexed as two tokens, `k` and
`-1`).


LICENSE
-------

Dual-licensed under the MIT and Apache licenses, as is Rust + the
[LALRPOP](https://github.com/nikomatsakis/lalrpop) parser generator.
The lexer is heavily borrowed from LALRPOP.
