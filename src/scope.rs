
/// This files defines a root scope, and a tree of scopes below it.
///
/// Scopes can be cloned to pass around whenever borrowing is not convenient; it is
/// small because most data is behind Rc.
///
/// It is designed to avoid excessive allocations, by storing names and sub-scopes
/// contiguously inside the root scope. This does mean that no memory will be
/// reclaimed until the last scope is dropped (which drops the root along with data).

use ::std::borrow::BorrowMut;
use ::std::cell::RefCell;
use ::std::collections::HashMap;
use ::std::rc::Rc;

use ::string_interner::StringInterner;

use crate::name::Name;

#[derive(Debug, Clone)]
pub struct RootScope {
    // This prevents us from needing
    root_data: Rc<RootScopeData>,
}

#[derive(Debug)]
struct RootScopeData {
    //TODO @mark: names & interning incomplete
    names: RefCell<StringInterner<usize>>,
    scopes: RefCell<Vec<ScopeData>>,
    // I decided to not expose the Scope of the root for now. If it's desired after
    // all, it can be obtained by relying on the convention that scopes[0] is the root.
}

impl RootScope {
    /// Return a new Scope, that holds a reference to a newly created RootScope.
    pub fn new_root() -> Scope {
        let mut root = RootScope {
            root_data: Rc::new(RootScopeData {
                names: RefCell::new(StringInterner::new()),
                scopes: RefCell::new(vec![]),
            }),
        };
        root.root_data.scopes.borrow_mut()
            .push(ScopeData {
                parent: None,
                children: vec![],
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
            index: self.root_data.scopes.borrow().len() - 1,
        }
    }

    /// Look up a scope in the arena.
    fn scope_data_at<T>(&self, scope: &Scope, getter: impl FnOnce(&ScopeData) -> T) -> T {
        getter(&self.root_data.scopes.borrow()[scope.index])
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    root: RootScope,
    index: usize,
}

#[derive(Debug, Clone)]
pub struct ScopeData {
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug)]
struct ScopeIterator {
    scope: Scope,
    child_index: usize,
}

impl Iterator for ScopeIterator {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        let children = self.scope.root.scope_data_at(&self.scope, |data| data.children);
        if self.child_index >= children.len() {
            return None
        }
        let scope = Scope {
            root: self.scope.root.clone(),
            index: children[self.child_index],
        };
        self.child_index += 1;
        Some(scope)
    }
}

impl Scope {
    pub fn children(&self) -> ScopeIterator {
        unimplemented!()
    }

    pub fn add_child(&mut self) -> Self {
        // During this method, the state is not consistent.
        // Step 1: add the new scope data to the root 'arena'.
        let child = self.root.add_scope(ScopeData {
            parent: Some(self.index),
            children: vec![],
        });
        // Step 2: register that this is a child.


        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_root() {
        RootScope::new_root();
    }
}
