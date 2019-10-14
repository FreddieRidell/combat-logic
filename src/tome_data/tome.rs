use super::*;
use crate::error::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub trait TomeItemInstance<Spec> {
    fn create_from_spec(spec: &Rc<Spec>) -> Self;
}

pub struct Tome<Spec: DeserializeOwned, Instance: TomeItemInstance<Spec>> {
    root_dir: PathBuf,
    specs: HashMap<PathBuf, Rc<Spec>>,
    phantom: PhantomData<Instance>,
}

impl<Spec: DeserializeOwned, Instance: TomeItemInstance<Spec>> Tome<Spec, Instance> {
    pub fn new(root_dir: &str) -> Self {
        Self {
            root_dir: Path::new(root_dir).to_path_buf(),
            specs: HashMap::new(),

            phantom: PhantomData,
        }
    }

    pub fn get_instance(&mut self, instance_id: &str) -> RPGResult<Instance> {
        let instance_path = Path::new(instance_id);
        let full_instance_path: PathBuf = self
            .root_dir
            .as_path()
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

        let spec: Rc<Spec> = Rc::new(toml::from_str(&data_raw).map_err(|e| {
            RPGError::new(RPGErrorKind::TomeEntryInvalid)
                .with_msg(|| format!("could not parse tome spec for `{}`", &instance_id))
                .from(e)
        })?);

        Ok(Instance::create_from_spec(&spec))
    }
}
