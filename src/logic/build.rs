use std::process::Child;

use crate::{
    config::Config,
    hashes::{get_hash_file, Hashes},
    out,
    utils::build::{self, any}, Language,
};



pub struct Builder {
    childs: Vec<Option<(String, Option<Child>)>>,
}

fn get_args(lang: Language, mode: build::Mode, config: &Config) -> Vec<String> {
    match mode {
        build::Mode::Rls => {
            config.get_compile_rls_args(lang)
        },
        build::Mode::Dbg => {
            config.get_compile_dbg_args(lang)
        },
        build::Mode::Hdbg => {
            let mut args = config.get_compile_dbg_args(lang);
            args.append(&mut config.get_compile_hdbg_add_args(lang));
            args
        },
    }
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
                    out::info(&path, " -> succesful compiled");
                } else {
                    res = Err(());
                    out::error(&path, " -> compiled with error");
                }
            }
        }
        res
    }

    pub fn solution(&mut self, config: &Config, hashes: &mut Hashes, compile_mode: build::Mode) -> Result<&mut Self, ()> {
        let path = config.get_solution_path()?;
        let lang = Language::from_path(&path)?;
        let args = get_args(lang, compile_mode, config);
        let hash = get_hash_file(&path, &args);

        if Some(hash) != hashes.solution {
            hashes.tests_count = 0;
            hashes.solution_results_count = 0;
            hashes.reference_results_count = 0;

            let res = any(&path, &args, lang)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.solution = Some(hash);            
        }
        Ok(self)
    }

    pub fn generator(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_generator_path()?;
        let lang = Language::from_path(&path)?;
        let args = get_args(lang, build::Mode::Rls, config);
        let hash = get_hash_file(&path, &args);

        if Some(hash) != hashes.generator {
            hashes.tests_count = 0;
            hashes.solution_results_count = 0;
            hashes.reference_results_count = 0;

            let res = any(&path, &args, lang)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.generator = Some(hash);
        }
        Ok(self)
    }

    pub fn reference(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_reference_path()?;
        let lang = Language::from_path(&path)?;
        let args = get_args(lang, build::Mode::Rls, config);
        let hash = get_hash_file(&path, &args);

        if Some(hash) != hashes.reference {
            hashes.reference_results_count = 0;

            let res = any(&path, &args, lang)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.reference = Some(hash);
        }
        Ok(self)
    }

    pub fn comparator(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_comparator_path()?;
        let lang = Language::from_path(&path)?;
        let args = get_args(lang, build::Mode::Rls, config);
        let hash = get_hash_file(&path, &args);

        if Some(hash) != hashes.comparator {
            let res = any(&path, &args, lang)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.comparator = Some(hash);
        }
        Ok(self)
    }

    pub fn checker(&mut self, config: &Config, hashes: &mut Hashes) -> Result<&mut Self, ()> {
        let path = config.get_checker_path()?;
        let lang = Language::from_path(&path)?;
        let args = get_args(lang, build::Mode::Rls, config);
        let hash = get_hash_file(&path, &args);

        if Some(hash) != hashes.checker {
            let res = any(&path, &args, lang)?;
            let res = Some((path, res));
            self.childs.push(res);
            hashes.checker = Some(hash);
        }
        Ok(self)
    }
}