use std::{fs, io::Write, path::Path};

use crate::{
    config::Config, hashes::Hashes, paths::{self, touch_dir}, readme
};

pub fn all() {
    touch_dir(&paths::dir());
    touch_dir(&paths::build_dir());
    touch_dir(&paths::tests_dir());
    touch_dir(&paths::solves_results_dir());
    touch_dir(&paths::ref_results_dir());

    if !Path::new(paths::config().as_str()).exists() {
        let mut config = Config::default();
        Config::write(&mut config);
    }

    if !Path::new(paths::hashes().as_str()).exists() {
        let mut hashes = Hashes::default();
        Hashes::write(&mut hashes);
    }

    if !Path::new(paths::readme().as_str()).exists() {
        let mut file = fs::File::create(paths::readme()).unwrap();
        file.write(readme::README.as_bytes()).unwrap();
    }
}
