use super::*;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ItemSpec {
    name: String,
    description: OneOrMany<String>,
    one_of_a_kind: bool,
}

pub struct ItemTome {
    items: HashMap<Id, Rc<ItemSpec>>,
}

pub struct ItemInstance {
    spec: Rc<ItemSpec>,
}
