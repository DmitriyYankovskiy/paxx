use std::fs::File;

use crate::out;

pub fn any(path: String) -> Result<(), ()> {
    let (name, ext) = match path.split_once('.') {
        None => {
            out::error("path", "incorrect");
            return Err(());
        },
        Some(p) => p,
    };

    match ext {
        "c++" | "cpp" => {
            cpp(path)?;
            Ok(())
        }
        ext => {
            out::error(ext, "not formatted");
            Err(())
        }
    }
}

fn cpp(path: String) -> Result<(), ()> {
    let mut file = if let Ok(f) = File::options().write(true).open(path) {
        f
    } else {
        out::error("file", "cannot open");
        return Err(());
    };

    Ok(())
}