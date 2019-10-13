use super::*;

pub enum SpellType {
    Healing,
    Buff,
    DeBuff,
    Attack(DamageType),
}
