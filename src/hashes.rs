use serde::{Serialize, Deserialize};
use twox_hash::XxHash64;

use std::{fs, hash::Hasher, io::{Read, Write}};

use crate::{paths, Flags};

const SEED: u64 = 'p' as u64 + 'a' as u64 + 'x' as u64 + 'x' as u64;

pub fn get_hash_file(path: &String) -> u64 {
    let mut file = fs::File::open(path).unwrap();

    let mut hasher = XxHash64::with_seed(SEED);
    let mut buf = [0; 1024];
    loop {
        let len = file.read(&mut buf).expect("cannot read file while hashing");
        if len == 0 {
            break;
        }

        hasher.write(&buf[..len]);        
    }

    hasher.finish()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hashes {
    pub generator: Option<u64>,
    pub solution: Option<u64>,
    pub reference: Option<u64>,

    pub comparator: Option<u64>,
    pub checker: Option<u64>,

    pub generator_count: usize,
    pub solution_results_count: usize,
    pub reference_results_count: usize,
}

impl Hashes {
    pub fn load(flags: &Flags) -> Self {
        if flags.contains("r") {
            return Default::default();
        }
        let mut file = fs::File::open(paths::hashes()).unwrap();
        let mut hashes = String::new();
        file.read_to_string(&mut hashes).unwrap();
        serde_yml::from_str(hashes.as_str()).unwrap()
    }

    pub fn write(hashes: &mut Self) {
        let mut file = fs::File::create(paths::hashes()).unwrap();
        let hashes = serde_yml::to_string(hashes).unwrap();
        file.write_all(&hashes.as_bytes()).unwrap();
    }
}

impl Default for Hashes {
    fn default() -> Self {
        Self {
            generator: None,
            solution: None,
            reference: None,

            comparator: None,
            checker: None,

            generator_count: 0,
            solution_results_count: 0,
            reference_results_count: 0,
        }
    }
}


