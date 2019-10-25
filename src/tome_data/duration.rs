
use serde_derive::Deserialize;

/// How long something lasts, each variant is == to one of itself.
/// These are not designed to be paired with other numbers ({ d: Turn, n: 2 } === 3 turns), rather,
/// all durations should be described as one of a variant.
#[derive(Deserialize)]
pub enum DurationSpec {
    Instant,
    Turn,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Indefinite,
}
