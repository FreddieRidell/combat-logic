use super::*;

pub struct Character {
    id: Id,
    name: String,

    on_hand: ItemInstance,
    off_hand: ItemInstance,
}
