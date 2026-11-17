use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

use crate::error_handling::*;

/// Make sure a path is a file and if it isn't use the fallback
fn read_or(path: PathBuf, fallback: String) -> String {
    if path.is_file() {
        match fs::read(path) {
            Ok(path_content) => {
                String::from_utf8_lossy_owned(path_content)
            }
            Err(_) => {
                throw_error("file unaccessible", 4); unreachable!()
            }
        }
    } else {
        warning("preset not found. using fallback instead");
        fallback
    }
}

/// If the name is found: use that as the configuration
/// If the name isn't found: make a warning and use the default configuration as a fallback
pub fn preset(config_dir: PathBuf, config_name: String) -> String {
    let mut config_file = config_dir;
    config_file.push(config_name);

    read_or(config_file, fallback())
}

/// Get information from the .lns configuration file
pub fn lns_file() -> String {
    match current_dir() {
        Ok(mut lns) => {
            lns.push(".lns");
            read_or(lns, fallback())
        },
        Err(_) => {
            throw_error("reading failed", 3); unreachable!()
        }
    }
}

/// The fallback in case no configuration file was found
pub fn fallback() -> String {
    "++src".to_string()
}
