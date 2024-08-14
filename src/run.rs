use colored::Colorize;

use std::{fs, process::Command, str::from_utf8};

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

pub fn run(path: &String, input: Option<&String>, output: Option<&String>, args: Vec<&String>) -> Result<String, ()> {   
    let mut cmd = &mut run_cmd(path)?;

    if let Some(input) = input {
        cmd = cmd.stdin(fs::File::open(input).unwrap());
    }
    if let Some(output) = output {
        cmd = cmd.stdout(fs::File::create(output).unwrap());
    }
    if !args.is_empty() {
        cmd = cmd.args(args);
    }
    if output == None {
        let output = cmd.output();
        if let Ok(output) = output{
            if output.status.success() {
                if let Ok(stdout) = from_utf8(&output.stdout) {
                    Ok(stdout.to_string())
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    } else {
        let status = cmd.status();
        if let Ok(_) = status {
            Ok(String::new())
        } else {
            println!("{} execute with error: {:?}", path.bold().bright_red(), status);
            Err(())
        }
    }
}

