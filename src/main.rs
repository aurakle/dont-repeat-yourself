use std::{env, process};

use app::Dialogue;
use clipboard::Clipboard;
use x11rb::protocol::xproto::ConnectionExt;
use data::Contents;

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
    let contents = Clipboard::get_contents()?;
    println!("Opening save dialogue...");
    let name = dialogue()?;
    contents.put(name)
}

fn load() -> Result<(), String> {
    println!("Opening load dialogue...");
    let key = dialogue()?;
    println!("Expanding {}...", key);
    let result = Contents::get(key.clone())?;
    println!("Loading into clipboard...");
    let clip = Clipboard::new()?;
    clip.set_contents(result)?;
    println!("Successfully loaded clipboard {}, waiting for X11 selection to change owners...", key);
    
    // Wait for clipboard owner to change, then kill process
    loop {
        if clip.0.setter.connection.get_selection_owner(clip.0.getter.atoms.clipboard)
            .map_err(|e| format!("Failed to obtain current X11 clipboard owner due to: {}", e))?
            .reply()
            .map(|reply| reply.owner != clip.0.setter.window)
            .unwrap_or(true)
        {
            process::exit(0);
        }
    }
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
