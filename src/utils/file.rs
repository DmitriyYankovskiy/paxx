use std::{fs, io::Write, path::Path};

use crate::{log, Flags};

pub fn create_file(path: String, text: &str, flags: &Flags) {
    if !Path::new(path.as_str()).exists() || flags.contains("r") {
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(text.as_bytes()).unwrap();
        log::ok(&path, "created");
    } else {
        log::info(&path, "already exists");
    }
} 