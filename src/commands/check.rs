use crate::config::Config;

pub fn all(config: &Config) -> bool {
    config.check()
}