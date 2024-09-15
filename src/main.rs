mod readme;
mod config;
mod hashes;
mod paths;
mod cmd;
mod utils;
mod out;
mod controllers;
mod buisness;

use std::time::Instant;

use colored::Colorize;
use serde::{Deserialize, Serialize};

use out::error;
use utils::arg::Args; 


#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Cpp,
    Rust,
    Python,
}

impl Language {
    pub fn from_ext(s: &str) -> Result<Self, ()> {
        match s {
            "cpp" | "c++" => Ok(Self::Cpp),
            "rs" => Ok(Self::Rust),
            "py" => Ok(Self::Python),
            _ => Err(()),
        }
    }

    pub fn get_executable_ext(&self) -> &'static str {
        match self {
            Self::Cpp | Self::Rust => "exe",
            _ => "py",
        }
    }
}

const CAP: usize = 50_000;


fn index() {
    println!("{} {}", "code manager", "PAXX ".bold().on_purple());
}




fn cmd<'a>(args: &mut Args, flags: &utils::arg::Flags) -> Result<(), ()> {
    let start_time = Instant::now();
    let command = args.get("command")?;

    println!("{}", "PAXX --- >".bold().purple());
    println!(":");

    match command.as_str() {
        "index" => index(),
        "init" => {
            controllers::init();
        }
        "check" => {
            controllers::check(&flags);
        }
        "stress" => {
            controllers::stress(args, &flags)?;
        }
        "catch" => {
            controllers::catch(args, &flags)?;
        }
        "remove" => {
            controllers::remove();
        }
        "get" => {
            controllers::get(args)?;
        }
        "pat" => {
            controllers::pat(args, &flags)?;
        }
        "run" => {
            controllers::run(&flags)?;
        }
        "cfg" => {
            controllers::cfg(args, &flags)?;
        }
        _ => {
            error("command", "incorrect"); 
            return Err(());
        }
    };


    let time = start_time.elapsed();
    println!("{}", "____".bold());
    println!("{} in {} secs", "complited".white(), format!("{:.3}", (time.as_secs_f32() * 1000.0).ceil()/1000.0).bold().bright_magenta());

    Ok(())
}

fn main() {
    let (mut args, flags) = Args::init();
    let _ = cmd(&mut args, &flags);
}
