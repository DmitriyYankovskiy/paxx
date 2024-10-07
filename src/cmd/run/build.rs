use crate::{logic::build::Builder, config::Config, hashes::Hashes, utils::build};
pub fn all(config: &Config, hashes: &mut Hashes) -> Result<(), ()> {
    let mut builder = Builder::new();
    builder
        .solution(config, hashes, build::Mode::Dbg)?;
    builder.build()
}