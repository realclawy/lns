use std::path::PathBuf;

/// contains information about the configuration, including:
/// whether comments and empty lines are ignored
/// whether output should be verbose (mention all files or just the total)
/// what directories to include
/// what files to include
/// what directories within specified directories to ignore
/// what files within sppecified directories to ignore
pub struct Config {
    pub ignore_comments_empty: u8, // 0 - false, 1 - true, 2 - both
    pub verbose: bool,
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
    pub ignore_directories: Vec<PathBuf>,
    pub ignore_files: Vec<PathBuf>
}

pub mod read;
pub mod get;
pub mod consume;
pub mod run;
