use std::env;

use app::Dialogue;
use clipboard::{Clipboard, Contents};
use data::Mappings;

mod data;
mod app;
mod clipboard;

fn main() -> Result<(), String> {

    match env::args().collect::<Vec<_>>()[1].as_str() {
        "save" => save(),
        "load" => load(),
        &_ => Err(format!("Unknown verb -- either one of `load` or `save` allowed"))
    }
}

fn save() -> Result<(), String> {
    println!("Copying clipboard...");
    let contents = Clipboard::get_contents().map_err(|e| e.to_string())?;
    println!("Opening save dialogue...");
    let name = dialogue()?;
    put(name, contents)
}

fn load() -> Result<(), String> {
    println!("Opening load dialogue...");
    let key = dialogue()?;
    println!("Expanding {}...", key);
    let result = expand(key)?;
    println!("Loading into clipboard...");
    Clipboard::set_contents(result).map_err(|e| e.to_string())
}

fn dialogue() -> Result<String, String> {
    let mut text = format!("");
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Don't Repeat Yourself!",
        native_options,
        Box::new(|_cc| Ok(Box::new(Dialogue::new(&mut text)))),
    ).map_err(|e| e.to_string())?;
    text.retain(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation()); //TODO: probably
                                                                            //shouldn't be
                                                                            //using `retain`

    Ok(text)
}

fn put(key: String, value: Contents) -> Result<(), String> {
    Mappings::load()?.put(key, value).save()
}

fn expand(key: String) -> Result<Contents, String> {
    Mappings::load()?.get(key)
}
