use serde::{Serialize, Deserialize};

use std::{collections::HashMap, fs, io::{Read, Write}, path::Path};

use crate::{log, paths, Language};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub solution_path: Option<String>,
    pub generator_path: Option<String>,

    pub reference_path: Option<String>,

    pub comparator_path: Option<String>,
    pub checker_path: Option<String>,

    pub sample_path: Option<String>,

    compile_std_args: HashMap<Language, Vec<String>>,
    compile_dbg_args: HashMap<Language, Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        let mut compile_std_args = HashMap::new();
        compile_std_args.insert(Language::Cpp, vec![
            "-D".to_string(),
            "STRESS".to_string()],
        );
        compile_std_args.insert(Language::Rust, vec![]);

        let mut compile_dbg_args = HashMap::new();
        compile_dbg_args.insert(Language::Cpp, vec![
            "-D".to_string(),
            "SOLO".to_string()],
        );
        compile_dbg_args.insert(Language::Rust, vec![]);
        Self {
            solution_path: None,
            reference_path: None,            
            generator_path: None,

            comparator_path: None,
            sample_path: None,
            checker_path: None,

            compile_std_args,
            compile_dbg_args,
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
        let mut file = if let Ok(f) = fs::File::open(paths::config()) {
            f
        } else {
            log::error("config path", "was corrupted");
            return Err(());
        };
        let mut config = String::new();
        file.read_to_string(&mut config).unwrap();
        let config: Config = match serde_yml::from_str(config.as_str()) {
            Ok(cfg) => cfg,
            Err(_) => {
                log::error("config file path", "incorrect");
                return Err(())
            },
        };
        Ok(config)
    }

    pub fn get_solution_path(&self) -> Result<String, ()> {
        let path = self.solution_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        log::error("solution path", "not found");
        Err(())
    }

    pub fn get_sample_path(&self) -> Result<String, ()> {
        let path = self.sample_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        log::error("sample path", "not found");
        Err(())
    }

    pub fn get_generator_path(&self) -> Result<String, ()> {
        let path = self.generator_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        log::error("generator path", "not found");
        Err(())
    }

    pub fn get_checker_path(&self) -> Result<String, ()> {
        let path = self.checker_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        log::error("checker path", "not found");
        Err(())
    }
    

    pub fn get_reference_path(&self) -> Result<String, ()> {
        let path = self.reference_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        log::error("reference path", "not found");
        Err(())
    }

    pub fn get_comparator_path(&self) -> Result<String, ()> {
        let path = self.comparator_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        log::error("comparator path", "not found");
        Err(())
    }
    
    
    
    pub fn get_compile_std_args(&self, lang: Language) -> Vec<String> {
        if let Some(args) = self.compile_std_args.get(&lang) {
            args.clone()
        } else {
            Vec::new()
        }
    }

    pub fn get_compile_dbg_args(&self, lang: Language) -> Vec<String> {
        if let Some(args) = self.compile_dbg_args.get(&lang) {
            args.clone()
        } else {
            Vec::new()
        }
    }
}