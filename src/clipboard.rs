use libc::fork;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::{self, Command}, time::Duration};
use x11_clipboard::{Atom, Clipboard as X11Clipboard, RustConnection};
use x11rb::protocol::xproto::ConnectionExt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Contents (HashMap<String, Vec<u8>>);

impl Contents {
    fn new(targets: Vec<&str>) -> Result<Self, String> {
        let mut map = HashMap::new();

        for target in targets {
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

        Ok(Self(map))
    }
}

pub struct Clipboard (X11Clipboard);

impl Clipboard {
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

    pub fn set_contents(contents: Contents) -> Result<(), String> {
        //TODO: this only works with utf8 strings
        match unsafe { fork() } {
            -1 => Err(format!("Could not fork process")),
            0 => {
                //// Obtain new X11 clipboard context, set clipboard contents
                //let clip = Clipboard::new().expect("Failed to obtain X11 clipboard context");
                //clip.0.store(
                //    clip.0.setter.atoms.clipboard,
                //    clip.0.setter.atoms.utf8_string,
                //    contents,
                //)
                //.expect("Failed to set clipboard contents through forked process");
                //
                //// Wait for clipboard to change, then kill fork
                //clip.0.load_wait(
                //    clip.0.getter.atoms.clipboard,
                //    clip.0.getter.atoms.utf8_string,
                //    clip.0.getter.atoms.property,
                //)
                //.expect("Failed to wait on new clipboard value in forked process");

                process::exit(0)
            }
            _pid => Ok(()),
        }
    }
}
