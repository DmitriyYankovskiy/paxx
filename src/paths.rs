use std::{fs, path::Path};

pub fn dir() -> String { format!(".paxx") }
pub fn tests_dir() -> String { format!("{}/tests", dir()) }
pub fn solves_results_dir() -> String { format!("{}/solves_results", dir()) }
pub fn ref_results_dir() -> String { format!("{}/ref_results", dir()) }
pub fn build_dir() -> String { format!("{}/build", dir()) }

pub fn config() -> String { format!("{}/config.yml", dir()) }
pub fn hashes() -> String { format!("{}/.hashes.yml", dir()) }

pub fn touch_dir(path: &String) {
    if !Path::new(path).exists() {
        fs::create_dir(path).unwrap();
    }
}

pub fn remove_all_in_dir(path: &String) {
    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();
}