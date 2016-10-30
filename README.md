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

- [ ] Pass separate k_alloc and env (where env has both in-scope α-renamed variables AND path constraints) to cons
- [ ] Generate constraints
- [ ] Does k get instantiated as set of all possible type templates that match shape w/ Stars filled in by program variables, or as just 'k'
      (what would I need to instantiate it Q + closure?  What would I need to delay it -- just closure/env?  Check if well-formed?
- [ ] Cons does type inference -- can't just convert HM(e) -> Liquid(e), need to generate constraints as well.
- [ ] Allow specifying Q on commandline (like Q = {0 ≤ ν; ν < len a} // from pg. 21)
- [ ] Uninterpreted function terms in Z3
- [ ] fix: "expr typechecks if there is an appropriate liquid type that can be instantiated for the α in the polymorphic type of fix; intuitively, this liquid type corresponds to the type of the recursive function f"
- [ ] α-rename variables
- [ ] Does Γ include both Dependent Types AND Liquid Types?
- [ ] Builtin functions - in the builtin type environment and evaluation context
- [ ] Divide by zero
- [ ] Add refined::Expr -- Just S(E) instead of S(E ∪ (ϴ·k)) -- necessary?
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
