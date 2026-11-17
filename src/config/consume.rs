use std::path::PathBuf;
use std::fs::{read_dir};

use crate::config::Config;

impl Config {
    /// Uses up all vectors in self and makes the final list of all files to be searched up
    pub fn consume(&mut self) -> Vec<PathBuf> {

        let mut paths: Vec<PathBuf> = Vec::new();
        let dirs_len = self.directories.len();

        // Include all explicitly stated files
        paths.append(&mut self.files);

        // Include all directories
        for _ in 0..dirs_len {
            let directory = self.directories.swap_remove(0);
            let mut directory = self.read_dir(directory);
            paths.append(&mut directory);
        }

        // Clear all unneeded memory
        self.ignore_directories.clear();
        self.ignore_files.clear();

        // Return all paths
        paths
    }

    /// Reads the directory and gets out all files that should be read from it
    fn read_dir(&self, dir: PathBuf) -> Vec<PathBuf> {
        // The vector that will get returned
        let mut paths: Vec<PathBuf> = Vec::new();

        // Convert dir to all of its elements
        let dir = read_dir(dir).unwrap();
        let dir = dir.map(|item| {
            item.unwrap().path() // Who needs error handling anyway?
        });

        // Check for all files and directories to be added.
        // If any is in the list of the "get ignored", they don't get included
        for path in dir {
            if path.is_file() && !self.ignore_files.contains(&path) {
                paths.push(path)
            } else if path.is_dir() && !self.ignore_directories.contains(&path) {
                paths.append(
                    &mut self.read_dir(path)
                );
            }
        }

        // Returns all paths extracted from the directory
        paths
    }
}
