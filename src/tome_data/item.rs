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
    /// The name that renders by default for an item
    name: String,
    /// If the item can be identified, this will contain its true name.
    identified_name: Option<String>,
    /// The point-buy value of an item, used to derive GP and level
    value: u64,
    /// The one scentence description of the item, each instance picks a fixed option from the
    /// OneOrMany
    tagline: OneOrMany<String>,
    /// Any extra description text, chosen at random every time the item is viewed
    flavour: OneOrMany<String>,
    /// If true, the item can not ever be generated in-world
    #[serde(default = "default_bool")]
    one_of_a_kind: bool,
    /// Information about how/where to equip the item, if it is equipable. If `equip = Some(_)`
    /// then `action`, `consume`, and `passive` will only be accessable when the item is equiped
    equip: Option<ItemEquipSpec>,
    /// The weight in kg, of the item
    #[serde(default = "default_u64")]
    weight: u64,
    /// Things that can be done with the item.
    action: Option<Vec<ActionSpec>>,
    /// Single use actions that consume the item, removing it from the actor's inventory
    consume: Option<ActionSpec>,
    /// Constant buffs that apply for as long as the item is equiped
    passive: Option<OneOrMany<BuffSpec>>,
}

fn default_u64() -> u64 {
    0
}
fn default_bool() -> bool {
    false
}

impl TomeSpec for ItemSpec {}

pub struct ItemInstance {
    item_spec: Rc<ItemSpec>,
}
impl TomeInstance for ItemInstance {}

impl CreateFromInstance<ItemSpec> for ItemInstance {
    fn create_from_spec(_: &Tome, spec: &Rc<ItemSpec>) -> Self {
        Self {
            item_spec: spec.clone(),
        }
    }
}
