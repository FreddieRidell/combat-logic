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
    buff: BuffSpec,
    lasts_for: u64,
    label: String,
    must_target: Option<TargetRequirementSpec>,
    report: OneOrMany<String>,
    time_to_complete: u64,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ItemActionSpec {
    Attack(AttackActionSpec),
    Buff(BuffActionSpec),
    Consume(BuffActionSpec),
}
