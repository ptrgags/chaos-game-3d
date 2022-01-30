## Nothing to see here...

This directory contains some Python 3 scripts that use the `clifford` library
to help me test my Rust implementation. The scripts just perform certain
operations and print the result, so I can compare with the unit tests
in `src/half_multivector.rs`. Nothing really interesting.

## You're still here?

Wow. Okay, well one thing that might be useful, the `cga3.py` script sets
up `clifford` for 3D Conformal Geometric Algebra, and defines some functions
for the various types of transformations. Just start a python interpreter
in this directory and `from cga3 import *`.