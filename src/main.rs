mod readme;
mod config;
mod hashes;
mod paths;
mod cmd;
mod utils;
mod log;
mod controllers;

use std::{collections::HashSet, env, time::Instant};

use colored::Colorize;
use log::error;

fn index() {
    println!("{} {}", "stress testing manager", "PAXX ".bold().on_purple());
    println!("{} {}", "stress testing manager", "PAXX ".bold().on_purple());
    println!("{} {}", "stress testing manager", "PAXX ".bold().on_purple());
    println!("{} {}", "stress testing manager", "PAXX ".bold().on_purple());
}

type Flags = HashSet<String>;



fn cmd<'a>(args: &'a Vec<String>) -> Result<(), ()> {
    let start_time = Instant::now();
    let command = match args.get(1) {
        Some(c) => c.clone(),
        None => {
            index();
            return Ok(());
        }
    };

    let mut flags = Flags::new();
    for arg in args {
        if arg.starts_with('-') {
            flags.insert(arg[1..].to_string());
        }
    }

    println!("{}", "PAXX --- >".bold().purple());

    match command.as_str() {
        "init" => {
            controllers::init();
        }

        "check" => {
            controllers::check(&flags);
        }

        "build" => {
            controllers::build(&flags)?;
        }

        "run" => {
            controllers::run(&args, &flags)?;
        }

        "catch" => {
            controllers::catch(args, &flags)?;
        }

        "remove" => {
            controllers::remove();
        }

        "get" => {
            controllers::get(&args)?;
        }

        "pat" => {
            controllers::pat(args, &flags)?;
        }

        "solo" => {
            controllers::solo(&flags)?;
        }

        "cfg" => {
            controllers::cfg(args, &flags)?;
        }

        _ => {
            error("command", "incorrect");
        },
    };


    let time = start_time.elapsed();
    println!("{}", "____".bold());
    println!("{} in {} secs", "complited".white(), format!("{:.3}", (time.as_secs_f32() * 1000.0).ceil()/1000.0).bold().bright_magenta());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = cmd(&args);
}
