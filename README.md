Logically Qualified Data Types
==============================

This repository contains a rust implementation of [liquid
types](http://goto.ucsd.edu/~rjhala/liquid/liquid_types.pdf) on a core
language λ<sub>L</sub> -- a variant of the λ-calculus with ML-style
polymorphism extended with liquid types.


NOT IMPLEMENTED
---------------

- Specifying Q on the command line - to change Q edit the definition in main.rs.
- Types -- instantiation or polymorphism, only built-in `bool` and `int` types are available.
- Some built-in functions like `len`, and precise dependent types for `intarray` `sub`.


QUIRKS
------

Subtraction: - make sure there are spaces around `-` like in `k - 1`. `k-1` doesn't currently work.


LICENSE
-------

Dual-licensed under the MIT and Apache licenses, as is Rust + the
[LALRPOP](https://github.com/nikomatsakis/lalrpop) parser generator.
The lexer is heavily borrowed from LALRPOP.
