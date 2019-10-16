use super::*;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Character {
    name: String,

    equiped: HashMap<ItemSlot, Rc<ItemSpec>>,
}
