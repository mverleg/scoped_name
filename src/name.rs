use ::std::rc::Rc;

use crate::scope::Scope;

#[derive(Debug, Clone)]
pub struct Name {
    scope: Scope,
    data: Rc<InputName>,
}

#[derive(Debug)]
enum InputName {
    Given {
        name: String,
    },
    Anonymous {
        prefix: Option<String>,
        number: usize,
    },
}
