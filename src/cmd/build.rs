use std::process::Child;

use crate::{
    config::{self, Config}, hashes::{get_hash_file, Hashes}, log, paths::{self, remove_all_in_dir}, utils::compile::compile
};

pub fn all(config: &Config, hashes: &mut Hashes) -> Result<(), ()> {
    log::status("build ...");
    let mut childs = Vec::<Option<(String, Option<Child>)>>::new();
    childs.push(solution(config, hashes)?);
    
    match config.testing_mode {
        config::TestingMode::CheckingResults => {
            childs.push(test_gen(config, hashes)?);
            childs.push(res_checker(config, hashes)?);
        },
        config::TestingMode::ComparisonResults => {
            childs.push(test_gen(config, hashes)?);
            childs.push(reference(config, hashes)?);
            childs.push(comparator(config, hashes)?);
        }
        config::TestingMode::AutoComparisonResults => {
            childs.push(test_gen(config, hashes)?);
            childs.push(reference(config, hashes)?);
        }
        config::TestingMode::Manual => {
        }
    };   

    let mut res = Ok(());

    for child in childs {
        if let Some((path, child)) = child {
            let mut status = true;
            if let Some(mut child) = child {
                status = child.wait().unwrap().success();
            }
            if status {
                log::info(&path, " - succesful compiled");
            } else {
                res = Err(());
                log::error(&path, " - compiled with error");
            }
        }
    };
    res
}

fn test_gen(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Option<Child>)>, ()> {
    let path = config.test_gen_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if hash != hashes.test_gen {
        remove_all_in_dir(&paths::tests_dir());

        remove_all_in_dir(&paths::solution_results_dir());
        remove_all_in_dir(&paths::ref_results_dir());

        let res = compile(&path, config)?;
        let res = Ok(Some((path, res)));
        hashes.test_gen = hash;
        res
    } else {
        Ok(None)
    }    
}

fn solution(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Option<Child>)>, ()>   {    
    let path = config.solution_path.clone();
    let hash = get_hash_file(&path);

    if hash != hashes.solution {
        remove_all_in_dir(&paths::solution_results_dir());

        let res = compile(&path, config)?;
        let res = Ok(Some((path, res)));
        hashes.solution = hash;
        res
    } else {
        Ok(None)
    }    
}

fn reference(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Option<Child>)>, ()>   {    
    let path = config.reference_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.reference {
        remove_all_in_dir(&paths::ref_results_dir());

        let res = compile(&path, config)?;
        let res = Ok(Some((path, res)));
        hashes.reference = Some(hash);
        res
    } else {
        Ok(None)
    }  
}

fn comparator(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Option<Child>)>, ()>   {    
    let path = config.comparator_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.comparator {
        let res = compile(&path, config)?;
        let res = Ok(Some((path, res)));
        hashes.comparator = Some(hash);
        res
    } else {
        Ok(None)
    }
} 

fn res_checker(config: &Config, hashes: &mut Hashes) -> Result<Option<(String, Option<Child>)>, ()>   {
    let path = config.res_checker_path.clone().unwrap();
    let hash = get_hash_file(&path);

    if Some(hash) != hashes.res_checker {
        let res = compile(&path, config)?;
        let res = Ok(Some((path, res)));
        hashes.res_checker = Some(hash);
        res
    } else {
        Ok(None)
    }
} 
