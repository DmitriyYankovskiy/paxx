use crate::{utils::file, Flags};

pub fn file(path: &String, flags: &Flags) {
    file::create_file(path.to_string(), "", flags);
}