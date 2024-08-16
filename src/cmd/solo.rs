use colored::Colorize;

use crate::{
    config::{Config, TestingMode}, utils::run::{self, RunResult},
    log,
};

pub fn solution(config: &Config) -> Result<String, ()> {
    log::status("solo ...");
    if let TestingMode::Manual = config.testing_mode {
        let path = config.solution_path.clone();
        
        let result = run::run(&path, Some(&format!("{}", &config.sample_path.clone().unwrap())), None, vec![]);
        if let Err(err) = result {
            if let Some(output) = err {
                print!("{}", output.bright_cyan());
            }

            return Err(())
        }
        let RunResult{output, duration: _} = result.unwrap();
        let output = output.unwrap();
        print!("{}", &output.bright_blue());
        Ok(output)
    } else {
        log::error("[Manual mode]", "not selected");
        Err(())
    }
}
