Logically Qualified Data Types
==============================

This repository contains a rust implementation of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on a core
language λ<sub>L</sub> -- a variant of the λ-calculus with ML-style
polymorphism extended with liquid types.

TODO
----

- [ ] Shape(\Gamma)
- [ ] Add refined::Expr
- [ ] Builtin functions - in the builtin type environment and evaluation context
- [ ] lower from explicit::Expr => refined::Expr w/ liquid type variables
- [ ] Generate liquid type constraints
- [ ] Solve constraints in z3
- [ ] Report error or not
- [ ] Replace liquid type variables w/ precise refinements
- [ ] verify no liquid type vars remain
- [ ] Polymorphic constraints for HM?  or does that already happen if I don't make sure metavars are removed?

NOT IMPLEMENTED
---------------

- Polymorphic Types
- A-Normalization (which means our refinements aren't as refined as they could be)


LICENSE
-------

Dual-licensed under the MIT and Apache licenses, as is Rust + the
[LALRPOP](https://github.com/nikomatsakis/lalrpop) parser generator.
The lexer is heavily borrowed from LALRPOP.
