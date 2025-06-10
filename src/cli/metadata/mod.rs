pub mod date;
pub mod size;
pub mod stats;

/// Defines built-in functions that can be applied to file and directory contents via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliBuiltInFunction {
    // File functions
    /// Counts occurrences of the '+' character.
    CountPluses,
    /// Displays the content of each file.
    Cat,

    // Directory functions
    /// Counts the number of files (non-directories) in the directory.
    CountFiles,
    /// Counts the number of subdirectories in the directory.
    CountDirs,
    /// Calculates the total size of all contents recursively.
    SizeTotal,
    /// Shows combined statistics for the directory (files, dirs, total size).
    DirStats,
}
