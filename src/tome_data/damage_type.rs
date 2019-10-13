pub enum DamageType {
    Any,
    Poking,
    Fire,
    Bonking,
    Not(Box<DamageType>),
    And(Box<DamageType>, Box<DamageType>),
    Or(Box<DamageType>, Box<DamageType>),
}
