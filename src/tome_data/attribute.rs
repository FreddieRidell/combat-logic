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

    FlySpeed,
    LandSpeed,
    SwimSpeed,
}
