use super::*;
use std::collections::HashMap;
use std::rc::Rc;

enum ItemAction {
    Attack {
        label: String,
        damage: String,
        accuracy: String,
        damage_type: OneOrMany<DamageType>,
        report: String,
    },

    TempoaryBuff {
        label: String,
        attribute: Attribute,
        change: DiceExpression,
        duration: u64,
        report: String,
    },
}

pub enum ItemSlot {
    OnHand,
    OffHand,
    Head,
    LeftFingers,
    RightFingers,
    Torso,
    Legs,
    Arms,
}

struct ItemEquip {
    label: String,
    slot: OneOrMany<ItemSlot>,
}

pub struct ItemSpec {
    name: String,
    description: OneOrMany<String>,
    one_of_a_kind: Option<bool>,

    equip: Option<ItemEquip>,
    action: Vec<ItemAction>,
}

pub struct ItemTome {
    items: HashMap<Path, Rc<ItemSpec>>,
}
