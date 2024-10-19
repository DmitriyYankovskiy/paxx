use crate::{config::Config, hashes::Hashes, logic::build::Builder, utils::{arg::Flags, build}};
pub fn all(config: &Config, hashes: &mut Hashes, flags: &Flags) -> Result<(), ()> {
    let mut builder = Builder::new();
    builder
        .solution(config, hashes, if flags.contains("hdbg") {
            build::Mode::Hdbg
        } else {
            build::Mode::Dbg
        })?;
    builder.build()
}