use std::env;
use crate::error_handling::*;
use std::path::PathBuf;
use std::fs;

pub fn config_dir() -> PathBuf {
    let mut config_dir: PathBuf;

    match dirs::config_dir() {
        Some(config_path) => config_dir = config_path,
        None => {
            throw_error("config directory is not reachable. This is probably either the library's fault or your computer's.", 1);
            unreachable!();
        }
    }

    config_dir.push("lns");

    if config_dir.exists() {
        config_dir
    } else {
        match fs::create_dir(&config_dir) {
            Ok(_) => {},
            Err(err) => {
                    throw_error(
                        format!("config directory is not writtable to.\nThis is what the error says: {err}").as_str(), 2
                    ); unreachable!()
                }
        }
        config_dir
    }
}

pub fn arg() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let arg_num = args.len() - 1;

    if arg_num == 0 {
        None
    } else if arg_num > 1 {
        throw_error("Too many arguments...", 1); unreachable!()
    } else {
        let mut args = args.into_iter();
        args.next(); // Consume the path thing

        Some(args.next().unwrap())
    }
}
