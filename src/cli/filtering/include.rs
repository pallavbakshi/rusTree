// src/cli/filtering/include.rs
use clap::Args;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct IncludeArgs {
    /// List only those files that match the wild-card pattern. (Original tree: -P)
    /// Can be specified multiple times.
    /// See `glob` crate documentation for pattern syntax.
    /// `|` can be used within a pattern for alternation, e.g., "*.txt|*.md".
    /// A `/` at the end of a pattern matches directories only, e.g., "docs/".
    #[arg(short = 'P', long = "filter-include", action = clap::ArgAction::Append)]
    pub match_patterns: Option<Vec<String>>,

    /// Read include patterns from a gitignore-style file. Each line in the file
    /// should contain one pattern. Can be specified multiple times.
    #[arg(long = "filter-include-from", value_name = "FILE", action = clap::ArgAction::Append)]
    pub match_patterns_from: Option<Vec<PathBuf>>,
}

impl IncludeArgs {
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

    /// Gets all include patterns, combining those from command line and file sources.
    pub fn get_all_match_patterns(&self) -> Result<Option<Vec<String>>, io::Error> {
        let mut all_patterns = Vec::new();

        // Add patterns from command line
        if let Some(ref patterns) = self.match_patterns {
            all_patterns.extend(patterns.clone());
        }

        // Add patterns from files
        if let Some(ref file_paths) = self.match_patterns_from {
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
