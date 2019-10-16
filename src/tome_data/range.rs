use serde_derive::Deserialize;

/// The range between an actor an a target, measured in 5ft squares (e.g, Range(2) = 10ft)
/// A range of 0 means that the actor and the target must be the same
#[derive(Deserialize)]
pub struct Range(u64);
