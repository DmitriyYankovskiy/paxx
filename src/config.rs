use serde::{Serialize, Deserialize};

use std::{collections::HashMap, fs, io::{Read, Write}};

use crate::paths;

#[derive(Serialize, Deserialize, Debug)]
pub enum TestingType {
    DifferenceResults,
    CheckingResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub test_gen_path: String,
    pub solve_path: String,

    pub reference_path: Option<String>,

    pub diff_checker_path: Option<String>,
    pub res_checker_path: Option<String>,

    pub testing_type: TestingType,

    pub args: HashMap<String, Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        let mut args = HashMap::new();
        args.insert("cpp".to_string(), vec!["-D".to_string(), "LOCAL".to_string(), "-D".to_string(), "STRESS".to_string()]);
        Self {
            solve_path: String::from("solve.cpp"),
            reference_path: Some(String::from("reference.cpp")),            
            test_gen_path: String::from("test_gen.cpp"),

            diff_checker_path: Some(String::from("checker.cpp")),
            res_checker_path: None,            

            testing_type: TestingType::DifferenceResults,

            args,
        }
    }
}

impl Config {
    pub fn load() -> Self{
        let mut file = fs::File::open(paths::config()).unwrap();
        let mut config = String::new();
        file.read_to_string(&mut config).unwrap();
        serde_yml::from_str(config.as_str()).unwrap()
    }

    pub fn write(config: &mut Self) {
        let mut file = fs::File::create(paths::config()).unwrap();
        let config = serde_yml::to_string(config).unwrap();
        file.write_all(&config.as_bytes()).unwrap();
    }
}