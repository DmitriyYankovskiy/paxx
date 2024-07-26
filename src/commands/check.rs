use colored::Colorize;

use std::path::Path;

use crate::config::{Config, TestingType};

pub fn all(config: &Config) -> bool {
    let mut error = false;
    if !Path::new(&config.test_gen_path).exists() {
        error = true;
        println!("{} {}", "tests generator".bold().bright_red(), "not found".red());
    }
    if !Path::new(&config.solve_path).exists() {
        error = true;
        println!("{} {}", "solve code".bold().bright_red(), "not found".red());
    }
    match config.testing_type {
        TestingType::CheckingResults => {
            if config.res_checker_path == None || !Path::new(&config.res_checker_path.clone().unwrap()).exists() {
                error = true;
                println!("{} {}", "result checker".bold().bright_red(), "not found".red());
            }
        }

        TestingType::DifferenceResults => {
            if config.reference_path == None || !Path::new(&config.reference_path.clone().unwrap()).exists() {
                error = true;
                println!("{} {}", "reference code".bold().bright_red(), "not found".red());;
            }

            if config.diff_checker_path == None || !Path::new(&config.diff_checker_path.clone().unwrap()).exists() {
                error = true;
                println!("{} {}", "difference checker".bold().bright_red(), "not found".red());
            }
        }
    }

    error
}