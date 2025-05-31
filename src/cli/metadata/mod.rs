pub mod date;
pub mod size;
pub mod stats;

/// Defines built-in functions that can be applied to file contents via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliBuiltInFunction {
    /// Counts occurrences of the '+' character.
    CountPluses,
    // Add other function names here
}
