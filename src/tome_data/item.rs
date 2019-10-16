use super::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Deserialize)]
struct ItemEquipSpec {
    label: String,
    slot: ItemSlotSpec,
}

#[derive(Deserialize)]
pub struct ItemSpec {
    name: String,
    level: u64,
    description: OneOrMany<String>,
    one_of_a_kind: Option<bool>,

    equip: Option<ItemEquipSpec>,
    action: Option<Vec<ActionSpec>>,
    passive: Option<OneOrMany<PassiveSpec>>,
}

pub struct ItemInstance {
    item_spec: Rc<ItemSpec>,
}

impl TomeItemInstance<ItemSpec> for ItemInstance {
    fn create_from_spec(spec: &Rc<ItemSpec>) -> Self {
        Self {
            item_spec: spec.clone(),
        }
    }
}

pub type ItemTome = Tome<ItemSpec, ItemInstance>;
