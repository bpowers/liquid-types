Logically Qualified Data Types
==============================

This repository contains an implementation in Rust of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on a core
language λ<sub>L</sub> -- a variant of the λ-calculus with ML-style
polymorphism extended with liquid types.


BUILD
-----

This projects requires building with a *nightly* version of rust, as
it uses features like `box_syntax` that aren't in release builds.  It
also requires having the [z3 theorem
prover](https://github.com/Z3Prover/z3/wiki) on your path.

A Dockerfile is included to ease development and testing.  To get a
shell in a docker container suitable for testing, first [install
docker](https://docs.docker.com/engine/getstarted/step_one/) and then
run:

```sh
$ make docker
```

This will download an image with rust nightly and z3, build the
project, and give you a bash shell you can test examples in by running
the `liquid-types` binary.

For example, `examples/max3.ml` defines a function max and applies it
to two arguments:

```ML
let max = fun x -> fun y -> if x > y then x else y in max 1 -3
```


RUN
---

In the docker shell, you can infer liquid types for this program by
running:

```sh
$ liquid-types examples/max3.ml
```

This will spit out debugging information about the liquid environment
(Γ) and subtyping constraints, along with the refined types produced
by running liquid type inference.  The type inferred by max is:

```
max_a1:	F(x_a2: {ν: Int | 0 ≤ ν} → F(y_a3: {ν: Int | true} → {ν: Int | 0 ≤ ν ∧ x_a2 ≤ ν ∧ y_a3 ≤ ν}))
```

(you can ignore the `_a$N` suffix on variables, which is added during A-normalization)


NOT IMPLEMENTED
---------------

- Specifying Q on the command line - to change Q edit the definition in main.rs.
- Types -- instantiation or polymorphism, only built-in `bool` and `int` types are available.
- Some built-in functions like `len`, and precise dependent types for `intarray` `sub`.


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
