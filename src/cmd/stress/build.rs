use super::Mode;
use crate::{logic::build::Builder, config::Config, hashes::Hashes, utils::compile};
pub fn all(config: &Config, hashes: &mut Hashes, mode: Mode) -> Result<(), ()> {
    let mut builder = Builder::new();
    builder
        .generator(config, hashes)?
        .solution(config, hashes, compile::Mode::Std)?;
    match mode {
        Mode::Check => {
            builder
                .checker(config, hashes)?;
        },
        Mode::Compare => {
            builder
                .reference(config, hashes)?
                .comparator(config, hashes)?;
        },
        Mode::AutoCompare => {
            builder
                .reference(config, hashes)?;
        },
    };
    builder.build()
}