use super::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct AttackActionSpec {
    label: String,
    damage: DiceExpression,
    accuracy: DiceExpression,
    damage_type: OneOrMany<DamageType>,
    range: Range,
    report: OneOrMany<String>,
}

#[derive(Deserialize)]
pub struct BuffActionSpec {
    label: String,
    attribute: Attribute,
    must_target: Option<TargetRequirementSpec>,
    value: DiceExpression,
    duration: Option<u64>,
    report: OneOrMany<String>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ItemActionSpec {
    Attack(AttackActionSpec),
    Buff(BuffActionSpec),
    Consume(BuffActionSpec),
}
