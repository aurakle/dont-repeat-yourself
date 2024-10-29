use std::{collections::HashMap, fs::{self, File}, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::clipboard::Contents;

#[derive(Serialize, Deserialize)]
pub struct Mappings (HashMap<String, Contents>);

impl Mappings {
    fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn load() -> Result<Self, String> {
        match File::open(path()?) {
            Ok(reader) => ciborium::from_reader::<Mappings, File>(reader).map_err(|e| e.to_string()),
            Err(e) => Ok(Mappings::new())
        }
    }

    pub fn save(self: &Mappings) -> Result<(), String> {
        ciborium::into_writer(self, File::create(path()?).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
    }

    pub fn put(self: &mut Mappings, key: String, value: Contents) -> &mut Self {
        self.0.insert(key, value);
        self
    }

    pub fn get(self: &Mappings, key: String) -> Result<Contents, String> {
        match self.0.get(&key) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Key does not map to a value"))
        }
    }
}

fn path() -> Result<PathBuf, String> {
    let mut result = match directories::BaseDirs::new() {
        Some(v) => v,
        None => return Err(format!("OS failed to provide configuration paths"))
    }.config_dir().to_path_buf();
    result.push("dont-repeat-yourself.cbor");

    Ok(result)
}
