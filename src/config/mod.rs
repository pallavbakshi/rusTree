// src/config/mod.rs
pub mod tree_options;
pub mod sorting;
pub mod output;
pub mod fileinfo;

// Re-export the main config struct and key types
pub use tree_options::RustreeLibConfig;
pub use sorting::SortKey;
pub use output::OutputFormat;
pub use fileinfo::{BuiltInFunction, ApplyFnError};