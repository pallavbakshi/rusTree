// src/cli/mod.rs
mod args;
mod mapping; // CLI to library configuration mapping functions
pub mod pruning; // New module for pruning arguments

// Re-export the main CLI args struct.
pub use args::CliArgs;
pub use mapping::{map_cli_to_lib_config, map_cli_to_lib_output_format};

// Declare the new sub-modules
pub mod filtering;
pub mod listing;
pub mod metadata;
pub mod misc;
pub mod output;
pub mod sorting;
