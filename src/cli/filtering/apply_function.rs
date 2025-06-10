use clap::Args;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Command-line arguments for filtering apply-function operations.
#[derive(Args, Debug)]
pub struct ApplyFunctionFilterArgs {
    /// Only apply function to files/dirs matching these patterns.
    /// Multiple patterns can be specified.
    #[arg(long = "apply-include", value_name = "PATTERN")]
    pub apply_include: Option<Vec<String>>,

    /// Skip applying function to files/dirs matching these patterns.
    /// Multiple patterns can be specified.
    #[arg(long = "apply-exclude", value_name = "PATTERN")]
    pub apply_exclude: Option<Vec<String>>,

    /// Read include patterns for apply-function from a file.
    /// Each line in the file should contain one pattern.
    #[arg(long = "apply-include-from", value_name = "FILE_PATH")]
    pub apply_include_from: Option<PathBuf>,

    /// Read exclude patterns for apply-function from a file.
    /// Each line in the file should contain one pattern.
    #[arg(long = "apply-exclude-from", value_name = "FILE_PATH")]
    pub apply_exclude_from: Option<PathBuf>,
}

impl ApplyFunctionFilterArgs {
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
    pub fn get_all_include_patterns(&self) -> Result<Option<Vec<String>>, io::Error> {
        let mut all_patterns = Vec::new();

        // Add patterns from command line
        if let Some(ref patterns) = self.apply_include {
            all_patterns.extend(patterns.clone());
        }

        // Add patterns from file
        if let Some(ref file_path) = self.apply_include_from {
            let file_patterns = Self::read_patterns_from_file(file_path)?;
            all_patterns.extend(file_patterns);
        }

        if all_patterns.is_empty() {
            Ok(None)
        } else {
            Ok(Some(all_patterns))
        }
    }

    /// Gets all exclude patterns, combining those from command line and file sources.
    pub fn get_all_exclude_patterns(&self) -> Result<Option<Vec<String>>, io::Error> {
        let mut all_patterns = Vec::new();

        // Add patterns from command line
        if let Some(ref patterns) = self.apply_exclude {
            all_patterns.extend(patterns.clone());
        }

        // Add patterns from file
        if let Some(ref file_path) = self.apply_exclude_from {
            let file_patterns = Self::read_patterns_from_file(file_path)?;
            all_patterns.extend(file_patterns);
        }

        if all_patterns.is_empty() {
            Ok(None)
        } else {
            Ok(Some(all_patterns))
        }
    }
}
