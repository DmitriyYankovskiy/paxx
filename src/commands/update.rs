use std::process::{Child, ExitStatus};

use colored::Colorize;

use crate::{
    config::{self, Config}, 
    hashes::{get_hash_file, Hashes},
    paths::{self, remove_all_in_dir},
    compile::compile,
};

pub fn all(config: &Config, hashes: &mut Hashes) -> Result<(), ()> {
    println!("{}", "* updating ...".bright_yellow());
    let mut childs = Vec::<Option<(String, Child)>>::new();
    childs.push(test_gen(config, hashes)?);
    childs.push(solve(config, hashes)?);
    
    match config.testing_type {
        config::TestingType::CheckingResults => {
            childs.push(res_checker(config, hashes)?);
        },
        config::TestingType::DifferenceResults => {
            childs.push(reference(config, hashes)?);
            childs.push(diff_checker(config, hashes)?);
        }
    };   

    for child in childs {
        if let Some((path, mut child)) = child {
            let res = child.wait().unwrap();
            if res.success() {
                println!("{} succesful compiled with status code: {res}", path.as_str().bold().green());
            } else {
                println!("{} compiled with status code: {res}", path.as_str().bold().bright_red());
            }
        }
    };
    Ok(())
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
        remove_all_in_dir(&paths::solves_results_dir());

        hashes.reference = Some(hash);

        Ok(Some((path, compile(&config.reference_path.clone().unwrap(), config)?)))
    } else {    println!("{}", "<> updating ...".bright_yellow());

        Ok(None)
    }  
}

fn diff_checker(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Child)>, ()>   {    
    let path = config.diff_checker_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.diff_checker {
        remove_all_in_dir(&paths::solves_results_dir());

        hashes.diff_checker = Some(hash);

        Ok(Some((path, compile(&config.diff_checker_path.clone().unwrap(), config)?)))
    } else {
        Ok(None)
    }
} 

fn res_checker(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Child)>, ()>   {
    let path = config.res_checker_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.res_checker {
        remove_all_in_dir(&paths::solves_results_dir());

        hashes.res_checker = Some(hash);

        Ok(Some((path, compile(&config.res_checker_path.clone().unwrap(), config)?)))
    } else {
        Ok(None)
    }
} 
