use crate::{
    config::{Config, TestingMode}, run
};

pub fn solution(config: &Config) -> Result<String, ()> {
    if let TestingMode::Manual = config.testing_mode {
        let path = config.solution_path.clone();
        
        run::run(&path, Some(&format!("{}", &config.sample_path.clone().unwrap())), None, vec![])
    } else {
        Err(())
    }
}