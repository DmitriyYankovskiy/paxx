use std::{fs, process::Command, str::from_utf8, time::{self, Duration}};

use crate::{out::{self, error}, paths, Language};

fn run_cmd(path: &str) -> Result<Command, ()> {
    let (name, ext) = path.split_once(".").unwrap();
    let ext = {let lang = Language::from_ext(ext)?; lang.get_executable_ext()};
    Ok(match ext {
        "exe" => Command::new(format!("./{}/{name}.{ext}", paths::build_dir())),
        "py" => {
            let mut cmd = Command::new("python3");
            cmd.arg(format!("{}/{name}.{ext}", paths::build_dir())); 
            cmd
        },
        _ => {
            error(&ext, "is not executable");
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
        let output = cmd.output().unwrap();
        if output.status.success() {
            if let Ok(stdout) = from_utf8(&output.stdout) {
                Ok(RunResult{
                    output: Some(stdout.to_string()),
                    duration: start_time.elapsed(),
                })
            } else {
                out::error("code output", "incorrect");
                Err(None)
            }
        } else {
            out::error(&path, "execute with error");
            println!("{:#?}", output.status);
            let output =  String::from_utf8(output.stdout);
            let output = if let Ok(o) = output {
                o
            } else {
                out::error(&path, "incorrect output");
                return Err(None);
            };
            Err(Some(output))
        }
    } else {
        let status = cmd.status();
        if let Ok(_) = status {
            Ok(RunResult {
                output: None,
                duration: start_time.elapsed(),
            })
        } else {
            out::error(&path, "execute with error");
            println!("{:#?}", status);
            Err(None)
        }
    }
}

