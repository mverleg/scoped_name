use ::std::hash;

use crate::scope::Scope;

/// An identifier, either anonymous or given.
///
/// Instances should be created through `Scope`.
#[derive(Debug, Clone)]
pub struct Name {
    pub(crate) scope: Scope,
    pub(crate) data: InputName,
}

/// A given identifier that should not collide within a scope.
#[derive(Debug, Clone)]
pub struct GivenName {
    // Index in the scope's string 'arena'.
    pub(crate) index: usize,
}

/// /// An anonymous identifier, optionally with a prefix.
#[derive(Debug, Clone)]
pub struct AnonName {
    // Index in the scope's string 'arena'.
    // Empty string is used to mean 'no prefix'.
    pub(crate) index: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum InputName {
    Given(GivenName),
    Anonymous(AnonName),
}

/// Only given identifiers can be equal; anonymous ones have no identifying information, so are assumed non-equal.
impl PartialEq for GivenName {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for GivenName {}

impl hash::Hash for GivenName {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}


#[cfg(test)]
mod mixed {
    use ::std::collections::HashSet;

    use crate::scope::RootScope;

    use super::*;

    #[test]
    fn different_variant() {
        unimplemented!();
        // let mut scope = RootScope::new_root();
        // let name1 = scope.add_named("hello").unwrap();
        // let name2 = scope.add_named("world").unwrap();
        // assert_eq!(name1, name1);
        // assert_eq!(name2, name2.clone());
        // assert_ne!(name1, name2);
        // let mut set = HashSet::new();
        // assert!(set.insert(name1.clone()));
        // assert!(set.insert(name2));
        // assert!(!set.insert(name1));
    }
}

//TODO @mark: anon tests?

#[cfg(test)]
mod given {
    use ::std::collections::HashSet;

    use crate::scope::RootScope;

    use super::*;

    #[test]
    fn different_variant() {
        unimplemented!();
        // let mut scope = RootScope::new_root();
        // let name1 = scope.add_named("hello").unwrap();
        // let name2 = scope.add_named("world").unwrap();
        // assert_eq!(name1, name1);
        // assert_eq!(name2, name2.clone());
        // assert_ne!(name1, name2);
        // let mut set = HashSet::new();
        // assert!(set.insert(name1.clone()));
        // assert!(set.insert(name2));
        // assert!(!set.insert(name1));
    }

    #[test]
    fn different_text() {
        let mut scope = RootScope::new_root();
        let name1 = scope.add_named("hello").unwrap();
        let name2 = scope.add_named("world").unwrap();
        assert_eq!(name1, name1);
        assert_eq!(name2, name2.clone());
        assert_ne!(name1, name2);
        let mut set = HashSet::new();
        assert!(set.insert(name1.clone()));
        assert!(set.insert(name2));
        assert!(!set.insert(name1));
    }

    #[test]
    fn different_scope() {
        let mut scope = RootScope::new_root();
        let child1 = scope.add_child();
        let name1 = child1.add_named("hello").unwrap();
        let child2 = scope.add_child();
        let name2 = child2.add_named("hello").unwrap();
        assert_eq!(name1, name1);
        assert_eq!(name2, name2.clone());
        assert_ne!(name1, name2);
        let mut set = HashSet::new();
        assert!(set.insert(name1.clone()));
        assert!(set.insert(name2));
        assert!(!set.insert(name1));
    }

    #[test]
    fn different_root() {
        let mut scope1 = RootScope::new_root();
        let mut scope2 = RootScope::new_root();
        let name1 = scope1.add_named("hello").unwrap();
        let name2 = scope2.add_named("hello").unwrap();
        assert_eq!(name1, name1);
        assert_eq!(name2, name2.clone());
        assert_ne!(name1, name2);
        let mut set = HashSet::new();
        assert!(set.insert(name1.clone()));
        assert!(set.insert(name2));
        assert!(!set.insert(name1));
    }
}