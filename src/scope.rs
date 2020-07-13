
#[derive(Debug, Clone)]
pub struct Scope {

}

struct ScopeContainer {
    parent: Scope,
    names: HashMap<String, Name>,
}
