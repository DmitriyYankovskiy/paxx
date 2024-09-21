pub mod build;

use colored::Colorize;

use crate::{
    config::Config, utils::run::{self, RunResult},
    out,
};

pub fn solution(config: &Config) -> Result<String, ()> {
    out::status("run ...");
    let path = config.get_solution_path()?;
    
    let result = run::run(&path, Some(&format!("{}", &config.get_sample_path()?)), None, vec![]);
    if let Err(err) = result {
        if let Some(output) = err {
            print!("{}", output.bright_cyan());
        }

        return Err(())
    }
    let RunResult{output, duration: _} = result.unwrap();
    let output = output.unwrap();
    println!("output:");
    print!("{}", &output.bright_blue());
    Ok(output)
}
