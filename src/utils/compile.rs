use std::{fs, process::{Child, Command}};
use crate::Language::{self, Cpp, Python, Rust};

use crate::{
    config::Config, paths
};

pub enum CompileMode {
    Std,
    Dbg,
}

fn compile_rust(path: &str, ext: &str, config: &Config, mode: CompileMode) -> Result<Child, ()> {
    let lang: Language = Language::Rust;

    let from = format!("{path}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), path, "exe");
    let mut args = vec![from, String::from("-o"), to];
    args.append(&mut match mode {
        CompileMode::Std => config.get_compile_std_args(lang),
        CompileMode::Dbg => config.get_compile_dbg_args(lang),
    });

    if let Ok(child) = Command::new("rustc")
    .args(args)
    .spawn() {
        Ok(child)
    } else {
        Err(())
    }
}

fn compile_cpp(name: &str, ext: &str, config: &Config, mode: CompileMode) -> Result<Child, ()> {
    let lang: Language = Language::Cpp;

    let from = format!("{name}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), name, "exe");
    let mut args = vec![from, String::from("-o"), to];
    args.append(&mut match mode {
        CompileMode::Std => config.get_compile_std_args(lang),
        CompileMode::Dbg => config.get_compile_dbg_args(lang),
    });

    if let Ok(child) = Command::new("rustc")
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

pub fn compile_any(path: &String, config: &Config, mode: CompileMode) -> Result<Option<Child>, ()> {
    let (name, ext) = path.split_once(".").unwrap();
    let lang = Language::from_ext(ext)?;

    Ok(match lang {
        Cpp => Some(compile_cpp(name, ext, config, mode)?),
        Rust => Some(compile_rust(name, ext, config, mode)?),
        Python => {
            copy_file(path)?;
            None
        },
    })
}