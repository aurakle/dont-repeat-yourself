use std::{collections::HashMap, process::Command};
use x11_clipboard::Clipboard as X11Clipboard;

use crate::data::Contents;

pub struct Clipboard(pub X11Clipboard);

impl Clipboard {
    pub fn new() -> Result<Self, String> {
        Ok(Self(X11Clipboard::new().map_err(|e| e.to_string())?))
    }

    pub fn get_contents() -> Result<Contents, String> {
        let targets = Command::new("xclip")
            .arg("-o")
            .arg("-target")
            .arg("TARGETS")
            .arg("-selection")
            .arg("clipboard")
            .output()
            .map_err(|e| e.to_string())?
            .stdout;
        let targets = String::from_utf8(targets)
            .map_err(|e| e.to_string())?;
        let targets = targets
            .split_whitespace()
            .collect();

        Contents::new(targets)
    }

    pub fn set_contents(&self, contents: Contents) -> Result<(), String> {
        let mut target_map = HashMap::new();

        for kv in contents.0 {
            target_map.insert(self.0.getter.get_atom(kv.0.as_str()).map_err(|e| format!("Failed to obtain atom for target {} due to: {}", kv.0, e))?, kv.1);
        }

        self.0.store_multiple(
            self.0.setter.atoms.clipboard,
            target_map,
        ).map_err(|e| format!("Failed to load clipboard due to: {}", e))?;

        Ok(())
    }
}
