use colored::Colorize;

pub fn error(acc: &str, comment: &str) {
    println!("{} {}", acc.bold().bright_red(), comment.red());
}

pub fn ok(acc: &str, comment: &str) {
    println!("{} {}", acc.bold().bright_green(), comment.green());
}

pub fn info(acc: &str, comment: &str) {
    println!("{} {}", acc.bold().white(), comment.white());
}
pub fn status(comment: &str) {
    println!("{}", comment.bright_yellow());
}