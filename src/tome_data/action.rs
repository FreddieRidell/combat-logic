use super::*;

pub enum Action {
    Defend,
    HideFrom,
    MoveAway,
    MoveTowards,
    MoveTowardsIfNescisary,

    Say(CombatDialougeType),
    Spell(Spell),

    PhyiscalAttack {
        strenght: Strength,
        preffered_damage_type: DamageType,
    },
}
