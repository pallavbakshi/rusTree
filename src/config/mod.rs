// src/config/mod.rs
pub mod file;
pub mod filtering;
pub mod html;
pub mod input_source;
pub mod listing;
pub mod llm;
pub mod metadata;
pub mod misc;
pub mod output_format;
pub mod partial;

// Re-export key types for convenience
pub use file::load_merged as load_merged_config;
pub use partial::{MergeInto, PartialConfig};
pub mod sorting;
pub mod tree_options;

// Re-export the main config struct and key types
pub use output_format::OutputFormat;
pub use tree_options::RustreeLibConfig;

// Re-export specific enums for convenience in other modules
pub use filtering::FilteringOptions;
pub use html::HtmlOptions;
pub use input_source::InputSourceOptions;
pub use listing::ListingOptions;
pub use llm::{LlmConfigError, LlmOptions, LlmProvider};
pub use metadata::{ApplyFnError, BuiltInFunction, MetadataOptions}; // Re-export BuiltInFunction, ApplyFnError
pub use misc::MiscOptions;
pub use sorting::{SortKey, SortingOptions}; // Re-export SortKey directly as it's a common enum
