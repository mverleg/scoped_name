
Scoped name
===============================

`scoped_name` is a small Rust library meant for tracking identifiers in nested scopes.

This can be used as part of programming tools, to model variables in different code blocks.

There are input variables, as they appear in the source code. They:

* May have a given name, which can be looked up efficiently, or
* May be anonymous, possibly with a prefix, which isn't looked up.
* May shadow variables in higher scopes.

These are then transformed to identifiers in the generated code. They:

* Do not shadow parent scope names (but sibling duplicates are okay).
* Are not unnecessarily long (possibly even close-to-optimally short).
* Somewhat resemble the input names, if given.

In both cases, things have one name, and scope-name combinations refer to one thing.

Status
-------------------------------

The input variable names are mostly done, computing output variable names is unfinished. Overall not production-ready.

