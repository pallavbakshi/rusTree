// src/cli/mod.rs
mod args;
mod mapping; // CLI to library configuration mapping functions

// Re-export the main CLI args struct.
pub use args::CliArgs;
pub use mapping::{
    CliMappingError, map_cli_to_diff_options, map_cli_to_lib_config, map_cli_to_lib_output_format,
};

// Declare the new sub-modules
pub mod diff;
pub mod filtering;
pub mod input;
pub mod listing;
pub mod llm;
pub mod metadata;
pub mod misc;
pub mod output;
pub mod sorting;
