use std::env;

use crate::log;

pub struct Args {
    pub args: std::collections::VecDeque<String>,
}

impl Args {
    pub fn init() -> Self {
        Self {
            args: env::args().collect(),
        }
    }

    pub fn get(&mut self, name: &str) -> Result<String, ()> {
        if let Some(arg) = self.args.pop_front() {
            Ok(arg)
        } else {
            log::error(&format!("arg: {name}"), "not found");
            Err(())
        }
    }

    pub fn try_get(&mut self) -> Option<String> {
        if let Some(arg) = self.args.pop_front() {
            Some(arg)
        } else {
            None
        }
    }
}
