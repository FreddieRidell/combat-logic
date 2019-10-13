pub enum Status {
    Bleed,
    Burn,
    Freeze,
    Sleep,
    Paralysis,
    Not(Box<Status>),
    And(Box<Status>, Box<Status>),
    Or(Box<Status>, Box<Status>),
}
