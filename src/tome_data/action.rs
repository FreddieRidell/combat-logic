use super::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ActionAttack {
    label: String,
    damage: DiceExpression,
    accuracy: DiceExpression,
    damage_type: OneOrMany<DamageType>,
    report: String,
}

#[derive(Deserialize)]
pub struct ActionBuff {
    label: String,
    attribute: Attribute,
    value: DiceExpression,
    duration: Option<u64>,
    consumes: Option<bool>,
    report: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    Attack(ActionAttack),
    Buff(ActionBuff),
}
