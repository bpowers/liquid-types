Logically Qualified Data Types
==============================

This repository contains a rust implementation of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on a core
language λ<sub>L</sub> -- a variant of the λ-calculus with ML-style
polymorphism extended with liquid types.

TODO
----

- [ ] make Expr generic w.r.t. Type
- [ ] Add interpreter/evaluator
- [ ] Add intarray intrinsics, constructor array(len, fill_val), a[i] get(a, i), a[i] = v set(a, i, v)
- [ ] Add refined::Expr
- [ ] lower from explicit::Expr => refined::Expr w/ liquid type variables
- [ ] Generate liquid type constraints
- [ ] Solve constraints in z3
- [ ] Report error or not
- [ ] Replace liquid type variables w/ precise refinements
- [ ] verify no liquid type vars remain
- [ ] Polymorphic constraints for HM?  or does that already happen if I don't make sure metavars are removed?

LICENSE
-------

Dual-licensed under the MIT and Apache licenses, as is Rust + the
[LALRPOP](https://github.com/nikomatsakis/lalrpop) parser generator.
The lexer is heavily borrowed from LALRPOP.
