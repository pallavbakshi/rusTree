//! Option structs and helper enums used by the `rustree` **core** layer.
//!
//! These types are required by the low-level algorithmic code that lives under
//! `src/core/`.  They are intentionally kept free of any direct dependency on
//! the higher-level *configuration* / *CLI* layers so that the core module can
//! be compiled and published as a stand-alone crate in the future.

// Re-export everything for ergonomic access (e.g. `crate::core::options::ListingOptions`).

pub mod contexts;
pub mod filtering;
pub mod html;
pub mod input_source;
pub mod listing;
pub mod llm;
pub mod metadata;
pub mod misc;
pub mod output_format;
pub mod sorting;
pub mod tree_options;

pub use contexts::{
    FormattingContext, OwnedFormattingContext, OwnedProcessingContext, OwnedSortingContext,
    OwnedWalkingContext, ProcessingContext, ProcessingContextBuilder, SortingContext,
    WalkingContext,
};
pub use filtering::FilteringOptions;
pub use html::HtmlOptions;
pub use input_source::InputSourceOptions;
pub use listing::ListingOptions;
pub use metadata::{
    ApplyFnError, ApplyFunction, BuiltInFunction, ExternalFunction, FunctionOutputKind,
    MetadataOptions,
};
pub use misc::MiscOptions;
pub use output_format::OutputFormat;
pub use sorting::{DirectoryFileOrder, SortKey, SortingOptions};
pub use tree_options::RustreeLibConfig;
