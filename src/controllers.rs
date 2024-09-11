use std::fs;

use crate::{cmd, config::Config, hashes::Hashes, log, paths, utils, Flags};

pub fn init() {
    cmd::init::all();
}

pub fn check(flags: &Flags) {
    if flags.contains("r") {
        Config::write(&mut Config::default());
    } 
    if let Ok(_) = Config::load() {
        log::ok("config", "is valid");
    }
}

pub fn build(flags: &Flags) -> Result<(), ()> {
    let config = Config::load()?;

    let mut hashes = Hashes::load(flags);
    let _ = cmd::build::all(&config, &mut hashes);
    Hashes::write(&mut hashes);
    Ok(())
}

pub fn run(args: &Vec<String>, flags: &Flags) -> Result<(), ()> {
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

    let config = Config::load()?;

    let mut hashes = Hashes::load(flags);
    let build_res = cmd::build::all(&config, &mut hashes);
    Hashes::write(&mut hashes);
    build_res?;

    cmd::run::all(tests_count, None, &config, &mut hashes, flags)?;
    Ok(())
}

pub fn catch(args: &Vec<String>, flags: &Flags) -> Result<(), ()> {
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

    let config = Config::load()?;

    let mut hashes = Hashes::load(&flags);
    let build_res = cmd::build::all(&config, &mut hashes);
    Hashes::write(&mut hashes);
    build_res?;

    cmd::run::all(tests_count, Some(errors_count), &config, &mut hashes, &flags)?;

    Ok(())
}

pub fn remove() {
    fs::remove_dir_all(paths::dir()).unwrap();
}

pub fn get(args: &Vec<String>) -> Result<(), ()> {
    let config = Config::load()?;

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
    Ok(())
}

pub fn pat(args: &Vec<String>, flags: &Flags) -> Result<(), ()> {
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
                Config::load()?.get_generator_path()?
            };

            cmd::pat::gen(&arg3, &flags);
            if flags.contains("s") {
                cmd::cfg::set_test_gen(&arg3)?;
            }
        },

        "edit_cfg_c++_vscode" => {
            cmd::pat::edit_cfg_cpp_vscode(&".editorconfig".to_string(), &flags);
        },

        "std" => {
            let arg3 = args.get(3);
            let arg3 = if let Some(arg3) = arg3 {
                arg3.clone()
            } else {
                Config::load()?.get_solution_path()?
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
    };
    Ok(())
}

pub fn solo(flags: &Flags) -> Result<(), ()> {
    let config = Config::load()?;

    let mut hashes = Hashes::load(&flags);
    let build_res = cmd::build::all(&config, &mut hashes);
    Hashes::write(&mut hashes);
    build_res?;

    cmd::solo::solution(&Config::load()?)?;
    Ok(())
}

pub fn cfg(args: &Vec<String>, flags: &Flags) -> Result<(), ()> {
    let arg2 = args.get(2);
    let arg2 = if let Some(arg2) = arg2 {
        arg2.clone()
    } else {
        log::error("field", "not found");
        return Err(());
    };

    let arg3 = args.get(3);
    let arg3 = if let Some(arg3) = arg3 {
        arg3.clone()
    } else {
        log::error("path", "not found");
        return Err(());
    };

    utils::file::create_file(arg3.clone(), "", &flags);

    match arg2.as_str() {
        "sample" => {
            cmd::cfg::set_sample(&arg3)?;
        }
        _ => {
            log::error("field", "incorrect");
            return Err(());
        }
    }

    Ok(())
}