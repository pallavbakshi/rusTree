//! Partial configuration structures used for deserialising `.toml` files.
//!
//! Each `Partial*` struct mirrors its counterpart in the runtime configuration
//! but wraps every field in `Option<T>`.  This allows us to know whether a
//! value was explicitly provided by the user (Some) or was absent (None),
//! enabling precise merging semantics where CLI flags always win.

use super::RustreeLibConfig;
use super::filtering::FilteringOptions;
use super::html::HtmlOptions;
use super::input_source::InputSourceOptions;
use super::listing::ListingOptions;
use super::metadata::MetadataOptions;
use super::misc::MiscOptions;
use super::sorting::SortingOptions;

/// Trait implemented by partial structs so they can be merged into their full
/// counterparts.
pub trait MergeInto<T> {
    fn merge_into(self, dest: &mut T);
}

/* ------------------------------------------------------------------------- */
/*  Group-level partial structs                                              */
/* ------------------------------------------------------------------------- */

#[derive(Debug, Clone, Default)]
pub struct PartialListingOptions {
    pub max_depth: Option<Option<usize>>, // double Option: Some(Some(x)) = set, Some(None)=explicit null, None=not present
    pub show_hidden: Option<bool>,
    pub list_directories_only: Option<bool>,
    pub show_full_path: Option<bool>,
}

