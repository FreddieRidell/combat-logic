use super::*;
use crate::error::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::rc::Rc;

mod action;
mod attribute;
mod buff;
mod character;
mod creature;
mod damage_type;
mod dice_expression;
mod duration;
mod equipment_slot;
mod generics;
mod item;
mod range;
mod spell;
mod status;
mod target_requirement;

pub use action::*;
pub use attribute::*;
pub use buff::*;
pub use character::*;
pub use creature::*;
pub use damage_type::*;
pub use dice_expression::*;
pub use duration::*;
pub use equipment_slot::*;
pub use generics::*;
pub use item::*;
pub use range::*;
pub use spell::*;
pub use status::*;
pub use target_requirement::*;

pub trait TomeSpec: DeserializeOwned {}
pub trait TomeInstance {}
pub trait CreateFromInstance<Spec: TomeSpec>: TomeInstance {
    fn create_from_spec(tome: &Tome, spec: &Rc<Spec>) -> Self;
}

pub struct Tome {
    root_dir: PathBuf,
    item_specs: HashMap<PathBuf, Rc<ItemSpec>>,
}

impl Tome {
    /// Takes a specific chapter (eg: items|creatures), and a slug (eg: equipment/weapons/sword),
    /// and returns a Spec generated from the file system.
    fn read_spec<Spec: TomeSpec>(
        &mut self,
        chapter_name: &str,
        instance_id: &str,
    ) -> RPGResult<Spec> {
        let chapter_name_path = Path::new(chapter_name);
        let instance_path = Path::new(instance_id);

        let full_instance_path: PathBuf = self
            .root_dir
            .as_path()
            .join(&chapter_name_path)
            .join(&instance_path)
            .with_extension("toml")
            .canonicalize()
            .expect("could not join paths")
            .to_path_buf();

        let data_raw: String = fs::read_to_string(&full_instance_path).map_err(|e| {
            RPGError::new(RPGErrorKind::TomeEntryNotFound)
                .with_msg(|| format!("could find find tome spec for `{}`", &instance_id))
                .from(e)
        })?;

        let spec: Spec = toml::from_str(&data_raw).map_err(|e| {
            RPGError::new(RPGErrorKind::TomeEntryInvalid)
                .with_msg(|| format!("could not parse tome spec for `{}`", &instance_id))
                .from(e)
        })?;

        Ok(spec)
    }
}

/// Generates functions for loading a spec from the filesystem, caching it, and created a specific
/// instance of something from that spec
macro_rules! get_and_load_tome_type {
    ($spec_fn_name:ident, $instance_fn_name:ident, $chapter:expr, $store:ident, $spec:ident, $instance:ident) => {
            pub fn $spec_fn_name(&mut self, instance_id: &str) -> RPGResult<Rc<$spec>> {
                let item: Rc<$spec> = Rc::new(self.read_spec($chapter, instance_id)?);

                self.$store.insert(PathBuf::from(instance_id), item.clone());

                Ok(item)
            }

            pub fn $instance_fn_name(&mut self, instance_id: &str) -> RPGResult<$instance> {
                let spec = self.load_item_spec(instance_id)?;

                Ok(ItemInstance::create_from_spec(&self, &spec))
            }
    };
}

impl Tome {
    get_and_load_tome_type!(
        load_item_spec,
        create_item_instance,
        "items",
        item_specs,
        ItemSpec,
        ItemInstance
    );
}

impl Tome {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
            item_specs: HashMap::new(),
        }
    }
}
