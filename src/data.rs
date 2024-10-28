use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mappings {
    map: HashMap<String, String>,
}

impl Mappings {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self, String> {
        match fs::read(path()?) {
            Ok(contents) => serde_json::from_str::<Mappings>(String::from_utf8(contents)
                .map_err(|e| e.to_string())?.as_str())
                .map_err(|e| e.to_string()),
            Err(_) => Ok(Mappings::new())
        }
    }

    pub fn save(self: &Mappings) -> Result<(), String> {
        fs::write(path()?, serde_json::to_string(&self)
            .map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }

    pub fn put(self: &mut Mappings, key: String, value: String) -> &mut Self {
        self.map.insert(key, value);
        self
    }

    pub fn get(self: &Mappings, key: String) -> Result<String, String> {
        match self.map.get(&key) {
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
    result.push("dont-repeat-yourself.json");

    Ok(result)
}
