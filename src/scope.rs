
/// This files defines a root scope, and a tree of scopes below it.
///
/// Scopes can be cloned to pass around whenever borrowing is not convenient; it is
/// small because most data is behind Rc.
///
/// It is designed to avoid excessive allocations, by storing names and sub-scopes
/// contiguously inside the root scope. This does mean that no memory will be
/// reclaimed until the last scope is dropped (which drops the root along with data).

use ::std::cell::RefCell;
use ::std::rc::Rc;

use ::string_interner::StringInterner;

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
        let root = RootScope {
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
    fn scope_data_at<T>(&self, index: usize, accessor: impl FnOnce(&mut ScopeData) -> T) -> T {
        accessor(&mut self.root_data.scopes.borrow()[index])
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
struct ScopeChildrenIterator {
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

impl Scope {
    pub fn children(&self) -> ScopeChildrenIterator {
        ScopeChildrenIterator {
            scope: self.clone(),
            child_nr: 0,
        }
    }

    pub fn add_child(&mut self) -> Self {
        // During this method, the state is not consistent.
        // Step 1: add the new scope data to the root 'arena'.
        let child_scope = self.root.add_scope(ScopeData {
            parent: Some(self.index),
            children: vec![],
        });
        // Step 2: register that this is a child.
        self.root.scope_data_at(self.index,
            |data| data.children.push(child_scope.index));
        // Step 3: create and return scope.
        child_scope
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
