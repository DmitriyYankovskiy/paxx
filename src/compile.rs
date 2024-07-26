use colored::Colorize;

use std::process::{Child, Command};

use crate::{
    config::Config,
    paths
};

pub fn compile_cpp(path: &String, ext: &String, config: &Config) -> Result<Child, ()> {
    let from = format!("{path}.{ext}");
    let to = format!("{}/{}.exe", paths::build_dir(), path);
    let mut args = vec![from, String::from("-o"), to];
    args.append(&mut config.args[ext].clone());
    if let Ok(child) = Command::new("g++")
        .args(args)
        .spawn() {
            Ok(child)
        } else {
            println!("{} {}", "cpp".bold().bright_red(), "args incorrect".red());
            Err(())
        }
}

pub fn compile(path: &String, config: &Config) -> Result<Child, ()> {
    let (path, ext) = path.split_once(".").unwrap();
    match ext {
        "cpp" | "c++" => {
            compile_cpp(&path.to_string(), &ext.to_string(), config)
        }
        _ => {
            println!("{} {}", ext.bold().bright_red(), "does not compile".red());
            Err(())
        }
    }
}