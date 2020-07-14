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
    // Note: this is only optional to be able to initialize the circular reference.
    // After initialization it should always be Some (accessible with .scope() method).
    scope: Option<Scope>,
}

#[derive(Debug)]
struct RootScopeData {
    //TODO @mark: names & interning incomplete
    names: RefCell<StringInterner<usize>>,
    scopes: RefCell<Vec<ScopeData>>,
}

impl RootScope {
    pub fn new() -> Self {
        // Note: while this constructor is running, the object is invalid state.
        let mut root = RootScope {
            root_data: Rc::new(RootScopeData {
                names: RefCell::new(StringInterner::new()),
                scopes: RefCell::new(vec![]),
            }),
            scope: None,
        };
        let scope_data = ScopeData {
            parent: None,
            children: vec![],
        };
        root.root_data.scopes.borrow_mut().push(scope_data);
        root.scope = Some(Scope {
            root: root.clone(),
            index: 0,
        });
        //TODO @mark: check that the scope resolves
        debug_assert!(root.scope());
        root
    }

    pub fn scope(&self) -> &Scope {
        &self.scope.unwrap()
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

impl Scope {
    pub fn add_child(&self) -> Self {
        unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_root() {
        RootScope::new();
    }
}
