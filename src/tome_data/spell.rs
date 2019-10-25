use super::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SpellSpec {
    /// The name that renders by default for an item
    name: String,
    /// The point-buy value of an item, used to derive GP and level
    value: u64,
    /// The one scentence description of the item, each instance picks a fixed option from the
    /// OneOrMany
    tagline: OneOrMany<String>,
    /// Any extra description text, chosen at random every time the item is viewed
    flavour: OneOrMany<String>,

    /// The mana consumed by the spell
    mana: u64,

    /// Spells inherently represent 1 action, so the properties of `ActionSpec` are spread into
    /// `SpellSpec`
    #[serde(flatten)]
    action: ActionSpec,
}

impl TomeSpec for SpellSpec {}

pub struct SpellInstance {
    spell_spec: Rc<SpellSpec>,
}
impl TomeInstance for SpellInstance {}

impl CreateFromInstance<SpellSpec> for SpellInstance {
    fn create_from_spec(_: &Tome, spec: &Rc<SpellSpec>) -> Self {
        Self {
            spell_spec: spec.clone(),
        }
    }
}
