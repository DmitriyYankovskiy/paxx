use colored::Colorize;
use serde::{Serialize, Deserialize};

use std::{collections::HashMap, fs, io::{Read, Write}, path::Path};

use crate::paths;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TestingMode {
    Manual,
    CheckingResults,
    ComparisonResults,
    AutoComparisonResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub solution_path: String,
    pub test_gen_path: Option<String>,

    pub reference_path: Option<String>,

    pub comparator_path: Option<String>,
    pub res_checker_path: Option<String>,

    pub testing_mode: TestingMode,

    pub sample_path: Option<String>,

    pub args: HashMap<String, Vec<String>>,

    pub solo_args: HashMap<String, Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        let mut args = HashMap::new();
        args.insert("c++".to_string(), vec![
            "-D".to_string(),
            "STRESS".to_string()],
        );
        args.insert("rust".to_string(), vec![]);

        let mut solo_args = HashMap::new();
        solo_args.insert("c++".to_string(), vec![
            "-D".to_string(),
            "SOLO".to_string()],
        );
        solo_args.insert("rust".to_string(), vec![]);
        Self {
            solution_path: String::from("solution.cpp"),
            reference_path: None,            
            test_gen_path: None,

            comparator_path: None,
            sample_path: None,
            res_checker_path: None,            

            testing_mode: TestingMode::Manual,

            args,
            solo_args,
        }
    }
}

impl Config {
    pub fn write(config: &mut Self) {
        let mut file = fs::File::create(paths::config()).unwrap();
        let config = serde_yml::to_string(config).unwrap();
        file.write_all(&config.as_bytes()).unwrap();
    }

    fn check(&self) -> Result<(), ()> {
        let mut error = false;
        if !Path::new(&self.solution_path).exists() {
            error = true;
            println!("{} {}", "solution code".bold().bright_red(), "not found".red());
        }

        let testing_mode = self.testing_mode;

        match testing_mode {
            TestingMode::Manual => {
                if self.sample_path == None || !Path::new(&self.sample_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "sample".bold().bright_red(), "not found".red());
                }
            }
            _ => {}
        }

        match testing_mode {
            TestingMode::ComparisonResults | TestingMode::CheckingResults | TestingMode::AutoComparisonResults => {
                if self.test_gen_path == None || !Path::new(&self.test_gen_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "tests generator".bold().bright_red(), "not found".red());
                }
            }

            _ => {}
        }

        match testing_mode {
            TestingMode::CheckingResults => {
                if self.res_checker_path == None || !Path::new(&self.res_checker_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "result checker".bold().bright_red(), "not found".red());
                }
            }

            _ => {}
        }

        match testing_mode {
            TestingMode::ComparisonResults | TestingMode::AutoComparisonResults => {
                if self.reference_path == None || !Path::new(&self.reference_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "reference code".bold().bright_red(), "not found".red());
                }
            }

            _ => {}
        }

        match testing_mode {
            TestingMode::ComparisonResults => {
                if self.comparator_path == None || !Path::new(&self.comparator_path.clone().unwrap()).exists() {
                    error = true;
                    println!("{} {}", "comparator".bold().bright_red(), "not found".red());
                }
            }

            _ => {}
        }
        
        if error {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn load() -> Result<Self, ()> {
        let mut file = fs::File::open(paths::config()).unwrap();
        let mut config = String::new();
        file.read_to_string(&mut config).unwrap();
        let config: Config = match serde_yml::from_str(config.as_str()) {
            Ok(cfg) => cfg,
            Err(_) => {
                println!("{}", "incorrect config file".bright_red().bold());
                return Err(())
            },
        };
        Ok(config)
    }

    pub fn load_and_check() -> Result<Self, ()> {
        let config = Self::load()?;
        config.check()?;
        Ok(config)
    }

    pub fn get_args(&self, language: &str) -> Vec<String> {
        match self.testing_mode {
            TestingMode::Manual => {
                if let Some(args) = self.solo_args.get(language) {
                    args.clone()
                } else {
                    Vec::new()
                }
            }
            _ => {
                if let Some(args) = self.args.get(language) {
                    args.clone()
                } else {
                    Vec::new()
                }
            }
        }
    }
}