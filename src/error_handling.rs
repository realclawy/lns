const ERROR: &str = "\x1b[31mError:\x1b[0m"; // "Error:" with red letters
const WARN: &str = "\x1b[33mWarning:\x1b[0m"; // "Warning:" with orange letters

pub fn throw_error(error: &str, code: i32) {
    eprintln!("{ERROR} {error}");
    std::process::exit(code);
}

pub fn warning(warn: &str) {
    eprintln!("{WARN} {warn}")
}

