use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum TargetRequirementSpec {
    Myself,
    Ally,
    Enemy,
    WithinRange,
}
