use super::*;

pub enum Query {
    Myself,
    Ally,
    Foe,
    InRange,
    InRangeAfterMoving,
    DamageDealt(PartyNumber<u64>),
    TargetedBy(PartyNumber<u64>),
    Health(PartyNumber<i64>),
    Level(PartyNumber<u64>),
    Mana(PartyNumber<i64>),
    Name(String),
    Resistant(DamageType),
    Status(Status),
    Weakness(DamageType),
    WithinDistance(Box<Query>, f64),
    Not(Box<Query>),
    And(Box<Query>, Box<Query>),
    Or(Box<Query>, Box<Query>),
}
