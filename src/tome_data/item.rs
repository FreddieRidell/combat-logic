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
    identified_name: Option<String>,
    value: u64,
    tagline: OneOrMany<String>,
    flavour: OneOrMany<String>,
    one_of_a_kind: Option<bool>,

    action: Option<Vec<ActionSpec>>,
    consume: Option<ActionSpec>,
    equip: Option<ItemEquipSpec>,
    passive: Option<OneOrMany<BuffSpec>>,
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
