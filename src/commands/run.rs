use colored::{Color, Colorize};

use std::{cmp::min, fs, io::{BufReader, Read}, process::Command, str::from_utf8};

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


pub fn all(tests_count: usize, config: &Config) -> Result<(), ()> {
    println!("{}", "running ...".bright_yellow());

    test_gen(tests_count, config)?;
    solve(tests_count, config)?;
    
    let res = match config.testing_type {
        config::TestingType::CheckingResults => {
            let errors = res_checker(tests_count, config)?;
            errors
        },
        config::TestingType::ComparisonResults => {
            reference(tests_count, config)?;
            let errors = comparator(tests_count, config)?;
            errors
        },
        config::TestingType::AutoComparisonResults => {
            reference(tests_count, config)?;
            let errors = auto_comparator(tests_count)?;
            errors
        },
    };

    println!();

    if !res.is_empty() {
        println!("{}", "errors on test:".red());
        let cnt = min(res.len(), 2);
        for i in 0..cnt {
            print!("{} ", format!("{}", res[i]).bold().bright_red());
        }
        println!("{}", "... ".red());
    }


    Ok(())
}

fn test_gen(tests_count: usize, config: &Config) -> Result<(), ()> {
    println!("{}", " - test generating ...");
    let count = fs::read_dir(paths::tests_dir()).unwrap().count();
    let path = config.test_gen_path.clone();
    if count < tests_count {
        for test in count + 1..=tests_count {
            run(&path, None, Some(&format!("{}/{}.dat", paths::tests_dir(), test)), vec![&test.to_string()])?;
        } 
    }

    Ok(())
}

fn solve(tests_count: usize, config: &Config) -> Result<(), ()> {
    println!("{}", " - solving tests ...");
    let count = fs::read_dir(paths::solves_results_dir()).unwrap().count();
    let path = config.solve_path.clone();
    if count < tests_count {
        for test in count + 1..=tests_count {
            run(&path, Some(&format!("{}/{}.dat", paths::tests_dir(), test)), Some(&format!("{}/{}.dat", paths::solves_results_dir(), test)), vec![])?;
        } 
    }

    Ok(())
}

fn get_verdict(test: usize, tests_count: usize, mut output: String) -> Result<bool, ()> {
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
            Ok(false)
        }
        "ERR" => {
            println!("{} {}", format!("ER:{test_string}").on_color(Color::TrueColor { r: (255), g: (0), b: (0) }).bold(), comment);
            Ok(true)
        }
        _ => {
            println!("{} {}: {}", "incorrect".red(), "result checker verdict".bold().bright_red(), verdict);
            Err(())
        }
    }

}

fn res_checker(tests_count: usize, config: &Config) -> Result<Vec<usize>, ()> {
    println!("{}", " - result checking ...");
    let mut errors = vec![];
    let path = config.res_checker_path.clone().unwrap();
    for test in 1..= tests_count {
        let output = run(&path, Some(&format!("{}/{}.dat", paths::solves_results_dir(), test)), None, vec![])?;
       
        if get_verdict(test, tests_count, output)? {
            errors.push(test);
        }
    }
    Ok(errors)
}

fn reference(tests_count: usize, config: &Config) -> Result<(), ()> {
    println!("{}", " - reference solve tests ...");
    let count = fs::read_dir(paths::ref_results_dir()).unwrap().count();
    let path = config.reference_path.clone().unwrap();
    if count < tests_count {
        for test in count + 1..=tests_count {
            run(&path, Some(&format!("{}/{}.dat", paths::tests_dir(), test)), Some(&format!("{}/{}.dat", paths::ref_results_dir(), test)), vec![])?;
        } 
    }

    Ok(())
}

fn comparator(tests_count: usize, config: &Config) -> Result<Vec<usize>, ()> {
    println!("{}", " - comparation ...");
    let mut errors = vec![];
    let path = config.comparator_path.clone().unwrap();
    for test in 1..= tests_count {
        let input_solve = format!("{}/{}.dat", paths::solves_results_dir(), &test.to_string());
        let input_ref = format!("{}/{}.dat", paths::ref_results_dir(), &test.to_string());
        let output = run(&path, None, None, vec![&input_solve, &input_ref])?;
        
        if get_verdict(test, tests_count, output)? {
            errors.push(test);
        }
    }
    Ok(errors)
} 

fn auto_comparator(tests_count: usize) -> Result<Vec<usize>, ()> {
    println!("{}", " - comparation ...");
    let mut errors = vec![];
    for test in 1..= tests_count {
        let input_solve = format!("{}/{}.dat", paths::solves_results_dir(), &test.to_string());
        let input_ref = format!("{}/{}.dat", paths::ref_results_dir(), &test.to_string());

        let mut reader_solve = BufReader::new(fs::File::open(&input_solve).unwrap());
        let mut reader_ref = BufReader::new(fs::File::open(&input_ref).unwrap());

        let mut output = String::from("OK");

        loop {
            let mut buf_solve = [0; 1024];
            let mut buf_ref = [0; 1024];

            let len_solve = reader_solve.read(&mut buf_solve).unwrap();
            let len_ref = reader_ref.read(&mut buf_ref).unwrap();

            if buf_solve != buf_ref {
                output = String::from("ERR");
            }

            if len_ref == 0 || len_solve == 0 {
                break;
            }
        }

        
        if get_verdict(test, tests_count, output)? {
            errors.push(test);
        }
    }
    Ok(errors)
} 