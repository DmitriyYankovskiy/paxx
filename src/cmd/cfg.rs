use colored::Colorize;

use crate::{config::Config, out};

pub fn set_solution(path: &String) -> Result<(), ()> {
    let mut config = Config::load()?;
    config.solution_path = Some(path.clone());
    out::ok("cfg: solution_path", "set on:");
    println!("---> {}", path.blue().bold());
    Config::write(&mut config);
    Ok(())
}

pub fn set_generator(path: &String) -> Result<(), ()> {
    let mut config = Config::load()?;
    config.generator_path = Some(path.clone());
    out::ok("cfg: test_gen_path", "set on:");
    println!("---> {}", path.blue().bold());
    Config::write(&mut config);
    Ok(())
}

pub fn set_sample(path: &String) -> Result<(), ()> {
    let mut config = Config::load()?;
    config.sample_path = Some(path.clone());
    out::ok("cfg: sample_path", "set on:");
    println!("---> {}", path.blue().bold());
    Config::write(&mut config);
    Ok(())
}