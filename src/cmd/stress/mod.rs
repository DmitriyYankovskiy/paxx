pub mod get;
pub mod build;

use colored::{Color, Colorize};

use std::{cmp::{max, min}, fs, io::{BufReader, Read}, str::FromStr, time::Duration, usize};

use crate::{
    config::Config, hashes::Hashes, out, paths, utils::{run::{self, RunResult}, arg::Flags}, CAP
};

#[derive(Clone, Copy)]
pub enum Mode {
    Check,
    Compare,
    AutoCompare,
}

impl FromStr for Mode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "check" => Ok(Mode::Check),
            "comp" => Ok(Mode::Compare),
            "acomp" => Ok(Mode::AutoCompare),
            _ => {out::error("mode", "incorrect"); Err(())}
        }
    }
}

pub enum StopMode {
    All,
    UpToMistake(usize),
}

pub enum Verdict {
    OK,
    WA,
}

impl FromStr for Verdict {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OK" => Ok(Self::OK),
            "WA" => Ok(Self::WA),
            _ => {
                out::error(format!("velidator's verdict: {s}").as_str(), "invalid");
                Err(())
            },
        }
    }
}

impl Verdict {
    pub fn is_ok(&self) -> bool {
        if let Self::OK = self {
            true
        } else {
            false
        }
    } 
}

pub fn all<'a>(tests_count: usize, stop_mode: StopMode, mode: Mode, config: &Config, hashes: &mut Hashes, flags: &'a Flags) -> Result<(), ()> {
    out::status("stress ...");
    println!();
    let mut mistakes = Vec::<usize>::new();
    let mut max_duration = Duration::ZERO;
    'tests: for test_number in 1..=tests_count {
        let res = one(test_number, config, hashes, mode, &flags)?;
        if !res.verdict.is_ok() {
            mistakes.push(test_number);
        } else if let Some(duration) = res.duration {
            max_duration = max(max_duration, duration);
        }

        if let StopMode::UpToMistake(cap) = stop_mode {
            if mistakes.len() >= cap {
                break 'tests;
            }
        }
    };

    println!();

    if !mistakes.is_empty() {
        println!("{}", "errors on test:".red());
        let cnt = min(mistakes.len(), 2);
        for i in 0..cnt {
            print!("{} ", format!("{}", mistakes[i]).bold().bright_red());
        }
        println!("{}", "... ".red());
    }

    println!("maximal time solution execution: {} secs", format!("{:.3}", (max_duration.as_secs_f32() * 1000.0).ceil()/1000.0).bold().bright_green());

    Ok(())
}

struct TestResult {
    verdict: Verdict, 
    duration: Option<Duration>,
}

fn one(test_number: usize, config: &Config, hashes: &mut Hashes, mode: Mode, flags: &Flags) -> Result<TestResult, ()> {
    let mut duration = None;
    if test_number > hashes.tests_count {
        generator(test_number, config)?;
        hashes.tests_count += 1;
    }
    if test_number > hashes.solution_results_count {
        duration = Some(solution(test_number, config)?);
        hashes.solution_results_count += 1;
    }

    Ok(TestResult{
        verdict: match mode {
            Mode::Check => checker(test_number, config, flags, &format!("{}/{}.dat", paths::solution_results_dir(), test_number))?,
            Mode::Compare => {
                if test_number > hashes.reference_results_count {
                    reference(test_number, config)?;
                    hashes.reference_results_count += 1;
                }
                comparator(test_number, config, flags, &std_input_solution(test_number), &std_input_reference(test_number))?
            },
            Mode::AutoCompare => {
                if test_number > hashes.reference_results_count {
                    reference(test_number, config)?;
                    hashes.reference_results_count += 1;
                }
                auto_comparator(test_number, flags, &std_input_solution(test_number), &std_input_reference(test_number))?
            },
        }, 
        duration
    })
}

fn generator(test_number: usize, config: &Config) -> Result<(), ()> {
    let path = config.get_generator_path()?;
    if let Err(_) = run::run(&path, None, Some(&format!("{}/{}.dat", paths::tests_dir(), test_number)), vec![&test_number.to_string()]) {
        return Err(());
    }

    Ok(())
}

