use colored::Colorize;
use serde::{Serialize, Deserialize};

use std::{collections::HashMap, fs, io::{Read, Write}, path::Path};

use crate::paths;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TestingType {
    CheckingResults,
    ComparisonResults,
    AutoComparisonResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub test_gen_path: String,
    pub solution_path: String,

    pub reference_path: Option<String>,

    pub comparator_path: Option<String>,
    pub res_checker_path: Option<String>,

    pub testing_type: TestingType,

    pub args: HashMap<String, Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        let mut args = HashMap::new();
        args.insert("cpp".to_string(), vec![
            "-D".to_string(),
            "STRESS".to_string()],
        );
        args.insert("c++".to_string(), vec![
            "-D".to_string(),
            "STRESS".to_string()],
        );
        args.insert("rs".to_string(), vec![]);
        Self {
            solution_path: String::from("solution.cpp"),
            reference_path: Some(String::from("ref.cpp")),            
            test_gen_path: String::from("gen.cpp"),

            comparator_path: Some(String::from("check.cpp")),
            res_checker_path: None,            

            testing_type: TestingType::ComparisonResults,

            args,
        }
    }
}

impl Config {
    pub fn write(config: &mut Self) {
        let mut file = fs::File::create(paths::config()).unwrap();
        let config = serde_yml::to_string(config).unwrap();
        file.write_all(&config.as_bytes()).unwrap();
    }

    pub fn load() -> Result<Self, ()> {
        let mut file = fs::File::open(paths::config()).unwrap();
        let mut config = String::new();
        file.read_to_string(&mut config).unwrap();
        let config: Config = match serde_yml::from_str(config.as_str()) {
            Ok(cfg) => cfg,
            Err(_) => return Err(()),
        };

        let mut error = false;
        if !Path::new(&config.test_gen_path).exists() {
            error = true;
            println!("{} {}", "tests generator".bold().bright_red(), "not found".red());
        }
        if !Path::new(&config.solution_path).exists() {
            error = true;
            println!("{} {}", "solution code".bold().bright_red(), "not found".red());
        }

        let testing_type = config.testing_type;
        match testing_type {
            TestingType::CheckingResults => {
                if config.res_checker_path == None || !Path::new(&config.res_checker_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "result checker".bold().bright_red(), "not found".red());
                }
            }

            _ => {}
        }

        match testing_type {
            TestingType::ComparisonResults | TestingType::AutoComparisonResults => {
                if config.reference_path == None || !Path::new(&config.reference_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "reference code".bold().bright_red(), "not found".red());
                }
            }

            _ => {}
        }

        match testing_type {
            TestingType::ComparisonResults => {
                if config.comparator_path == None || !Path::new(&config.comparator_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "comparator".bold().bright_red(), "not found".red());
                }
            }

            _ => {}
        }
        
        if error {
            Err(())
        } else {
            Ok(config)
        }
    }
}