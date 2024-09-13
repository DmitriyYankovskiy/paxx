use crate::{buisness::build::Builder, config::Config, hashes::Hashes, utils::compile};
pub fn all(config: &Config, hashes: &mut Hashes) -> Result<(), ()> {
    let mut builder = Builder::new();
    builder
        .solution(config, hashes, compile::Mode::Dbg)?;
    builder.build()
}