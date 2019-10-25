
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum ItemSlotSpec {
    Arms,
    Feet,
    Ring,
    Gloves,
    Head,
    Legs,
    Neck,
    OffHand,
    OnHand,
    EitherHand,
    TwoHand,
    Torso,
}

pub enum ItemSlot {
    Arms,
    Feet,
    RingLeft,
    RingRight,
    Gloves,
    Head,
    Legs,
    Neck,
    OffHand,
    OnHand,
    Torso,
}
