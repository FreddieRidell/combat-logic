use super::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SpellLevel(u8);

#[derive(Deserialize)]
pub enum SpellAreaOfEffectSpec {
    /// A number of characters that are directly targeted
    Character(u8),
    /// A number of points where the effect will occour
    Points(u8),
    /// A straight line with a given length, from the caster out
    Line(u8),
    /// A 45 degreen cone of a given length, out from the caster
    Cone(u8),
    /// A sphere of a given diameter, anywhere within range of the caster
    Sphere(u8),
}
impl Default for SpellAreaOfEffectSpec {
    fn default() -> Self {
        SpellAreaOfEffectSpec::Character(1)
    }
}

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
    /// The lowest level that the spell can be cast at
    level: SpellLevel,

    /// Does the behaviour of the spell change when cast using higher level slots
    #[serde(default = "default_bool")]
    multiplies_at_higher_level: bool,

    /// Who/Where the spell creates its effect,
    #[serde(default)]
    effects: SpellAreaOfEffectSpec,

    #[serde(flatten)]
    action: ActionSpec,
}

fn default_bool() -> bool {
    false
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
