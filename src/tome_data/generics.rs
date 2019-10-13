pub enum PartyNumber<T> {
    Above(T),
    Below(T),
    Highest,
    Lowest,
}

pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