impl MergeInto<ListingOptions> for PartialListingOptions {
    fn merge_into(self, dest: &mut ListingOptions) {
        if let Some(v) = self.max_depth {
            dest.max_depth = v;
        }
        if let Some(v) = self.show_hidden {
            dest.show_hidden = v;
        }
        if let Some(v) = self.list_directories_only {
            dest.list_directories_only = v;
        }
        if let Some(v) = self.show_full_path {
            dest.show_full_path = v;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialFilteringOptions {
    pub match_patterns: Option<Option<Vec<String>>>,
    pub ignore_patterns: Option<Option<Vec<String>>>,
    pub use_gitignore_rules: Option<bool>,
    pub gitignore_file: Option<Option<Vec<std::path::PathBuf>>>,
    pub case_insensitive_filter: Option<bool>,
    pub prune_empty_directories: Option<bool>,

    pub apply_include_patterns: Option<Option<Vec<String>>>,
    pub apply_exclude_patterns: Option<Option<Vec<String>>>,
    pub min_file_size: Option<Option<u64>>,
    pub max_file_size: Option<Option<u64>>,
}

impl MergeInto<FilteringOptions> for PartialFilteringOptions {
    fn merge_into(self, dest: &mut FilteringOptions) {
        if let Some(v) = self.match_patterns {
            dest.match_patterns = v;
        }
        if let Some(v) = self.ignore_patterns {
            dest.ignore_patterns = v;
        }
        if let Some(v) = self.use_gitignore_rules {
            dest.use_gitignore_rules = v;
        }
        if let Some(v) = self.gitignore_file {
            dest.gitignore_file = v;
        }
        if let Some(v) = self.case_insensitive_filter {
            dest.case_insensitive_filter = v;
        }
        if let Some(v) = self.prune_empty_directories {
            dest.prune_empty_directories = v;
        }

        if let Some(v) = self.apply_include_patterns {
            dest.apply_include_patterns = v;
        }
        if let Some(v) = self.apply_exclude_patterns {
            dest.apply_exclude_patterns = v;
        }
        if let Some(v) = self.min_file_size {
            dest.min_file_size = v;
        }
        if let Some(v) = self.max_file_size {
            dest.max_file_size = v;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialSortingOptions {
    pub sort_by: Option<Option<super::sorting::SortKey>>, // None=None, Some(None)=explicit null? might not happen
    pub reverse_sort: Option<bool>,
    pub files_before_directories: Option<bool>,
}

impl MergeInto<SortingOptions> for PartialSortingOptions {
    fn merge_into(self, dest: &mut SortingOptions) {
        if let Some(v) = self.sort_by {
            dest.sort_by = v;
        }
        if let Some(v) = self.reverse_sort {
            dest.reverse_sort = v;
        }
        if let Some(v) = self.files_before_directories {
            dest.files_before_directories = v;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialMetadataOptions {
    pub show_size_bytes: Option<bool>,
    pub human_readable_size: Option<bool>,
    pub report_permissions: Option<bool>,
    pub show_last_modified: Option<bool>,
    pub report_change_time: Option<bool>,
    pub report_creation_time: Option<bool>,
    pub calculate_line_count: Option<bool>,
    pub calculate_word_count: Option<bool>,
    pub apply_function: Option<Option<super::metadata::BuiltInFunction>>,
    // external_function omitted for now.
}

impl MergeInto<MetadataOptions> for PartialMetadataOptions {
    fn merge_into(self, dest: &mut MetadataOptions) {
        if let Some(v) = self.show_size_bytes {
            dest.show_size_bytes = v;
        }
        if let Some(v) = self.human_readable_size {
            dest.human_readable_size = v;
        }
        if let Some(v) = self.report_permissions {
            dest.report_permissions = v;
        }
        if let Some(v) = self.show_last_modified {
            dest.show_last_modified = v;
        }
        if let Some(v) = self.report_change_time {
            dest.report_change_time = v;
        }
        if let Some(v) = self.report_creation_time {
            dest.report_creation_time = v;
        }
        if let Some(v) = self.calculate_line_count {
            dest.calculate_line_count = v;
        }
        if let Some(v) = self.calculate_word_count {
            dest.calculate_word_count = v;
        }
        if let Some(v) = self.apply_function {
            dest.apply_function = v;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialHtmlOptions {
    pub base_href: Option<Option<String>>,
    pub strip_first_component: Option<bool>,
    pub custom_intro: Option<Option<std::path::PathBuf>>,
    pub custom_outro: Option<Option<std::path::PathBuf>>,
    pub include_links: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialLlmOptions {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub api_key_env: Option<String>,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

impl MergeInto<super::llm::LlmOptions> for PartialLlmOptions {
    fn merge_into(self, dest: &mut super::llm::LlmOptions) {
        if let Some(p) = self.provider {
            dest.enabled = true;
            dest.export_mode = false; /* placeholder */
            let _ = p;
        }
        // Only API-level merging used at CLI later; for now store nothing.
    }
}

impl MergeInto<HtmlOptions> for PartialHtmlOptions {
    fn merge_into(self, dest: &mut HtmlOptions) {
        if let Some(v) = self.base_href {
            dest.base_href = v;
        }
        if let Some(v) = self.strip_first_component {
            dest.strip_first_component = v;
        }
        if let Some(v) = self.custom_intro {
            dest.custom_intro = v;
        }
        if let Some(v) = self.custom_outro {
            dest.custom_outro = v;
        }
        if let Some(v) = self.include_links {
            dest.include_links = v;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialInputSourceOptions {
    pub root_display_name: Option<String>,
    pub root_node_size: Option<Option<u64>>, // keeps Option semantics from dest
    pub root_is_directory: Option<bool>,
}

impl MergeInto<InputSourceOptions> for PartialInputSourceOptions {
    fn merge_into(self, dest: &mut InputSourceOptions) {
        if let Some(v) = self.root_display_name {
            dest.root_display_name = v;
        }
        if let Some(v) = self.root_node_size {
            dest.root_node_size = v;
        }
        if let Some(v) = self.root_is_directory {
            dest.root_is_directory = v;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialMiscOptions {
    // Placeholder – currently no fields to set.
}

impl MergeInto<MiscOptions> for PartialMiscOptions {
    fn merge_into(self, _dest: &mut MiscOptions) {}
}

/* ------------------------------------------------------------------------- */
/*  Top-level PartialConfig                                                  */
/* ------------------------------------------------------------------------- */

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub input_source: Option<PartialInputSourceOptions>,
    pub listing: Option<PartialListingOptions>,
    pub filtering: Option<PartialFilteringOptions>,
    pub sorting: Option<PartialSortingOptions>,
    pub metadata: Option<PartialMetadataOptions>,
    pub html: Option<PartialHtmlOptions>,

    // New: LLM configuration (provider, model, api key indirection)
    pub llm: Option<PartialLlmOptions>,
    pub misc: Option<PartialMiscOptions>,
    // Unknown keys ignored for now.
}

impl PartialConfig {
    /// Merge this partial config into a fully-populated `RustreeLibConfig`.
    pub fn merge_into(self, dest: &mut RustreeLibConfig) {
        if let Some(src) = self.input_source {
            src.merge_into(&mut dest.input_source);
        }
        if let Some(src) = self.listing {
            src.merge_into(&mut dest.listing);
        }
        if let Some(src) = self.filtering {
            src.merge_into(&mut dest.filtering);
        }
        if let Some(src) = self.sorting {
            src.merge_into(&mut dest.sorting);
        }
        if let Some(src) = self.metadata {
            src.merge_into(&mut dest.metadata);
        }
        if let Some(src) = self.html {
            src.merge_into(&mut dest.html);
        }
        if let Some(src) = self.llm {
            src.merge_into(&mut dest.llm);
        }
        if let Some(src) = self.misc {
            src.merge_into(&mut dest.misc);
        }
        // Unknown keys are ignored for now.
    }
}
