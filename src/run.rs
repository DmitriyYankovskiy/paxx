use colored::Colorize;

use std::{fs, process::Command, str::from_utf8, time::{self, Duration}};

use crate::{paths, compile};

fn run_cmd(path: &str) -> Result<Command, ()> {
    let (path, ext) = path.split_once(".").unwrap();
    let ext = compile::executable_ext(ext);
    Ok(match ext.as_str() {
        "exe" => Command::new(format!("./{}/{path}.{ext}", paths::build_dir())),
        "py" => {
            let mut cmd = Command::new("python3");
            cmd.arg(format!("{}/{path}.{ext}", paths::build_dir())); 
            cmd
        },
        _ => {
            println!("{} {}", ext.bold().bright_red(), "is not executable".red());
            return Err(());
        }
    })
}

pub struct RunResult {
    pub output: Option<String>,
    pub duration: Duration,
}

pub fn run(path: &String, input: Option<&String>, output: Option<&String>, args: Vec<&String>) -> Result<RunResult, Option<String>> {   
    let cmd = run_cmd(path);
    let mut cmd = if let Ok(cmd) = cmd {
        cmd
    } else {
        return Err(None);
    };

    if let Some(input) = input {
        cmd.stdin(fs::File::open(input).unwrap());
    }
    if let Some(output) = output {
        cmd.stdout(fs::File::create(output).unwrap());
    }
    if !args.is_empty() {
        cmd.args(args);
    }
    let start_time = time::Instant::now();
    if output == None {
        let output = cmd.output();
        if let Ok(output) = output{
            if output.status.success() {
                if let Ok(stdout) = from_utf8(&output.stdout) {
                    Ok(RunResult{
                        output: Some(stdout.to_string()),
                        duration: start_time.elapsed(),
                    })
                } else {
                    println!("{}", "incorrect output".red().bold());
                    Err(None)
                }
            } else {
                println!("{} execute with error: {:#?}", path.bold().bright_red(), output.status);
                let output =  String::from_utf8(output.stdout);
                let output = if let Ok(o) = output {
                    o
                } else {
                    println!("{} incorrect output", path.bold().bright_red());
                    return Err(None);
                };
                Err(Some(output))
            }
        } else {
            println!("{}", "---".red());
            Err(None)
        }
    } else {
        let status = cmd.status();
        if let Ok(_) = status {
            Ok(RunResult {
                output: None,
                duration: start_time.elapsed(),
            })
        } else {
            println!("{} execute with error: {:#?}", path.bold().bright_red(), status);
            Err(None)
        }
    }
}

