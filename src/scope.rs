/// This files defines a root scope, and a tree of scopes below it.
///
/// Scopes can be cloned to pass around whenever borrowing is not convenient; it is
/// small because most data is behind Rc.
///
/// It is designed to avoid excessive allocations, by storing names and sub-scopes
/// contiguously inside the root scope. This does mean that no memory will be
/// reclaimed until the last scope is dropped (which drops the root along with data).

use ::std::cell::RefCell;
use ::std::collections::HashSet;
use ::std::hash;
use ::std::rc::Rc;
use ::std::sync::atomic::AtomicUsize;

use ::lazy_static::lazy_static;
use ::string_interner::StringInterner;

use crate::name::{Name, InputName};
use std::sync::atomic::Ordering::Relaxed;

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootScope {
    // This prevents us from needing
    root_data: Rc<RootScopeData>,
}

#[derive(Debug)]
struct RootScopeData {
    // This number just exists for equality/hash, so that each RootScope is equal
    // if it points to the same RootScopeData. Perhaps this could have been done
    // with pointers, but for now I'm not confident I understand the guarantees
    // around moving and pointers and optimizations well enough for that.
    nr: usize,
    //TODO @mark: names & interning incomplete
    names: RefCell<StringInterner<usize>>,
    scopes: RefCell<Vec<ScopeData>>,
    // I decided to not expose the Scope of the root for now. If it's desired after
    // all, it can be obtained by relying on the convention that scopes[0] is the root.
}

impl RootScope {
    /// Return a new Scope, that holds a reference to a newly created RootScope.
    pub fn new_root() -> Scope {
        let root = RootScope {
            root_data: Rc::new(RootScopeData {
                nr: COUNTER.fetch_add(1, Relaxed),
                names: RefCell::new(StringInterner::new()),
                scopes: RefCell::new(vec![]),
            }),
        };
        root.root_data.scopes.borrow_mut()
            .push(ScopeData {
                parent: None,
                children: vec![],
                names: HashSet::new(),
            });
        Scope {
            root: root.clone(),
            index: 0,
        }
    }

    /// Add new scope data, returning a new scope that refers to it.
    fn add_scope(&self, scope_data: ScopeData) -> Scope {
        let mut scopes = self.root_data.scopes.borrow_mut();
        scopes.push(scope_data);
        Scope {
            root: self.clone(),
            index: scopes.len() - 1,
        }
    }

    /// Add new name data, returning the 'arena' index.
    fn add_text(&self, text: impl Into<String>) -> usize {
        let mut names = self.root_data.names.borrow_mut();
        names.get_or_intern(text.into())
    }

    /// Look up a scope in the arena.
    fn scope_data_at<T>(&self, index: usize, accessor: impl FnOnce(&mut ScopeData) -> T) -> T {
        accessor(&mut self.root_data.scopes.borrow_mut()[index])
    }
}

impl PartialEq for RootScopeData {
    fn eq(&self, other: &Self) -> bool {
        self.nr == other.nr
    }
}

impl Eq for RootScopeData {}

impl hash::Hash for RootScopeData {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.nr.hash(state)
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    root: RootScope,
    index: usize,
}

#[derive(Debug)]
pub struct ScopeData {
    parent: Option<usize>,
    children: Vec<usize>,
    names: HashSet<Name>,
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index &&
            self.root == other.root
    }
}

impl Eq for Scope {}

impl hash::Hash for Scope {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.root.hash(state);
        self.index.hash(state)
    }
}

#[derive(Debug)]
pub struct ScopeChildrenIterator {
    scope: Scope,
    // Note: `child_nr` is the index within `.children`, not within the 'arena'.
    child_nr: usize,
}

impl Iterator for ScopeChildrenIterator {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        // Convert the .children-index into arena-index.
        let child_index = self.scope.root.scope_data_at(self.scope.index,
            |data| data.children.get(self.child_nr).cloned());
        match child_index {
            Some(child_index) => {
                // Create a Scope for that index.
                let scope = Scope {
                    root: self.scope.root.clone(),
                    index: child_index,
                };
                // Make sure the next iteration returns the next child.
                self.child_nr += 1;
                Some(scope)
            },
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct AlreadyExists();

impl Scope {
    pub fn children(&self) -> ScopeChildrenIterator {
        ScopeChildrenIterator {
            scope: self.clone(),
            child_nr: 0,
        }
    }

    /// Connect a child scope to this one.
    pub fn add_child(&self) -> Self {
        // During this method, the state is not consistent.
        // Step 1: add the new scope data to the root 'arena'.
        let child_scope = {
            self.root.add_scope(ScopeData {
                parent: Some(self.index),
                children: vec![],
                names: HashSet::new(),
            })
        };
        // Step 2: register that this is a child.
        self.root.scope_data_at(self.index,
            |data| data.children.push(child_scope.index));
        child_scope
    }

    /// Register a named identifier in this scope, failing if it is already registered.
    pub fn add_named(&self, name: impl Into<String>) -> Result<Name, AlreadyExists> {
        // During this method, the state is not consistent.
        // Step 1: add the text to the root 'arena'.
        let name_index = {
            self.root.add_text(name)
        };
        // Step 2: create the name instance.
        let name = Name {
            scope: (*self).clone(),
            data: InputName::Given {
                index: name_index,
            }
        };
        // Step 3: register this name on the scope.
        let is_new = self.root.scope_data_at(self.index,
            |data| data.names.insert(name.clone()));
        // Step 4: return the name only if it was a new name.
        if !is_new {
            return Err(AlreadyExists())
        }
        Ok(name)
    }

    /// Register an anonymous identifier in this scope, possibly with a prefix.
    pub fn add_anonymous<S: Into<String>>(&mut self) -> Name {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_root() {
        let mut root = RootScope::new_root();
        let child1 = root.add_child();
        let mut child2 = root.add_child();
        let child2a = child2.add_child();
        let child2b = child2.add_child();
    }

    #[test]
    fn add_named_unique() {
        let mut root = RootScope::new_root();
        let name1 = root.add_named("hello").unwrap();
        let name2 = root.add_named("bye").unwrap();
        let mut child1 = root.add_child();
        let name3 = child1.add_named("nihao").unwrap();
    }

    #[test]
    fn add_named_duplicate() {
        let mut root = RootScope::new_root();
        let name1 = root.add_named("hello").unwrap();
        let mut child1 = root.add_child();
        // This is shadowing (in different scope) and is allowed:
        let name2 = child1.add_named("hello").unwrap();
        // This is a duplicate (in the same scope) and should fail:
        let failure = child1.add_named("hello").unwrap_err();
    }
}
