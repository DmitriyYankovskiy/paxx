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
    let RunResult{stdout, stderr, duration, ..} = result.unwrap();
    let output = stdout.unwrap();
    println!("{}", "stdout:".bright_blue());
    print!("{}", &output);

    if let Some(stderr) = stderr {
        println!("{}", "stderr:".bright_red());
        print!("{}", stderr);
    }

    println!("{} {}", "executing time:".bright_green(), duration.as_secs_f32());

    Ok(output)
}
