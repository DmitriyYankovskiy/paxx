mod readme;
mod config;
mod hashes;
mod paths;
mod commands;
mod compile;

use std::{collections::HashSet, env, fs, time::Instant};

use colored::Colorize;
use config::Config;
use hashes::Hashes;

fn index() {
    println!("{} {}", "stress testing manager", "PAXX ".bold().on_purple());
}

type Flags = HashSet<String>;



fn cmd<'a>(args: &'a Vec<String>) -> Result<(), ()> {
    let start_time = Instant::now();
    let command = match args.get(1) {
        Some(c) => c.clone(),
        None => {
            index();
            return Err(());
        }
    };

    let mut flags = Flags::new();
    for arg in args {
        if arg.starts_with('-') {
            flags.insert(arg[1..].to_string());
        }
    }



    match command.as_str() {
        "init" => {
            commands::init::all();
        }

        "check" => {
            if flags.contains("r") {
                Config::write(&mut Config::default());
            } 
            match Config::load() {
                Ok(_) => println!("{}", "all right".bold().green()),
                Err(_) => {},
            };
        }

        "build" => {
            let config = Config::load()?;

            let mut hashes = Hashes::load(&flags);
            let _ = commands::build::all(&config, &mut hashes);
            Hashes::write(&mut hashes);
        }

        "run" => {
            let arg2 = args.get(2);
            let arg2 = if let Some(arg2) = arg2 {
                arg2.clone()
            } else {
                println!("{} {}", "tests count".bold().bright_red(), "not found".red());
                return Err(());
            }; 
            let test_count: usize = match arg2.parse() {
                Ok(count) => count,
                Err(_) => {
                    println!("{} {}", "incorrect".red(), "tests count".bold().bright_red());
                    return Err(());
                }
            };

            let config = Config::load()?;

            let mut hashes = Hashes::load(&flags);
            let build_res = commands::build::all(&config, &mut hashes);
            Hashes::write(&mut hashes);
            build_res?;

            commands::run::all(test_count, None, &config, &flags)?;
        }

        "catch" => {
            let arg2 = args.get(2);
            let arg2 = if let Some(arg2) = arg2 {
                arg2.clone()
            } else {
                println!("{} {}", "tests count".bold().bright_red(), "not found".red());
                return Err(());
            }; 

            let arg3 = args.get(3);
            let arg3 = if let Some(arg3) = arg3 {
                arg3.clone()
            } else {
                println!("{} {}", "errors count".bold().bright_red(), "not found".red());
                return Err(());
            };

            let tests_count: usize = match arg2.parse() {
                Ok(count) => count,
                Err(_) => {
                    println!("{} {}", "incorrect".red(), "tests count".bold().bright_red());
                    return Err(());
                }
            };

            let errors_count: usize = match arg3.parse() {
                Ok(count) => count,
                Err(_) => {
                    println!("{} {}", "incorrect".red(), "errors count".bold().bright_red());
                    return Err(());
                }
            };

            let config = Config::load()?;

            let mut hashes = Hashes::load(&flags);
            let build_res = commands::build::all(&config, &mut hashes);
            Hashes::write(&mut hashes);
            build_res?;

            commands::run::all(tests_count, Some(errors_count), &config, &flags)?;
        }

        "remove" => {
            fs::remove_dir_all(paths::dir()).unwrap();
        }

        _ => {
            println!("{} {}", "no such command:".red(), command.bold().bright_red());
        },
    };


    let time = start_time.elapsed();
    println!("--------");
    println!("complited in {} secs", time.as_secs_f32());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = cmd(&args);
}
