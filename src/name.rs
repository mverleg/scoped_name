use ::std::hash;

use crate::scope::Scope;

/// An identifier, either anonymous or given.
///
/// Instances should be created through `Scope`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) scope: Scope,
    pub(crate) data: InputName,
}

#[derive(Debug, Clone)]
pub(crate) enum InputName {
    /// A given identifier that should not collide.
    Given {
        // Index in the scope's string 'arena'.
        index: usize,
    },
    /// An anonymous identifier, but with a prefix.
    Prefixed {
        // Index in the scope's string 'arena'.
        index: usize,
    },
    /// A totally anonymous identifier.
    Anonymous {
    },
}

/// Only given identifiers can be equal; anonymous ones have no identifying information, so are assumed non-equal.
impl PartialEq for InputName {
    fn eq(&self, other: &Self) -> bool {
        if let InputName::Given { index: first } = self {
            if let InputName::Given { index: second } = other {
                return first == second
            }
        }
        return false;
    }
}

impl Eq for InputName {}

impl hash::Hash for InputName {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            InputName::Given { index } => index.hash(state),
            InputName::Prefixed { index } => index.hash(state),
            InputName::Anonymous {} => 0.hash(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use ::std::collections::HashSet;

    use crate::scope::RootScope;

    use super::*;

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