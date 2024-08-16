use crate::config::Config;

pub fn set_solution(path: &String) -> Result<(), ()> {
    let mut config = Config::load()?;
    config.solution_path = path.clone();
    Config::write(&mut config);
    Ok(())
}

pub fn set_test_gen(path: &String) -> Result<(), ()> {
    let mut config = Config::load()?;
    config.test_gen_path = Some(path.clone());
    Config::write(&mut config);
    Ok(())
}