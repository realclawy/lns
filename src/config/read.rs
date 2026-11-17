use std::{env::current_dir};

use crate::error_handling::throw_error;
use super::Config;

impl Config {
    /// Gets a default Config struct, which would be:
    /// ```rust
    /// Config {
    ///     ignore_comments_empty: 2, // show both versions
    ///     verbose: true, // show all file lengths
    ///     directories: Vec::new(), // no directories to show
    ///     files: Vec::new(), // no files to show
    ///     ignore_directories: Vec::new(), // no directories to ignore
    ///     ignore_files: Vec::new() // no files to ignore
    /// ```
    /// }
    pub fn default() -> Self { // All the default settings
        Self {
            ignore_comments_empty: 2,
            verbose: true,
            directories: Vec::new(),
            files: Vec::new(),
            ignore_directories: Vec::new(),
            ignore_files: Vec::new()
        }
    }

    pub fn create(config_content: String) -> Self {
        let mut config = Self::default();
        let current_dir = current_dir().unwrap_or_else(|_| {throw_error("uhh", 4); unreachable!()});

        for line in config_content.lines()  {
            // Dirs and files to be looked for
            if let Some(line) = line.strip_prefix("++") {
                let mut dir = current_dir.clone();
                dir.push(line);
                if dir.is_dir() {
                    config.directories.push(dir);
                }
            }
            if let Some(line) = line.strip_prefix("+") {
                let mut file = current_dir.clone();
                file.push(line);
                if file.is_file() {
                    config.files.push(file);
                }
            }

            // Dirs and files to be explicitly ignored
            if let Some(line) = line.strip_prefix("--") {
                let mut undir = current_dir.clone();
                undir.push(line);
                if undir.is_dir() {
                    config.ignore_directories.push(undir);
                }
            }
            if let Some(line) = line.strip_prefix("-") {
                let mut unfile = current_dir.clone();
                unfile.push(line);
                if unfile.is_file() {
                    config.ignore_files.push(unfile);
                }
            }

            // Options
            if let Some(line) = line.strip_prefix("ignore_spaces_comments=") {
                match line {
                    "false" => config.ignore_comments_empty = 0,
                    "true" => config.ignore_comments_empty = 1,
                    &_ => {}
                }
            }
            if let Some(line) = line.strip_prefix("verbose=") && line == "false" {
                config.verbose = false; // We don't have to check for true as it's the default
            }
        }

        config
    }
}
