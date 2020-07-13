use ::std::collections::HashMap;
use ::std::rc::Rc;

use crate::name::Name;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct Scope {
    data: Rc<ScopeContainer>,
}

#[derive(Debug)]
struct ScopeContainer {
    parent: Option<Scope>,
    children: Vec<Scope>,
    names: HashMap<String, Name>,
}

impl Scope {
    pub fn new_root() -> Self {
        Scope {
            data: Rc::new(ScopeContainer {
                parent: None,
                children: vec![],
                names: HashMap::new(),
            })
        }
    }

    pub fn add_child(&mut self) -> Self {
        let parent: Scope = (*self).clone();
        let child = Scope {
            data: Rc::new(ScopeContainer {
                parent: Some(parent),
                children: vec![],
                names: HashMap::new(),
            })
        };
        self.data.borrow_mut().children.push(child.clone());
        child
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_root() {
        Scope::new_root();
    }
}
