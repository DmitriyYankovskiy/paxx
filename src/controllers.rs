use std::fs;

use crate::{cmd::{self, stress}, config::Config, hashes::Hashes, out, paths, utils, Flags};

pub fn init() {
    cmd::init::all();
}

pub fn check(flags: &Flags) {
    if flags.contains("re") {
        Config::default();
    } 
    if let Ok(_) = Config::load() {
        out::ok("config", "is valid");
    }
}

pub fn stress(args: &mut utils::arg::Args, flags: &Flags) -> Result<(), ()> {
    let mode = args.get("mode")?.parse()?;

    let test_count = args.get("test count")?;
    let tests_count: usize = match test_count.parse() {
        Ok(count) => count,
        Err(_) => {
            out::error("tests_count", "incorrect");
            return Err(());
        }
    };

    let config = Config::load()?;

    let mut hashes = Hashes::connect(flags);
    cmd::stress::build::all(&config, &mut hashes, mode)?;

    cmd::stress::all(tests_count, stress::StopMode::All, mode, &config, &mut hashes, flags)?;
    Ok(())
}

pub fn catch(args: &mut utils::arg::Args, flags: &Flags) -> Result<(), ()> {
    let mode = args.get("mode")?;
    let mode = match mode.as_str() {
        "check" => stress::Mode::Check,
        "comp" => stress::Mode::Compare,
        "acomp" => stress::Mode::AutoCompare,
        _ => {out::error("mode", "incorrect"); return Err(())}
    };

    let mistakes_cap = args.get("mistakes cap")?;
    let mistakes_cap: usize = match mistakes_cap.parse() {
        Ok(count) => count,
        Err(_) => {
            out::error("tests_count", "incorrect");
            return Err(());
        }
    };

    let test_count = args.get("test count")?;
    let tests_count: usize = match test_count.parse() {
        Ok(count) => count,
        Err(_) => {
            out::error("tests_count", "incorrect");
            return Err(());
        }
    };

    let config = Config::load()?;

    let mut hashes = Hashes::connect(flags);
    cmd::stress::build::all(&config, &mut hashes, mode)?;

    cmd::stress::all(tests_count, stress::StopMode::UpToMistake(mistakes_cap), mode, &config, &mut hashes, flags)?;
    Ok(())
}

pub fn remove() {
    fs::remove_dir_all(paths::dir()).unwrap();
}

pub fn get(args: &mut utils::arg::Args) -> Result<(), ()> {
    let test_number: usize = match args.get("test number")?.parse() {
        Ok(count) => count,
        Err(_) => {
            out::error("tests number", "incorrect");
            return Err(());
        }
    };

    let mode = args.get("mode")?.parse()?;


    cmd::stress::get::all(test_number, mode)?;
    Ok(())
}

pub fn pat(args: &mut utils::arg::Args, flags: &Flags) -> Result<(), ()> {
    let pattern = args.get("pattern")?;


    match pattern.as_str() {
        "generator" => {
            let path = if let Some(p) = args.try_get() {
                p.clone()
            } else {
                Config::load()?.get_generator_path()?
            };

            cmd::pat::gen(&path, &flags);
            if flags.contains("set") {
                cmd::cfg::set_test_gen(&path)?;
            }
        }
        "edit_cfg_c++_vscode" => {
            cmd::pat::edit_cfg_cpp_vscode(&".editorconfig".to_string(), &flags);
        }
        "solution" => {
            let path = if let Some(p) = args.try_get() {
                p.clone()
            } else {
                Config::load()?.get_generator_path()?
            };
            cmd::pat::std(&path, &flags);

            if flags.contains("set") {
                cmd::cfg::set_solution(&path)?;
            }
        }
        _ => {
            out::error("pattern", "incorrect");
            return Err(());
        }
    };
    Ok(())
}

pub fn run(flags: &Flags) -> Result<(), ()> {
    let config = Config::load()?;

    let mut hashes = Hashes::connect(&flags);
    cmd::run::build::all(&config, &mut hashes)?;

    cmd::run::solution(&Config::load()?)?;
    Ok(())
}

pub fn cfg(args: &mut utils::arg::Args, flags: &Flags) -> Result<(), ()> {
    let field = args.get("field")?;

    let path = args.get("path")?;

    utils::file::create_file(path.clone(), "", &flags);

    match field.as_str() {
        "sample" => {
            cmd::cfg::set_sample(&path)?;
        }
        _ => {
            out::error("field", "incorrect");
            return Err(());
        }
    }

    Ok(())
}