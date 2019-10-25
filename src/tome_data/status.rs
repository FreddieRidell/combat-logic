
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum StatusSpec {
    Poison,
    Bleed,
    Burn,
    Freeze,
    Sleep,
    Fear,
}
