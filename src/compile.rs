use colored::Colorize;

use std::{fs, process::{Child, Command}};

use crate::{
    config::Config,
    paths
};

pub fn executable_ext(source_ext: &str) -> String {
    match source_ext {
        "c++" | "cpp" | "rs" => "exe",
        "py" => "py",
        _ => "not compile",
    }.to_string()
}



pub fn compile_cpp(path: &str, ext: &str, config: &Config) -> Result<Child, ()> {
    let from = format!("{path}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), path, executable_ext(ext));
    let mut args = vec![from, String::from("-o"), to];
    args.append(&mut config.get_args("c++"));
    
    if let Ok(child) = Command::new("g++")
        .args(args)
        .spawn() {
        Ok(child)
    } else {
        println!("{} {}", "c++".bold().bright_red(), "args incorrect".red());
        Err(())
    }
}

pub fn compile_rust(path: &str, ext: &str, config: &Config) -> Result<Child, ()> {
    let from = format!("{path}.{ext}");
    let to = format!("{}/{}.{}", paths::build_dir(), path, executable_ext(ext));
    let mut args = vec![from, String::from("-o"), to];
    args.append(&mut config.get_args("rust"));    

    if let Ok(child) = Command::new("rustc")
        .args(args)
        .spawn() {
            Ok(child)
        } else {
            println!("{} {}", "rust".bold().bright_red(), "args incorrect".red());
            Err(())
        }
}

pub fn copy_file(path: &str, ext: &str) -> Result<(), ()> {
    fs::copy(format!("{path}.{ext}"), format!("{}/{path}.{ext}", paths::build_dir())).unwrap();
    Ok(())
}

pub fn compile(path: &String, config: &Config) -> Result<Option<Child>, ()> {
    let (path, ext) = path.split_once(".").unwrap();
    match ext {
        "cpp" | "c++" => {
            Ok(Some(compile_cpp(path, ext, config)?))
        }
        "rs" => {
            Ok(Some(compile_rust(path, ext, config)?))
        }
        "py" => {
            copy_file(path, ext)?;
            Ok(None)
        }
        _ => {
            println!("{} {}", ext.bold().bright_red(), "does not compile".red());
            Err(())
        }
    }
}