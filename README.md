Logically Qualified Data Types
==============================

This repository contains a rust implementation of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on a core
language λ<sub>L</sub> -- a variant of the λ-calculus with ML-style
polymorphism extended with liquid types.

QUIRKS
------

Subtraction: - make sure there are spaces around `-` like in `k - 1`. `k-1` doesn't currently work.


TODO
----

- [ ] α-rename explicit -> implicit + Shape(Γ)
- [ ] Generate liquid type constraints from implicit + Shape(Γ)
- [ ] Solve constraints in z3 (what about uninterpreted function terms in Z3?)
- [ ] Builtin functions - in the builtin type environment and evaluation context
- [ ] Allow specifying Q on commandline (like Q = {0 ≤ ν; ν < len a} // from pg. 21)
- [ ] fixpoint: "expr typechecks if there is an appropriate liquid type that can be instantiated for the α in the polymorphic type of fix; intuitively, this liquid type corresponds to the type of the recursive function f"
- [ ] Divide by zero
- [ ] Report error or not
- [ ] Replace liquid type variables w/ precise refinements
- [ ] verify no liquid type vars remain
- [ ] Polymorphic constraints for HM?  or does that already happen if I don't make sure metavars are removed?
- [ ] more tests: add test that implicit -> explicit -> implicit is correct (mod α-renaming)

NOT IMPLEMENTED
---------------

- Polymorphic Types
- A-Normalization (which means our refinements aren't as refined as they could be)


LICENSE
-------

Dual-licensed under the MIT and Apache licenses, as is Rust + the
[LALRPOP](https://github.com/nikomatsakis/lalrpop) parser generator.
The lexer is heavily borrowed from LALRPOP.
