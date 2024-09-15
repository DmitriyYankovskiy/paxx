use std::{collections::VecDeque, env};

use crate::out;

pub struct Args {
    pub args: std::collections::VecDeque<String>,
}

impl Args {
    pub fn init() -> Self {
        let mut args = env::args().collect::<VecDeque<String>>();
        args.pop_front();
        Self {
            args,
        }
    }

    pub fn get(&mut self, name: &str) -> Result<String, ()> {
        if let Some(arg) = self.args.pop_front() {
            Ok(arg)
        } else {
            out::error(&format!("arg: {name}"), "not found");
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

