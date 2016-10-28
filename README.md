Logically Qualified Data Types
==============================

This repository contains a rust implementation of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on a core
language λ<sub>L</sub> -- a variant of the λ-calculus with ML-style
polymorphism extended with liquid types.

TODO
----

- [ ] Does Γ include both Dependent Types AND Liquid Types?
- [ ] Builtin functions - in the builtin type environment and evaluation context
- [ ] Add refined::Expr
- [ ] lower from explicit::Expr => refined::Expr w/ liquid type variables
- [ ] Generate liquid type constraints
- [ ] Weirdness/unexpected parse of (k-1) (App('k', '-1')) instead of (Op2(-, 'k', 1))
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
