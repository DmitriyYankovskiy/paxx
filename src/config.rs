use serde::{Serialize, Deserialize};

use std::{collections::HashMap, fs, io::{Read, Write}, path::Path};

use crate::{out, paths, Language};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub solution_path: Option<String>,
    pub generator_path: Option<String>,

    pub reference_path: Option<String>,

    pub comparator_path: Option<String>,
    pub checker_path: Option<String>,

    pub sample_path: Option<String>,

    compile_rls_args: HashMap<Language, Vec<String>>,
    compile_dbg_args: HashMap<Language, Vec<String>>,

    compile_hdbg_add_args: HashMap<Language, Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        let compile_rls_args = HashMap::new();

        let mut compile_dbg_args = HashMap::new();
        compile_dbg_args.insert(Language::Cpp, vec![
            "-DDBG".to_string(),
        ]);

        let mut compile_hdbg_add_args = HashMap::new();
        compile_hdbg_add_args.insert(Language::Cpp, vec![
            "-fsanitize=address,undefined,bounds".to_string(),
            "-g".to_string(),
            "-D_GLIBCXX_DEBUG".to_string(),
            "-Wall".to_string(),
            "-Wextra".to_string(),
        ]);

        Self {
            solution_path: None,
            reference_path: None,            
            generator_path: None,

            comparator_path: None,
            sample_path: None,
            checker_path: None,

            compile_rls_args,
            compile_dbg_args,
            compile_hdbg_add_args,
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
            out::error("config path", "was corrupted");
            return Err(());
        };
        let mut config = String::new();
        file.read_to_string(&mut config).unwrap();
        let config: Config = match serde_yml::from_str(config.as_str()) {
            Ok(cfg) => cfg,
            Err(_) => {
                out::error("config file path", "incorrect");
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

        out::error("solution path", "not found");
        Err(())
    }

    pub fn get_sample_path(&self) -> Result<String, ()> {
        let path = self.sample_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        out::error("sample path", "not found");
        Err(())
    }

    pub fn get_generator_path(&self) -> Result<String, ()> {
        let path = self.generator_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        out::error("generator path", "not found");
        Err(())
    }

    pub fn get_checker_path(&self) -> Result<String, ()> {
        let path = self.checker_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        out::error("checker path", "not found");
        Err(())
    }
    

    pub fn get_reference_path(&self) -> Result<String, ()> {
        let path = self.reference_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        out::error("reference path", "not found");
        Err(())
    }

    pub fn get_comparator_path(&self) -> Result<String, ()> {
        let path = self.comparator_path.clone();
        if let Some(p) = path {
            if Path::new(&p).exists() {
                
                return Ok(p);
            }
        }

        out::error("comparator path", "not found");
        Err(())
    }
    
    
    
    pub fn get_compile_rls_args(&self, lang: Language) -> Vec<String> {
        if let Some(args) = self.compile_rls_args.get(&lang) {
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

    pub fn get_compile_hdbg_add_args(&self, lang: Language) -> Vec<String> {
        if let Some(args) = self.compile_hdbg_add_args.get(&lang) {
            args.clone()
        } else {
            Vec::new()
        }
    }
}