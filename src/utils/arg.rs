use std::{collections::{HashSet, VecDeque}, env};

use crate::out;

pub type Flags = HashSet<String>;
pub struct Args {
    pub args: std::collections::VecDeque<String>,
}

impl Args {
    pub fn init() -> (Self, Flags) {
        let mut args = env::args().collect::<VecDeque<String>>();
        let mut flags = Flags::new();
        args = args.into_iter().filter(|arg| {
            if arg.starts_with('-') {
                flags.insert(arg[1..].to_string());
                false                
            } else {
                true
            }
        }).collect();
        args.pop_front();
        (Self {
            args,
        }, flags)
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
