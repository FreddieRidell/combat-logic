use super::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Polarity {
    #[serde(rename = "+")]
    Positive,
    #[serde(rename = "-")]
    Negative,
}

#[derive(Deserialize)]
pub struct BuffSpec {
    attribute: Attribute,
    value: DiceExpression,
}
