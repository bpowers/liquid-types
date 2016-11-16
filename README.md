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

- [ ] Get examples/sum.ml to give precise for sum of: k:int->{v:int| 0 <= v && k <= v}
- [ ] Builtin functions - in the builtin type environment and evaluation context
- [ ] Allow specifying Q on commandline (like Q = {0 ≤ ν; ν < len a} // from pg. 21)

NOT IMPLEMENTED
---------------

- Polymorphic Types


LICENSE
-------

Dual-licensed under the MIT and Apache licenses, as is Rust + the
[LALRPOP](https://github.com/nikomatsakis/lalrpop) parser generator.
The lexer is heavily borrowed from LALRPOP.
