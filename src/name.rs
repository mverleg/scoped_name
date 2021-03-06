use crate::scope::Scope;
use ustr::Ustr;

/// An identifier, either anonymous or given.
///
/// Instances should be created through `Scope`.
#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub(crate) scope: Scope,
    pub(crate) data: InputName,
}

impl Name {
    pub fn unwrap_given(self) -> GivenName {
        match self.data {
            InputName::Given(given) => given,
            InputName::Anonymous(_) => panic!("unwrap_given on an anonymous name"),
        }
    }
}

/// A given identifier that should not collide within a scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GivenName {
    // Index in the scope's string 'arena'.
    pub(crate) name: Ustr,
}

/// An anonymous identifier, optionally with a prefix.
#[derive(Debug, Clone)]
pub struct AnonName {
    // Index in the scope's string 'arena'.
    // Empty string is used to mean 'no prefix'.
    pub(crate) name: Ustr,
}

/// Only given identifiers can be equal; anonymous ones have no identifying information, so are assumed non-equal.
impl PartialEq for AnonName {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InputName {
    Given(GivenName),
    Anonymous(AnonName),
}

#[cfg(test)]
mod mixed {
    use crate::scope::RootScope;

    #[test]
    fn different_variant() {
        let scope = RootScope::new_root();
        let name1 = scope.add_prefixed("hello");
        let name2 = scope.add_named("hello").unwrap();
        assert_ne!(name1, name1);
        assert_eq!(name2, name2.clone());
        assert_ne!(name1, name2);
    }
}

#[cfg(test)]
mod anonymous {
    use crate::scope::RootScope;

    #[test]
    fn prefixed_ne() {
        let scope = RootScope::new_root();
        let name1 = scope.add_prefixed("hello");
        let name2 = scope.add_prefixed("hello");
        assert_ne!(name1, name1);
        assert_ne!(name2, name2.clone());
        assert_ne!(name1, name2);
    }

    #[test]
    fn anonymous_ne() {
        let scope = RootScope::new_root();
        let name1 = scope.add_anonymous();
        let name2 = scope.add_anonymous();
        assert_ne!(name1, name1);
        assert_ne!(name2, name2.clone());
        assert_ne!(name1, name2);
    }
}

#[cfg(test)]
mod given {
    use ::std::collections::HashSet;

    use crate::scope::RootScope;

    #[test]
    fn different_text_ne() {
        let scope = RootScope::new_root();
        let name1 = scope.add_named("hello").unwrap();
        let name2 = scope.add_named("world").unwrap();
        assert_eq!(name1, name1);
        assert_eq!(name2, name2.clone());
        assert_ne!(name1, name2);
    }

    #[test]
    fn different_text_hash() {
        let scope = RootScope::new_root();
        let name1 = scope.add_named("hello").unwrap().unwrap_given();
        let name2 = scope.add_named("world").unwrap().unwrap_given();
        let mut set = HashSet::new();
        assert!(set.insert(name1.clone()));
        assert!(set.insert(name2));
        assert!(!set.insert(name1));
    }

    #[test]
    fn different_scope_ne() {
        let scope = RootScope::new_root();
        let child1 = scope.add_child();
        let name1 = child1.add_named("hello").unwrap();
        let child2 = scope.add_child();
        let name2 = child2.add_named("hello").unwrap();
        assert_eq!(name1, name1);
        assert_eq!(name2, name2.clone());
        assert_ne!(name1, name2);
    }

    #[test]
    fn different_root_ne() {
        let scope1 = RootScope::new_root();
        let scope2 = RootScope::new_root();
        let name1 = scope1.add_named("hello").unwrap();
        let name2 = scope2.add_named("hello").unwrap();
        assert_eq!(name1, name1);
        assert_eq!(name2, name2.clone());
        assert_ne!(name1, name2);
    }
}
