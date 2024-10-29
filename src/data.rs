use std::{collections::HashMap, fs::{self, File}, path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Contents (pub HashMap<String, Vec<u8>>);

impl Contents {
    pub fn new(targets: Vec<&str>) -> Result<Self, String> {
        let mut map = HashMap::new();

        for target in targets {
            if target != "TARGETS" {
                let contents = Command::new("xclip")
                    .arg("-o")
                    .arg("-target")
                    .arg(target)
                    .arg("-selection")
                    .arg("clipboard")
                    .output()
                    .map_err(|e| e.to_string())?
                    .stdout;
                map.insert(target.to_string(), contents);
            }
        }

        Ok(Self(map))
    }

    pub fn get(key: String) -> Result<Contents, String> {
        match File::open(path(key)?) {
            Ok(reader) => ciborium::from_reader::<Contents, File>(reader).map_err(|e| e.to_string()),
            Err(_) => Ok(Contents(HashMap::new()))
        }
    }

    pub fn put(&self, key: String) -> Result<(), String> {
        ciborium::into_writer(self, File::create(path(key)?).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
    }
}

fn path(key: String) -> Result<PathBuf, String> {
    let mut result = match directories::BaseDirs::new() {
        Some(v) => v,
        None => return Err(format!("OS failed to provide configuration paths"))
    }.config_dir().to_path_buf();

    result.push("dont-repeat-yourself");
    let _ = fs::create_dir_all(result.clone());
    result.push(format!("{}.cbor", key));

    Ok(result)
}
