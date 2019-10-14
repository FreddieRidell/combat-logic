use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Range {
    Melee,
    Distance(u64),
}
