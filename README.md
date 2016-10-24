Logically Qualified Data Types
==============================

This repository contains a rust implementation of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on a core
language λ<sub>L</sub> -- a variant of the λ-calculus with ML-style
polymorphism extended with liquid types.

TODO
----

- [ ] Add interpreter/evaluator
- [ ] Add array intrinsics
- [ ] Add refined::Exp?
- [ ] Generate liquid type constraints
- [ ] Solve constraints in z3
- [ ] Report error or not

LICENSE
-------

Dual-licensed under the MIT and Apache licenses, as is Rust + the
[LALRPOP](https://github.com/nikomatsakis/lalrpop) parser generator.
The lexer is heavily borrowed from LALRPOP.
