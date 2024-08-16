mod readme;
mod config;
mod hashes;
mod paths;
mod cmd;
mod utils;
mod log;

use std::{collections::HashSet, env, fs, time::Instant};

use colored::Colorize;
use config::Config;
use hashes::Hashes;
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

    match command.as_str() {
        "init" => {
            cmd::init::all();
        }

        "check" => {
            if flags.contains("r") {
                Config::write(&mut Config::default());
            } 
            if let Ok(_) = Config::load_and_check() {
                log::ok("config", "is valid");
            }
        }

        "build" => {
            let config = Config::load_and_check()?;

            let mut hashes = Hashes::load(&flags);
            let _ = cmd::build::all(&config, &mut hashes);
            Hashes::write(&mut hashes);
        }

        "run" => {
            let arg2 = args.get(2);
            let arg2 = if let Some(arg2) = arg2 {
                arg2.clone()
            } else {
                log::error("tests_count", "not found");
                return Err(());
            }; 
            let tests_count: usize = match arg2.parse() {
                Ok(count) => count,
                Err(_) => {
                    log::error("tests_count", "incorrect");
                    return Err(());
                }
            };

            let config = Config::load_and_check()?;

            let mut hashes = Hashes::load(&flags);
            let build_res = cmd::build::all(&config, &mut hashes);
            Hashes::write(&mut hashes);
            build_res?;

            cmd::run::all(tests_count, None, &config, &flags)?;
        }

        "catch" => {
            let arg2 = args.get(2);
            let arg2 = if let Some(arg2) = arg2 {
                arg2.clone()
            } else {
                log::error("errors count", "not found");
                return Err(());
            }; 

            let arg3 = args.get(3);
            let arg3 = if let Some(arg3) = arg3 {
                arg3.clone()
            } else {
                log::error("tests count", "not found");
                return Err(());
            };

            let tests_count: usize = match arg3.parse() {
                Ok(count) => count,
                Err(_) => {
                    log::error("tests count", "incorrect");
                    return Err(());
                }
            };

            let errors_count: usize = match arg2.parse() {
                Ok(count) => count,
                Err(_) => {
                    log::error("errors count", "incorrect");
                    return Err(());
                }
            };

            let config = Config::load_and_check()?;

            let mut hashes = Hashes::load(&flags);
            let build_res = cmd::build::all(&config, &mut hashes);
            Hashes::write(&mut hashes);
            build_res?;

            cmd::run::all(tests_count, Some(errors_count), &config, &flags)?;
        }

        "remove" => {
            fs::remove_dir_all(paths::dir()).unwrap();
        }

        "get" => {
            let config = Config::load_and_check()?;

            let arg2 = args.get(2);
            let arg2 = if let Some(arg2) = arg2 {
                arg2.clone()
            } else {
                log::error("tests number", "not found");
                return Err(());
            }; 
            let test_number: usize = match arg2.parse() {
                Ok(count) => count,
                Err(_) => {
                    log::error("tests number", "incorrect");
                    return Err(());
                }
            };


            cmd::get::all(test_number, &config)?;
        }

        "pat" => {
            let arg2 = args.get(2);
            let arg2 = if let Some(arg2) = arg2 {
                arg2.clone()
            } else {
                log::error("pattern", "not found");
                return Err(());
            };


            match arg2.as_str() {
                "gen" => {
                    let arg3 = args.get(3);
                    let arg3 = if let Some(arg3) = arg3 {
                        arg3.clone()
                    } else {
                        if let Some(p) = Config::load()?.test_gen_path {
                            p
                        } else {
                            log::error("tests generator", "not found");
                            return Err(());
                        }
                    };

                    cmd::pat::gen(&arg3, &flags);
                    if flags.contains("s") {
                        cmd::cfg::set_test_gen(&arg3)?;
                    }
                },

                "edit_cfg_cpp_vscode" => {
                    cmd::pat::edit_cfg_cpp_vscode(&".editorconfig".to_string(), &flags);
                },

                "std" => {
                    let arg3 = args.get(3);
                    let arg3 = if let Some(arg3) = arg3 {
                        arg3.clone()
                    } else {
                        Config::load()?.solution_path
                    };
                    cmd::pat::std(&arg3, &flags);

                    if flags.contains("s") {
                        cmd::cfg::set_solution(&arg3)?;
                    }
                },
                _ => {
                    log::error("pattern", "incorrect");
                    return Err(());
                }
            }
        }

        "solo" => {
            let config = Config::load_and_check()?;

            let mut hashes = Hashes::load(&flags);
            let build_res = cmd::build::all(&config, &mut hashes);
            Hashes::write(&mut hashes);
            build_res?;

            cmd::solo::solution(&Config::load()?)?;
        }

        _ => {
            error("command", "incorrect");
        },
    };


    let time = start_time.elapsed();
    println!();
    println!("complited in {} secs", format!("{:.3}", (time.as_secs_f32() * 1000.0).ceil()/1000.0).bold().bright_magenta());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = cmd(&args);
}
