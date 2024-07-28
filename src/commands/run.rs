use colored::{Color, Colorize};

use std::{cmp::min, fs, io::{BufReader, Read}, process::Command, str::from_utf8};

use crate::{
    config::{self, Config},
    paths, Flags,
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
        "cpp" | "c++" | "rs" => {
            let path = format!("{}/{}.exe", paths::build_dir(), path); 
            run_exe(&path.to_string(), input, output, args)
        }
        _ => {
            println!("{} {}", ext.bold().bright_red(), "is not executable".red());
            Err(())
        }
    }
}


pub fn all<'a>(tests_count: usize, errors_count: Option<usize>, config: &Config, flags: &'a Flags) -> Result<(), ()> {
    println!("{}", "running ...".bright_yellow());
    println!();
    let mut errors = Vec::<usize>::new();
    let created_tests_count = fs::read_dir(paths::tests_dir()).unwrap().count();
    let created_solution_res_count = fs::read_dir(paths::solution_results_dir()).unwrap().count();
    let created_reference_res_count = fs::read_dir(paths::ref_results_dir()).unwrap().count();

    'tests: for test_number in 1..=tests_count {
        if test_number > created_tests_count {
            test_gen(test_number, config)?;
        }
        if test_number > created_solution_res_count {
            solution(test_number, config)?;
        }

        let res = match config.testing_type {
            config::TestingType::CheckingResults => {
                let error = res_checker(test_number, tests_count, config, flags)?;
                error
            },
            config::TestingType::ComparisonResults => {
                if test_number > created_reference_res_count {
                    reference(test_number, config)?;
                }
                let error = comparator(test_number, tests_count, config, flags)?;
                error
            },
            config::TestingType::AutoComparisonResults => {
                if test_number > created_reference_res_count {
                    reference(test_number, config)?;
                }
                let error = auto_comparator(test_number, tests_count, flags)?;
                error
            },
        };

        if let Some(error) = res {
            errors.push(error);
        }

        if let Some(errors_count) = errors_count {
            if errors.len() >= errors_count {
                break 'tests;
            }
        }
    }

    println!();

    if !errors.is_empty() {
        println!("{}", "errors on test:".red());
        let cnt = min(errors.len(), 2);
        for i in 0..cnt {
            print!("{} ", format!("{}", errors[i]).bold().bright_red());
        }
        println!("{}", "... ".red());
    }


    Ok(())
}

fn test_gen(test_number: usize, config: &Config) -> Result<(), ()> {
    let path = config.test_gen_path.clone();
    run(&path, None, Some(&format!("{}/{}.dat", paths::tests_dir(), test_number)), vec![&test_number.to_string()])?;

    Ok(())
}

fn solution(test_number: usize, config: &Config) -> Result<(), ()> {
    let path = config.solution_path.clone();
    run(&path, Some(&format!("{}/{}.dat", paths::tests_dir(), test_number)), Some(&format!("{}/{}.dat", paths::solution_results_dir(), test_number)), vec![])?;

    Ok(())
}

fn get_verdict(test: usize, tests_count: usize, mut output: String, flags: &Flags) -> Result<bool, ()> {
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
            if !flags.contains("t") {
                println!("{}", format!("OK:{test_string} "));
            }
            Ok(false)
        }
        "ERR" => {
            println!("{} {}", format!("ER:{test_string} ").on_color(Color::TrueColor { r: 255, g: 0, b: 0 }).bold().color(Color::TrueColor { r: 0, g: 0, b: 0 }), comment);
            Ok(true)
        }
        _ => {
            println!("{} {}: {}", "incorrect".red(), "result checker verdict".bold().bright_red(), verdict);
            Err(())
        }
    }

}

fn res_checker(test_number: usize, tests_count: usize, config: &Config, flags: &Flags) -> Result<Option<usize>, ()> {
    let mut error = None;
    let path = config.res_checker_path.clone().unwrap();
    let output = run(&path, Some(&format!("{}/{}.dat", paths::solution_results_dir(), test_number)), None, vec![])?;
    
    if get_verdict(test_number, tests_count, output, flags)? {
        error = Some(test_number);
    }
    Ok(error)
}

fn reference(test_number: usize, config: &Config) -> Result<(), ()> {
    let path = config.reference_path.clone().unwrap();
    run(&path, Some(&format!("{}/{}.dat", paths::tests_dir(), test_number)), Some(&format!("{}/{}.dat", paths::ref_results_dir(), test_number)), vec![])?;

    Ok(())
}

fn comparator(test_number: usize, tests_count: usize, config: &Config, flags: &Flags) -> Result<Option<usize>, ()> {
    let mut error = None;
    let path = config.comparator_path.clone().unwrap();
    let input_solution = format!("{}/{}.dat", paths::solution_results_dir(), &test_number.to_string());
    let input_ref = format!("{}/{}.dat", paths::ref_results_dir(), &test_number.to_string());
    let output = run(&path, None, None, vec![&input_solution, &input_ref])?;
    
    if get_verdict(test_number, tests_count, output, flags)? {
        error = Some(test_number);
    }
    Ok(error)
} 

fn auto_comparator(test_number: usize, tests_count: usize, flags: &Flags) -> Result<Option<usize>, ()> {
    let mut error = None;
    let input_solution = format!("{}/{}.dat", paths::solution_results_dir(), &test_number.to_string());
    let input_ref = format!("{}/{}.dat", paths::ref_results_dir(), &test_number.to_string());

    let mut reader_solution = BufReader::new(fs::File::open(&input_solution).unwrap());
    let mut reader_ref = BufReader::new(fs::File::open(&input_ref).unwrap());

    let mut output = String::from("OK");

    loop {
        let mut buf_solution = [0; 1024];
        let mut buf_ref = [0; 1024];

        let len_solution = reader_solution.read(&mut buf_solution).unwrap();
        let len_ref = reader_ref.read(&mut buf_ref).unwrap();

        if buf_solution != buf_ref {
            output = String::from("ERR");
        }

        if len_ref == 0 || len_solution == 0 {
            break;
        }
    }

    
    if get_verdict(test_number, tests_count, output, flags)? {
        error = Some(test_number);
    }
    Ok(error)
} 