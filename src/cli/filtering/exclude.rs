// src/cli/filtering/exclude.rs
use clap::Args;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ExcludeArgs {
    /// Do not list those files/directories that match the wild-card pattern. (Original tree: -I)
    /// Can be specified multiple times. Uses glob pattern syntax (see -P).
    #[arg(short = 'I', long = "filter-exclude", action = clap::ArgAction::Append)]
    pub ignore_patterns: Option<Vec<String>>,

    /// Read exclude patterns from a gitignore-style file. Each line in the file
    /// should contain one pattern. Can be specified multiple times.
    #[arg(long = "filter-exclude-from", value_name = "FILE", action = clap::ArgAction::Append)]
    pub ignore_patterns_from: Option<Vec<PathBuf>>,
}

impl ExcludeArgs {
    /// Reads patterns from the given file path, with each line representing a pattern.
    /// Empty lines and lines starting with '#' are ignored.
    fn read_patterns_from_file(file_path: &PathBuf) -> Result<Vec<String>, io::Error> {
        let content = fs::read_to_string(file_path)?;
        let patterns: Vec<String> = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| line.to_string())
            .collect();
        Ok(patterns)
    }

    /// Gets all exclude patterns, combining those from command line and file sources.
    pub fn get_all_ignore_patterns(&self) -> Result<Option<Vec<String>>, io::Error> {
        let mut all_patterns = Vec::new();

        // Add patterns from command line
        if let Some(ref patterns) = self.ignore_patterns {
            all_patterns.extend(patterns.clone());
        }

        // Add patterns from files
        if let Some(ref file_paths) = self.ignore_patterns_from {
            for file_path in file_paths {
                let file_patterns = Self::read_patterns_from_file(file_path)?;
                all_patterns.extend(file_patterns);
            }
        }

        if all_patterns.is_empty() {
            Ok(None)
        } else {
            Ok(Some(all_patterns))
        }
    }
}
