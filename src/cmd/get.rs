use colored::Colorize;

use std::{fs, io::{BufRead, BufReader}};

use crate::{config::Config, log::{self, error}, paths};

pub fn all(number: usize, config: &Config) -> Result<(), ()> {
    test(number)?;
    solution_result(number)?;
    match config.testing_mode {
        crate::config::TestingMode::ComparisonResults | crate::config::TestingMode::AutoComparisonResults => {
            ref_result(number)?
        }
        _ => {}
    }

    Ok(())
}

fn test(number: usize) -> Result<(), ()> {    
    let file = if let Ok(f) = fs::File::open(&format!("{}/{}.dat", paths::tests_dir(), number)) {
        f
    } else {
        error("tests", "corrupted");
        return Err(());
    };
    println!();
    println!("{}", "test:".bold().cyan());
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", format!("{line}").bright_blue());
    }

    println!();

    Ok(())
}

fn solution_result(number: usize) -> Result<(), ()> {
    let file = if let Ok(f) = fs::File::open(&format!("{}/{}.dat", paths::solution_results_dir(), number)) {
        f
    } else {
        log::error("solution results", "corrupted");
        return Err(());
    };
    println!("{}", "solution_result:".bold().cyan());
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", format!("{line}").white());
    }

    println!();

    Ok(())
}

fn ref_result(number: usize) -> Result<(), ()> {
    let file = if let Ok(f) = fs::File::open(&format!("{}/{}.dat", paths::ref_results_dir(), number)) {
        f
    } else {
        log::error("reference results", "corrupted");
        return Err(());
    };

    println!("{}", "ref_result:".bold().cyan());
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", format!("{line}").bright_green());
    }

    println!();

    Ok(())
}
