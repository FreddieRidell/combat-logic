use super::*;
use crate::error::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use walkdir::WalkDir;

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
    spell_specs: HashMap<PathBuf, Rc<SpellSpec>>,
}

impl Tome {
    /// Takes a specific chapter (eg: items|creatures), and a slug (eg: equipment/weapons/sword),
    /// and returns a Spec generated from the file system.
    fn read_spec<Spec: TomeSpec>(&mut self, chapter_name: &str, spec_id: &str) -> RPGResult<Spec> {
        let chapter_name_path = Path::new(chapter_name);
        let instance_path = Path::new(spec_id);

        let full_instance_path: PathBuf = self
            .root_dir
            .as_path()
            .join(&chapter_name_path)
            .join(&instance_path)
            .with_extension("toml")
            .canonicalize()
            .map_err(|e| RPGError::new(RPGErrorKind::TomeEntryNotFound).from(e))?
            .to_path_buf();

        let data_raw: String = fs::read_to_string(&full_instance_path).map_err(|e| {
            RPGError::new(RPGErrorKind::TomeEntryNotFound)
                .with_msg(|| format!("could find find tome spec for `{}`", &spec_id))
                .from(e)
        })?;

        let spec: Spec = toml::from_str(&data_raw).map_err(|e| {
            RPGError::new(RPGErrorKind::TomeEntryInvalid)
                .with_msg(|| format!("could not parse tome spec for `{}`", &spec_id))
                .from(e)
        })?;

        Ok(spec)
    }
}

/// Generates functions for loading a spec from the filesystem, caching it, and created a specific
/// instance of something from that spec
macro_rules! get_and_load_tome_type {
    (
        $spec_fn_name:ident,
        $instance_fn_name:ident,
        $test_fn_name:ident,
        $chapter:expr,
        $store:ident,
        $spec:ident,
        $instance:ident
    ) => {
        pub fn $spec_fn_name(&mut self, spec_id: &str) -> RPGResult<Rc<$spec>> {
            let x: Rc<$spec> = Rc::new(self.read_spec($chapter, spec_id)?);

            self.$store.insert(PathBuf::from(spec_id), x.clone());

            Ok(x)
        }

        pub fn $instance_fn_name(&mut self, spec_id: &str) -> RPGResult<$instance> {
            let spec = self.$spec_fn_name(spec_id)?;

            Ok($instance::create_from_spec(&self, &spec))
        }

        pub fn $test_fn_name(&mut self) -> HashMap<PathBuf, RPGResult<()>> {
            let root_dir_path_for_this_spec_type = &self.root_dir.as_path().join($chapter);
            let root_dir_replacement_length = root_dir_path_for_this_spec_type.to_str().unwrap().len() + 1;

            WalkDir::new(root_dir_path_for_this_spec_type).into_iter().filter(|child_path| {
                let child_file_descriptor = child_path.as_ref().unwrap();
                child_file_descriptor.file_type().is_file()
                && !child_file_descriptor.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
            })
            .map(|child_path| {
                let child_file_descriptor = child_path.unwrap();
                let child_full_path_name: &str = &child_file_descriptor.path().to_str().expect("couldn't read path as string");
                let child_identifier: &str = &child_full_path_name[root_dir_replacement_length..];

                (
                    PathBuf::from(child_identifier),
                    self.$instance_fn_name(child_identifier).map( |_| ())
                )
            })
            .collect()
        }
    };
}

impl Tome {
    get_and_load_tome_type!(
        load_item_spec,
        create_item_instance,
        test_all_items,
        "items",
        item_specs,
        ItemSpec,
        ItemInstance
    );
    get_and_load_tome_type!(
        load_spell_spec,
        create_spell_instance,
        test_all_spells,
        "spells",
        spell_specs,
        SpellSpec,
        SpellInstance
    );
}

impl Tome {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
            item_specs: HashMap::new(),
            spell_specs: HashMap::new(),
        }
    }
}
