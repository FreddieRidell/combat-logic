use super::*;
use serde_derive::Deserialize;

/// A certain amount of damage, of a ceartin type, dealt to ther action's target
#[derive(Deserialize)]
pub struct ActionEffectDamage {
    damage: DiceExpression,
    damage_type: OneOrMany<DamageType>,
}

/// A ceartina amount of healing, given to the action's target
#[derive(Deserialize)]
pub struct ActionEffectHeal {
    value: DiceExpression,
}

/// Adds a status to the target, with a ceartain duration
#[derive(Deserialize)]
pub struct ActionEffectAddStatus {
    status: StatusSpec,
    duration: DurationSpec,
}

/// Adds a buff to the target, with a character duration
#[derive(Deserialize)]
pub struct ActionEffectBuff {
    buff: BuffSpec,
    duration: DurationSpec,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ActionEffectSpec {
    Damage(ActionEffectDamage),
    Heal(ActionEffectHeal),
    AddStatus(ActionEffectAddStatus),
    Buff(ActionEffectBuff),
    DeBuf(ActionEffectBuff),
}

/// unlike DnD, there's no saving throws here. The attacker always rolls, they can just roll
/// against something other than ArmourClass, this means to can make spells that mean the actor
/// must roll against the target's Wis, for instance.
#[derive(Deserialize)]
pub struct ToHit {
    accuracy: DiceExpression,
    #[serde(default = "against_attribute_default")]
    against_attribute: Attribute,
}

fn against_attribute_default() -> Attribute {
    Attribute::ArmorClass
}

/// A description of an action that can be taken by a character, targeted to any character.
/// There may be certain limitations, like range or `must_target`.
/// Multiple effects can be triggered by any given action. For example, an action could cause
/// damage to a target, or could heal, add a status, add a buff, and move the target, all at once.
///
/// Although Movement and Speech are taken as actions, they are not described by `ActionSpecs`, as
/// they are something that every character has access to.
#[derive(Deserialize)]
pub struct ActionSpec {
    effect: Vec<ActionEffectSpec>,
    label: String,
    range: Range,
    report: OneOrMany<String>,
    time: Option<DurationSpec>,
    #[serde(flatten)]
    to_hit: Option<ToHit>,
}
