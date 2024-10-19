use std::{fs, process::{Child, Command}};
use crate::Language::{self, Cpp, Python, Rust};

use crate::{
    config::Config, paths
};

pub enum Mode {
    Rls,
    Dbg,
    Hdbg,
}

fn rust(path: &str, ext: &str, config: &Config, mode: Mode) -> Result<Child, ()> {
    let lang: Language = Language::Rust;

    let from = format!("{path}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), path, "exe");
    let mut args = vec![from, String::from("-o"), to];
    args.append(&mut match mode {
        Mode::Rls => config.get_compile_rls_args(lang),
        Mode::Dbg => config.get_compile_dbg_args(lang),
        Mode::Hdbg => config.get_compile_hdbg_args(lang),
    });

    if let Ok(child) = Command::new("rustc")
    .args(args)
    .spawn() {
        Ok(child)
    } else {
        Err(())
    }
}

fn cpp(name: &str, ext: &str, config: &Config, mode: Mode) -> Result<Child, ()> {
    let lang: Language = Language::Cpp;

    let from = format!("{name}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), name, "exe");
    let mut args = vec![from, String::from("-o"), to];
    args.append(&mut match mode {
        Mode::Rls => config.get_compile_rls_args(lang),
        Mode::Dbg => config.get_compile_dbg_args(lang),
        Mode::Hdbg => config.get_compile_hdbg_args(lang),
    });

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

pub fn any(path: &String, config: &Config, mode: Mode) -> Result<Option<Child>, ()> {
    let (name, ext) = path.split_once(".").unwrap();
    let lang = Language::from_ext(ext)?;

    Ok(match lang {
        Cpp => Some(cpp(name, ext, config, mode)?),
        Rust => Some(rust(name, ext, config, mode)?),
        Python => {
            copy_file(path)?;
            None
        },
    })
}