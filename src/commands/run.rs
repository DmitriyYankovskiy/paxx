use colored::{Color, Colorize};

use std::{fs, process::{Command, Output}, str::from_utf8};

use crate::{
    config::{self, Config},
    paths,
};

fn run_exe(path: &String, input: Option<&String>, output: Option<&String>, args: Vec<&String>) -> Result<String, ()> {
    let mut cmd = &mut Command::new(format!("./{}", path));
    if let Some(input) = input {
        cmd = cmd.stdin(fs::File::open(input).unwrap());
    }
    if let Some(output) = output {
        cmd = cmd.stdout(fs::File::create(output).unwrap());
    }
    cmd = cmd.args(args);
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
            Err(())
        }
    }
    
}

fn run(path: &String, input: Option<&String>, output: Option<&String>, args: Vec<&String>, ) -> Result<String, ()> {
    let (path, ext) = path.split_once(".").unwrap();
    match ext {
        "cpp" | "c++" => {
            let path = format!("{}/{}.exe", paths::build_dir(), path); 
            run_exe(&path.to_string(), input, output, args)
        }
        _ => {
            println!("{} {}", ext.bold().bright_red(), "is not executable".red());
            Err(())
        }
    }
}


pub fn all(tests_count: usize, config: &Config) -> Result<Vec<usize>, ()> {
    println!("{}", "* running ...".bright_yellow());

    test_gen(tests_count, config)?;
    solve(tests_count, config)?;
    
    match config.testing_type {
        config::TestingType::CheckingResults => {
            let errors = res_checker(tests_count, config)?;
            Ok(errors)
        },
        config::TestingType::DifferenceResults => {
            Ok(vec![])
        },
    }
}

fn test_gen(tests_count: usize, config: &Config) -> Result<(), ()> {
    println!("{}", "** test generating ...".yellow());
    let count = fs::read_dir(paths::tests_dir()).unwrap().count();
    let path = config.test_gen_path.clone();
    if count < tests_count {
        for test in count + 1..=tests_count {
            run(&path, None, Some(&format!("{}/{}.dat", paths::tests_dir(), test)), vec![&test.to_string()])?;
        } 
    }

    println!("{}", "** test succesfully generated".yellow());

    Ok(())
}

fn solve(tests_count: usize, config: &Config) -> Result<(), ()> {
    println!("{}", "** solving tests ...".yellow());
    let count = fs::read_dir(paths::solves_results_dir()).unwrap().count();
    let path = config.solve_path.clone();
    if count < tests_count {
        for test in count + 1..=tests_count {
            run(&path, Some(&format!("{}/{}.dat", paths::tests_dir(), test)), Some(&format!("{}/{}.dat", paths::solves_results_dir(), test)), vec![])?;
        } 
    }

    println!("{}", "** test succesfully solved".yellow());

    Ok(())
}

fn res_checker(tests_count: usize, config: &Config) -> Result<Vec<usize>, ()> {
    let mut errors = vec![];
    let path = config.res_checker_path.clone().unwrap();
    for test in 1..= tests_count {
        let mut output = run(&path, Some(&format!("{}/{}.dat", paths::solves_results_dir(), test)), None, vec![])?;
        output.push(' ');
        let (verdict, comment) = match output.split_once(" ") {
            Some(vc) => vc,
            None => {
                println!("{} {}: {}", "incorrect".red(), "result checker output".bold().bright_red(), output);
                return Err(());
            }
        };

        let comment = comment.trim();

        let space_cnt = tests_count.to_string().len() - test.to_string().len();
        let test_string = format!("{}{}", " ".repeat(space_cnt), test.to_string());
        match verdict.trim() {
            "OK" => {
                println!("{}", format!("OK:{test_string}").on_color(Color::TrueColor { r: (35), g: (255), b: (50) }));
            }
            "ERR" => {
                println!("{} {}", format!("ER:{test_string}").on_color(Color::TrueColor { r: (255), g: (0), b: (0) }).bold(), comment);
                errors.push(test);
            }
            _ => {
                println!("{} {}: {}", "incorrect".red(), "result checker verdict".bold().bright_red(), verdict);
                return Err(());
            }
        }
    }
    Ok(errors)
}