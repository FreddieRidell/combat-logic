use super::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Attribute {
    Str,
    Con,
    Dex,
    Wis,
    Int,
    Cha,

    MaxHealth,
    Health,
    ArmorClass,
    ActionsPerTurn,

    FlySpeed,
    LandSpeed,
    SwimSpeed,
    EncumberanceWeight,
}
