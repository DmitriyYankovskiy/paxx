use colored::Colorize;

use crate::{
    config::{Config, TestingMode}, run::{self, RunResult}
};

pub fn solution(config: &Config) -> Result<String, ()> {
    if let TestingMode::Manual = config.testing_mode {
        let path = config.solution_path.clone();
        
        let result = run::run(&path, Some(&format!("{}", &config.sample_path.clone().unwrap())), None, vec![]);
        if let Err(err) = result {
            if let Some(output) = err {
                print!("stdout:\n{}", output.bright_blue());
            }

            return Err(())
        }
        let RunResult{output, duration: _} = result.unwrap();
        let output = output.unwrap();
        print!("{}", &output.bright_blue());
        Ok(output)
    } else {
        println!("use {} mode", "Manual".bright_cyan().bold());
        Err(())
    }
}
