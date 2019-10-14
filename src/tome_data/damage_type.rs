use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum DamageType {
    Ignis,
    Aqua,
    Flora,

    Mind,
    Body,
    Soul,

    Slicing,
    Bashing,
    Poking,
}
