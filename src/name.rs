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
