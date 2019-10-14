use super::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Deserialize)]
pub enum ItemSlot {
    Arms,
    Feet,
    FingersLeft,
    FingersRight,
    Head,
    Legs,
    Neck,
    OffHand,
    OnHand,
    Torso,
}

#[derive(Deserialize)]
struct ItemEquip {
    label: String,
    slot: OneOrMany<ItemSlot>,
}

#[derive(Deserialize)]
pub struct ItemSpec {
    name: String,
    description: OneOrMany<String>,
    one_of_a_kind: Option<bool>,

    equip: Option<ItemEquip>,
    action: Option<Action>,
}

pub struct ItemInstance {
    item_spec: Rc<ItemSpec>,
}

impl TomeInstance<ItemSpec> for ItemInstance {
    fn create_from_spec(spec: &Rc<ItemSpec>) -> Self {
        Self {
            item_spec: spec.clone(),
        }
    }
}

pub type ItemTome = Tome<ItemSpec, ItemInstance>;
