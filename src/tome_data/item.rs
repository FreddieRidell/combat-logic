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
    rarity: u64,
    level: u64,
    description: OneOrMany<String>,
    one_of_a_kind: Option<bool>,

    equip: Option<ItemEquip>,
    action: Option<Vec<Action>>,
    passive: Option<OneOrMany<PassiveBuff>>,
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
