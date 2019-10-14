use super::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Dice {
    D4,
    D6,
    D8,
    D12,
    D20,
    D100,
    //Constant(u64),
}

pub type DiceExpression = OneOrMany<Dice>;
