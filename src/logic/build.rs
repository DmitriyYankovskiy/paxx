use std::process::Child;

use crate::{
    config::Config,
    hashes::{get_hash_file, Hashes},
    out,
    utils::build::{self, any},
};



pub struct Builder {
    childs: Vec<Option<(String, Option<Child>)>>,
}

impl Builder {
    pub fn new() -> Self {
        Self{childs: Vec::<Option<(String, Option<Child>)>>::new()}
    }

    pub fn build(self) -> Result<(), ()> {
        out::status("build ...");
        let mut res = Ok(());

        for child in self.childs {
            if let Some((path, child)) = child {
                let mut status = true;
                if let Some(mut child) = child {
                    status = child.wait().unwrap().success();
                }
                if status {
                    out::info(&path, " - succesful compiled");
                } else {
                    res = Err(());
                    out::error(&path, " - compiled with error");
                }
            }
        }
        res
    }

    pub fn solution(&mut self, config: &Config, hashes: &mut Hashes, compile_mode: build::Mode) -> Result<&mut Self, ()> {
        let path = config.get_solution_path()?;
        let hash = get_hash_file(&path);

        if Some(hash) != hashes.solution {
            hashes.tests_count = 0;
            hashes.solution_results_count = 0;
            hashes.reference_results_count = 0;

            let res = any(&path, config, compile_mode)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.solution = Some(hash);            
        }
        Ok(self)
    }

    pub fn generator(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_generator_path()?;
        let hash = get_hash_file(&path);

        if Some(hash) != hashes.generator {
            hashes.tests_count = 0;
            hashes.solution_results_count = 0;
            hashes.reference_results_count = 0;

            let res = any(&path, config, build::Mode::Std)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.generator = Some(hash);
        }
        Ok(self)
    }

    pub fn reference(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_reference_path()?;
        let hash = get_hash_file(&path);

        if Some(hash) != hashes.reference {
            hashes.reference_results_count = 0;

            let res = any(&path, config, build::Mode::Std)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.reference = Some(hash);
        }
        Ok(self)
    }

    pub fn comparator(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_comparator_path()?;
        let hash = get_hash_file(&path);

        if Some(hash) != hashes.comparator {
            let res = any(&path, config, build::Mode::Std)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.comparator = Some(hash);
        }
        Ok(self)
    }

    pub fn checker(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_checker_path()?;
        let hash = get_hash_file(&path);

        if Some(hash) != hashes.checker {
            let res = any(&path, config, build::Mode::Std)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.checker = Some(hash);
        }
        Ok(self)
    }
}
