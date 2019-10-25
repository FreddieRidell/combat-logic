use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum DamageType {
    Fire,
    Water,
    Plant,
    Ice,
    Electic,

    Mind,
    Body,
    Soul,

    Slicing,
    Bashing,
    Poking,
}
