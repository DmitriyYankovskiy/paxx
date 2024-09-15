use std::{fs, io::Write, path::Path};

use crate::{out, Flags};

pub fn create_file(path: String, text: &str, flags: &Flags) {
    if !Path::new(path.as_str()).exists() || flags.contains("re") {
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(text.as_bytes()).unwrap();
        out::ok(&path, "created");
    } else {
        out::info(&path, "already exists");
    }
} 