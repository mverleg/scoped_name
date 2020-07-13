
/// This code is for having names which belong to scopes.
///
/// There are input variables, as they appear in the source code. They:
/// * May have a given name, which can be looked up efficiently, or
/// * May be anonymous, possibly with a prefix, which isn't looked up.
/// * May shadow variables in higher scopes.
///
/// These are then transformed to identifiers in the generated code. They:
/// * Do not shadow parent scope names (but sibling duplicates are okay).
/// * Are not unnecessarily long (possibly even close-to-optimally short).
/// * Somewhat resemble the input names, if given.
///
/// In both cases, things have one name, and scope-name combinations refer to one thing.

mod name;
mod scope;

//TODO @mark: add interning
