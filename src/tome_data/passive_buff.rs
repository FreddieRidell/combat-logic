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
pub struct PassiveBuff {
    attribute: Attribute,
    polarity: Polarity,
    value: u64,
}