fn solution(test_number: usize, config: &Config) -> Result<Duration, ()> {
    let path = config.get_solution_path()?;
    let result = if let Ok(res) = run::run(&path, Some(&format!("{}/{}.dat", paths::tests_dir(), test_number)), Some(&format!("{}/{}.dat", paths::solution_results_dir(), test_number)), vec![]) {
        res
    } else {
        return Err(());
    };
    Ok(result.duration)
}

fn get_verdict(test: usize, mut output: String, flags: &Flags) -> Result<Verdict, ()> {
    output.push(' ');
    let (verdict, comment) = match output.split_once(" ") {
        Some(vc) => vc,
        None => {
            out::error("result checker output", "incorrect:");
            println!("{}", output);
            return Err(());
        }
    };

    let comment = comment.trim();

    let space_cnt = CAP.to_string().len() - test.to_string().len();
    let test_string = format!("{}{}", " ".repeat(space_cnt), test.to_string());
    let verdict = Verdict::from_str(verdict.trim())?;
    match verdict {
        Verdict::OK => {
            if !flags.contains("quite") {
                println!("{}", format!("  OK:{test_string} "));
            }
        }
        Verdict::WA => {
            println!("{} {}", format!("  WA:{test_string} ").on_color(Color::TrueColor { r: 255, g: 0, b: 0 }).bold().color(Color::TrueColor { r: 0, g: 0, b: 0 }), comment);
        }
    };
    Ok(verdict)

}

fn reference(test_number: usize, config: &Config) -> Result<(), ()> {
    let path = config.get_reference_path()?;
    if let Err(_) = run::run(&path, Some(&format!("{}/{}.dat", paths::tests_dir(), test_number)), Some(&format!("{}/{}.dat", paths::ref_results_dir(), test_number)), vec![]) {
        return Err(());
    }
    Ok(())
}

pub fn std_input_solution(test_number: usize) -> String {
    format!("{}/{}.dat", paths::solution_results_dir(), test_number)
}

pub fn std_input_reference(test_number: usize) -> String {
    format!("{}/{}.dat", paths::ref_results_dir(), test_number)
}


fn checker(test_number: usize, config: &Config, flags: &Flags, input_solution: &String) -> Result<Verdict, ()> {
    let path = config.get_checker_path()?;
    let result = if let Ok(res) = run::run(&path, Some(input_solution), None, vec![]) {
        res
    } else {
        return Err(());
    };
    
    let RunResult{stdout, ..} = result;
    
    Ok(get_verdict(test_number, stdout.unwrap(), flags)?)
}

fn comparator(test_number: usize, config: &Config, flags: &Flags, input_solution: &String, input_reference: &String) -> Result<Verdict, ()> {
    let path = config.get_comparator_path()?;
    let result = if let Ok(res) = run::run(&path, None, None, vec![&input_solution, &input_reference]) {
        res
    } else {
        return Err(());
    };
    let RunResult{stdout, ..} = result;
    Ok(get_verdict(test_number, stdout.unwrap(), flags)?)
} 

fn auto_comparator(test_number: usize, flags: &Flags, input_solution: &String, input_reference: &String) -> Result<Verdict, ()> {
    let mut reader_solution = BufReader::new(fs::File::open(&input_solution).unwrap());
    let mut reader_reference = BufReader::new(fs::File::open(&input_reference).unwrap());

    let mut output = String::from("OK");

    loop {
        let mut buf_solution = [0; 1024];
        let mut buf_reference = [0; 1024];

        let len_solution = reader_solution.read(&mut buf_solution).unwrap();
        let len_reference = reader_reference.read(&mut buf_reference).unwrap();

        if buf_solution != buf_reference {
            output = String::from("WA");
        }

        if len_reference == 0 || len_solution == 0 {
            break;
        }
    }

    
    Ok(get_verdict(test_number, output, flags)?)
} 