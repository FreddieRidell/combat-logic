enum PartyNumber<T> {
    Above(T),
    Below(T),
    Highest,
    Lowest,
}

enum Status {
    Bleed,
    Burn,
    Freeze,
    Sleep,
    Paralysis,

    Not(Box<Status>),
    And(Box<Status>, Box<Status>),
    Or(Box<Status>, Box<Status>),
}

enum DamageType {
    Any,

    Poking,
    Fire,
    Bonking,

    Not(Box<DamageType>),
    And(Box<DamageType>, Box<DamageType>),
    Or(Box<DamageType>, Box<DamageType>),
}

enum Query {
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

enum Target {
    Matched,
    Myself,
    Query,
    TargetedBy(Box<Target>),
    Targeting(Box<Target>),
}

enum Strength {
    Weak,
    Medium,
    Strong,
}

enum SpellType {
    Healing,
    Buff,
    DeBuff,
    Attack(DamageType),
}

struct Spell {
    strength: Strength,
    spell_type: SpellType,
}

struct CombatDialougeType {
    Intimidate,
    Pacify,
    Taunt,
}

enum Action {
    Defend,
    HideFrom,
    Item(Item),
    MoveAway,
    MoveTowards,
    MoveTowardsIfNescisary,
    Say(CombatDialougeType),
    Spell(Spell),

    PhyiscalAttack {
        strenght: Strength,
        preffered_damage_type: DamageType,
    },

    AndThen(Box<Action>, Box<Action>),
    AndThenTry(Box<Action>, Box<Action>),
}

struct TargetedAction {
    target: Target,
    action: Action,
}

struct Gambit {
    action: TargetedAction,
    persistance: u64,
    query: Query,
}
