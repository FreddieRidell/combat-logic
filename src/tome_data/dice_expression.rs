use super::*;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::collections::HashMap;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Dice {
    Constant(u64),
    D10,
    D100,
    D12,
    D20,
    D4,
    D6,
    D8,
}

pub struct DiceExpression(HashMap<Dice, u8>);

struct DiceExpressionVisitor;

impl<'de> Visitor<'de> for DiceExpressionVisitor {
    type Value = DiceExpression;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an expression of the form `2d4 + d8 + 2`")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let hash_map: HashMap<Dice, u8> = s
            .split("+")
            .map(|x| x.trim().to_lowercase())
            .map(|token| {
                if (token.contains("d")) {
                    let d_index = token.find("d").expect("should contain d");
                    let number = token[..d_index].parse::<u8>().unwrap_or(0);
                    let dice_size = token[1 + d_index..].parse::<u8>().map_err(|_| {
                        E::custom(format!("{} is an unparseable die value", &token))
                    })?;

                    let dice = match dice_size {
                        4 => Ok(Dice::D4),
                        6 => Ok(Dice::D6),
                        8 => Ok(Dice::D8),
                        10 => Ok(Dice::D10),
                        12 => Ok(Dice::D12),
                        20 => Ok(Dice::D20),
                        100 => Ok(Dice::D100),
                        _ => Err((E::custom(format!("{} is not a valid die size", &token)))),
                    }?;

                    Ok((dice, number))
                } else {
                    Ok((
                        Dice::Constant(token.parse::<u64>().map_err(|_| {
                            E::custom(format!("{} is an unparseable die value", &token))
                        })?),
                        1,
                    ))
                }
            })
            .collect::<Result<HashMap<Dice, u8>, E>>()?;

        Ok(DiceExpression(hash_map))
    }
}

impl<'de> Deserialize<'de> for DiceExpression {
    fn deserialize<D>(deserializer: D) -> Result<DiceExpression, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DiceExpressionVisitor)
    }
}
