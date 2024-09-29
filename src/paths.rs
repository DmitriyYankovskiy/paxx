use std::{fs, path::Path};

pub fn dir() -> String { format!("paxx") }
pub fn tests_dir() -> String { format!("{}/.tests", dir()) }
pub fn solution_results_dir() -> String { format!("{}/.solution_results", dir()) }
pub fn ref_results_dir() -> String { format!("{}/.ref_results", dir()) }
pub fn build_dir() -> String { format!("{}/.build", dir()) }

pub fn config() -> String { format!("{}/config.yml", dir()) }
pub fn hashes() -> String { format!("{}/.hashes.yml", dir()) }

pub fn readme() -> String { format!("{}/README.md", dir()) }

pub fn touch_dir(path: &String) {
    if !Path::new(path).exists() {
        fs::create_dir(path).unwrap();
    }
}