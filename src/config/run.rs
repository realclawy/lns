use std::{fs, path::PathBuf};

use crate::config::Config;

impl Config {
    pub fn run(&self, files: Vec<PathBuf>) {
        let mut total_lines = 0;
        let mut total_loc = 0;

        for file in files {
            // Save the file's name
            let file_name = file.file_name().unwrap().to_str().unwrap_or("?").to_string();
            // Read the file
            let file = String::from_utf8_lossy_owned(
                fs::read(file).unwrap_or(Vec::new())
            );

            // Read the lines from the file and collect data
            let mut lines = 0;
            let mut loc = 0;
            for line in file.lines() {
                if !line.is_empty() && !line.trim().starts_with("//") {
                    loc += 1;
                }
                lines += 1;
            }
            total_lines += lines;
            total_loc += loc;

            // If the output is verbose, print this file's line count
            if self.verbose {
                display_line_count(self.ignore_comments_empty, file_name, lines, loc);
            }
        }

        // Make a space between the other files and the total
        if self.verbose {
            println!();
        }

        display_line_count(self.ignore_comments_empty, "total: ".to_string(), total_lines, total_loc);
    }
}

/// Displays the line count with name `name`
fn display_line_count(ignore_comments_spaces: u8, name: String, lines: u32, loc: u32) {
    match ignore_comments_spaces {
        0 => println!("{name}: {lines}"),
        1 => println!("{name}: {loc}"),
        2 => println!("{name}: {lines} ({loc} loc)"),
        _ => {}
    }
}
