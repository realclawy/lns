mod init;
mod error_handling;
mod action;
mod config;

use crate::config::Config;

fn main() {
    let config_arg = init::arg();
    let mut config = String::new();

    if let Some(config_asked) = config_arg {
        match config_asked.as_str() {
            "help" | "h" | "-h" | "--help" => action::print_help(),
            &_ => {
                let config_dir = init::config_dir();
                config = config::get::preset(config_dir, config_asked);
            }
        }
    } else {
        config = config::get::lns_file();
    }

    // make the config struct
    let mut config = Config::create(config);

    // consume it and create the vector about all files lns has to read
    let files = config.consume();

    // Finally, get the line count
    config.run(files);
}
