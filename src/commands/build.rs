use std::process::Child;

use colored::Colorize;

use crate::{
    config::{self, Config}, 
    hashes::{get_hash_file, Hashes},
    paths::{self, remove_all_in_dir},
    compile::compile,
};

pub fn all(config: &Config, hashes: &mut Hashes) -> Result<(), ()> {
    println!("{}", "building ...".bright_yellow());
    let mut childs = Vec::<Option<(String, Child)>>::new();
    childs.push(test_gen(config, hashes)?);
    childs.push(solve(config, hashes)?);
    
    match config.testing_type {
        config::TestingType::CheckingResults => {
            childs.push(res_checker(config, hashes)?);
        },
        config::TestingType::ComparisonResults => {
            childs.push(reference(config, hashes)?);
            childs.push(comparator(config, hashes)?);
        }
        config::TestingType::AutoComparisonResults => {
            childs.push(reference(config, hashes)?);
        }
    };   

    let mut res = Ok(());

    for child in childs {
        if let Some((path, mut child)) = child {
            let status = child.wait().unwrap();
            if status.success() {
                println!(" - {} succesful compiled", path.as_str().bold().cyan());
            } else {
                res = Err(());
                println!(" - {} compiled with error", path.as_str().bold().bright_red());
            }
        }
    };
    res
}

fn test_gen(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Child)>, ()> {
    let path = config.test_gen_path.clone();
    let hash = get_hash_file(&path);

    if hash != hashes.test_gen {
        remove_all_in_dir(&paths::tests_dir());

        remove_all_in_dir(&paths::solves_results_dir());
        remove_all_in_dir(&paths::ref_results_dir());

        hashes.test_gen = hash;

        Ok(Some((path, compile(&config.test_gen_path, config)?)))
    } else {
        Ok(None)
    }    
}

fn solve(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Child)>, ()>   {    
    let path = config.solve_path.clone();
    let hash = get_hash_file(&path);

    if hash != hashes.solve {
        remove_all_in_dir(&paths::solves_results_dir());

        hashes.solve = hash;

        Ok(Some((path, compile(&config.solve_path, config)?)))
    } else {
        Ok(None)
    }    
}

fn reference(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Child)>, ()>   {    
    let path = config.reference_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.reference {
        remove_all_in_dir(&paths::ref_results_dir());

        hashes.reference = Some(hash);

        Ok(Some((path, compile(&config.reference_path.clone().unwrap(), config)?)))
    } else {
        Ok(None)
    }  
}

fn comparator(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Child)>, ()>   {    
    let path = config.comparator_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.comparator {
        hashes.comparator = Some(hash);
        Ok(Some((path, compile(&config.comparator_path.clone().unwrap(), config)?)))
    } else {
        Ok(None)
    }
} 

fn res_checker(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Child)>, ()>   {
    let path = config.res_checker_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.res_checker {
        hashes.res_checker = Some(hash);
        Ok(Some((path, compile(&config.res_checker_path.clone().unwrap(), config)?)))
    } else {
        Ok(None)
    }
} 
