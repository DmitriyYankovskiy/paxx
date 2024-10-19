use std::{fs, process::{Child, Command}};
use crate::Language::{self, Cpp, Python, Rust};

use crate::paths;

pub enum Mode {
    Rls,
    Dbg,
    Hdbg,
}

fn rust(path: &str, ext: &str, mut args: Vec<String>) -> Result<Child, ()> {
    let from = format!("{path}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), path, "exe");
    args.append(&mut vec![from, String::from("-o"), to]);

    if let Ok(child) = Command::new("rustc")
    .args(args)
    .spawn() {
        Ok(child)
    } else {
        Err(())
    }
}

fn cpp(name: &str, ext: &str, mut args: Vec<String>) -> Result<Child, ()> {
    let from = format!("{name}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), name, "exe");
    args.append(&mut vec![from, String::from("-o"), to]);
    if let Ok(child) = Command::new("g++")
    .args(args)
    .spawn() {
        Ok(child)
    } else {
        Err(())
    }
}

pub fn copy_file(path: &str) -> Result<(), ()> {
    fs::copy(format!("{path}"), format!("{}/{path}", paths::build_dir())).unwrap();
    Ok(())
}

pub fn any(path: &String, args: &Vec<String>, lang: Language) -> Result<Option<Child>, ()> {
    let (name, ext) = path.split_once(".").unwrap();
    Ok(match lang {
        Cpp => Some(cpp(name, ext,args.clone())?),
        Rust => Some(rust(name, ext, args.clone())?),
        Python => {
            copy_file(path)?;
            None
        },
    })
}