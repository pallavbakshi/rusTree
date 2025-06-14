//! Helpers for locating, loading and merging persistent configuration files.
//!
//! The lookup precedence follows the PRD:
//! 1. Explicit files passed via `--config-file` (handled later by CLI)
//! 2. Project file `./.rustree/config.toml`
//! 3. Global file `$XDG_CONFIG_HOME/rustree/config.toml` (via `dirs::config_dir()`)
//! 4. Built-in defaults (already covered by `RustreeLibConfig::default()`)

use std::fs;
use std::path::{Path, PathBuf};

use crate::core::error::RustreeError;

use super::partial::{
    PartialConfig, PartialFilteringOptions, PartialListingOptions, PartialSortingOptions,
};

/// Return path to `./.rustree/config.toml` if it exists.
pub fn project_file() -> Option<PathBuf> {
    let p = Path::new(".rustree").join("config.toml");
    if p.exists() { Some(p) } else { None }
}

/// Return `$XDG_CONFIG_HOME/rustree/config.toml` (or platform equivalent) if it exists.
pub fn global_file() -> Option<PathBuf> {
    let base_dir = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            // Fallback to ~/.config on Unix-like systems
            if let Some(home) = std::env::var_os("HOME") {
                let mut p = PathBuf::from(home);
                p.push(".config");
                Some(p)
            } else {
                None
            }
        });

    let mut base = base_dir?;
    base.push("rustree");
    base.push("config.toml");
    if base.exists() { Some(base) } else { None }
}

/// Load a single TOML file into a [`PartialConfig`].
pub fn load_toml(path: &Path) -> Result<PartialConfig, RustreeError> {
    let data = fs::read_to_string(path)?;
    // Basic secret-file permission check before deserialisation
    scan_llm_api_key_risks(path, &data);

    parse_simple_toml(&data).map_err(RustreeError::TreeBuildError)
}

/// Extremely small TOML subset parser sufficient for our current needs.
/// Accepts only:
/// * Top-level tables `[section]` (no nested tables).
/// * Key/Value lines inside a table.
/// * Booleans `true`/`false`, integers, quoted strings, and string arrays
///   like `["*.rs", "*.md"]`.
///
/// Parsing failures return a string-based error.
fn parse_simple_toml(input: &str) -> Result<PartialConfig, String> {
    let mut cfg = PartialConfig::default();
    let mut current = String::new();

    for (lineno, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current = line[1..line.len() - 1].trim().to_lowercase();
            continue;
        }

        // Expect key = value
        let mut parts = line.splitn(2, '=');
        let key = parts
            .next()
            .ok_or_else(|| format!("Line {}: missing key", lineno + 1))?
            .trim();
        let value = parts
            .next()
            .ok_or_else(|| format!("Line {}: missing value", lineno + 1))?
            .trim();

        match current.as_str() {
            "listing" => {
                let partial = cfg
                    .listing
                    .get_or_insert_with(PartialListingOptions::default);
                match key {
                    "show_hidden" => partial.show_hidden = Some(parse_bool(value)?),
                    "list_directories_only" => {
                        partial.list_directories_only = Some(parse_bool(value)?)
                    }
                    "show_full_path" => partial.show_full_path = Some(parse_bool(value)?),
                    "max_depth" => partial.max_depth = Some(Some(parse_usize(value)?)),
                    _ => {}
                }
            }
            "filtering" => {
                let partial = cfg
                    .filtering
                    .get_or_insert_with(PartialFilteringOptions::default);
                match key {
                    "use_gitignore_rules" => partial.use_gitignore_rules = Some(parse_bool(value)?),
                    "case_insensitive_filter" => {
                        partial.case_insensitive_filter = Some(parse_bool(value)?)
                    }
                    "prune_empty_directories" => {
                        partial.prune_empty_directories = Some(parse_bool(value)?)
                    }
                    "match_patterns" => {
                        partial.match_patterns = Some(Some(parse_string_array(value)?))
                    }
                    "ignore_patterns" => {
                        partial.ignore_patterns = Some(Some(parse_string_array(value)?))
                    }
                    _ => {}
                }
            }
            "sorting" => {
                let partial = cfg
                    .sorting
                    .get_or_insert_with(PartialSortingOptions::default);
                match key {
                    "reverse" | "reverse_sort" => partial.reverse_sort = Some(parse_bool(value)?),
                    "files_before_directories" => {
                        partial.files_before_directories = Some(parse_bool(value)?)
                    }
                    "sort_by" => {
                        let s = parse_string(value)?;
                        let key_variant = match s.to_ascii_lowercase().as_str() {
                            "name" => Some(super::sorting::SortKey::Name),
                            "size" => Some(super::sorting::SortKey::Size),
                            "mtime" => Some(super::sorting::SortKey::MTime),
                            "ctime" | "changetime" => Some(super::sorting::SortKey::ChangeTime),
                            "creationtime" | "crtime" => Some(super::sorting::SortKey::CreateTime),
                            "version" => Some(super::sorting::SortKey::Version),
                            "none" => Some(super::sorting::SortKey::None),
                            _ => None,
                        };
                        partial.sort_by = Some(key_variant);
                    }
                    _ => {}
                }
            }
            "llm" => {
                use crate::config::partial::PartialLlmOptions;
                let partial = cfg.llm.get_or_insert_with(PartialLlmOptions::default);
                match key {
                    "provider" | "llm_provider" => partial.provider = Some(parse_string(value)?),
                    "model" | "llm_model" => partial.model = Some(parse_string(value)?),
                    "api_key_env" => partial.api_key_env = Some(parse_string(value)?),
                    "api_key" => partial.api_key = Some(parse_string(value)?),
                    "endpoint" | "llm_endpoint" => partial.endpoint = Some(parse_string(value)?),
                    "temperature" | "llm_temperature" => {
                        partial.temperature = parse_float(value).ok()
                    }
                    "max_tokens" | "llm_max_tokens" => partial.max_tokens = parse_uint(value).ok(),
                    _ => {}
                }
            }
            _ => {
                // Unknown section – ignore for now
            }
        }
    }

    Ok(cfg)
}

fn parse_bool(s: &str) -> Result<bool, String> {
    match s.trim() {
        "true" | "True" | "TRUE" => Ok(true),
        "false" | "False" | "FALSE" => Ok(false),
        other => Err(format!("Invalid boolean value '{other}'")),
    }
}

fn parse_string(s: &str) -> Result<String, String> {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len() - 1];
        let unescaped = inner.replace("\\\"", "\"").replace("\\\\", "\\");
        Ok(unescaped)
    } else {
        Ok(s.to_string())
    }
}

fn parse_usize(s: &str) -> Result<usize, String> {
    let s = s.trim();
    s.parse::<usize>()
        .map_err(|e| format!("Invalid integer '{s}': {e}"))
}

fn parse_string_array(s: &str) -> Result<Vec<String>, String> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return Err(format!("Expected array, got '{s}'"));
    }
    let inner = &s[1..s.len() - 1];
    let mut items = Vec::new();
    for part in inner.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        items.push(parse_string(part)?);
    }
    Ok(items)
}

fn parse_float(s: &str) -> Result<f32, String> {
    s.trim()
        .parse::<f32>()
        .map_err(|e| format!("Invalid float '{}': {e}", s.trim()))
}

fn parse_uint(s: &str) -> Result<u32, String> {
    s.trim()
        .parse::<u32>()
        .map_err(|e| format!("Invalid integer '{}': {e}", s.trim()))
}

// -------------------------------------------------------------------------
//  Security helper – warn when api_key_file is world-readable
// -------------------------------------------------------------------------

#[cfg(unix)]
fn world_readable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    match fs::metadata(path) {
        Ok(meta) => meta.permissions().mode() & 0o044 != 0,
        Err(_) => false,
    }
}

#[cfg(not(unix))]
fn world_readable(_path: &Path) -> bool {
    false // Not implemented on Windows for now
}

/// Scan TOML text for `api_key_file` directives and warn if the referenced file
/// is world-readable.  This is a *best-effort* heuristic that does not attempt
/// full TOML parsing – good enough for Phase-4 requirements.
fn scan_llm_api_key_risks(cfg_path: &Path, toml_text: &str) {
    for line in toml_text.lines() {
        let l = line.trim_start();
        if !l.starts_with("api_key_file") {
            continue;
        }
        if let Some(eq_pos) = l.find('=') {
            let value_part = l[eq_pos + 1..].trim();
            if let Some(stripped) = value_part
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
            {
                let key_path = cfg_path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(stripped);
                if world_readable(&key_path) {
                    eprintln!(
                        "⚠️  rustree: api_key_file '{}' is world-readable; consider tightening permissions (chmod 600)",
                        key_path.display()
                    );
                }
            }
        }
    }
}

/// Load all configuration files following the PRD precedence and merge them.
///
/// * `explicit_files` – a slice of paths given on the CLI in the order they
///   appeared.  They take highest precedence with *last one winning*.
/// * `include_defaults` – if `false`, project + global discovery is skipped.
///
/// Returns the merged config **plus** the list of files that were successfully
/// loaded and applied, in the order they were merged (low → high priority).
pub fn load_merged(
    explicit_files: &[PathBuf],
    include_defaults: bool,
) -> Result<(PartialConfig, Vec<PathBuf>), RustreeError> {
    let mut merged = PartialConfig::default();
    let mut sources = Vec::<PathBuf>::new();

    // 3. Global
    if include_defaults {
        if let Some(p) = global_file() {
            if let Ok(cfg) = load_toml(&p) {
                cfg.merge_into_config(&mut merged);
                sources.push(p);
            }
        }

        // 2. Project
        if let Some(p) = project_file() {
            if let Ok(cfg) = load_toml(&p) {
                cfg.merge_into_config(&mut merged);
                sources.push(p);
            }
        }
    }

    // 1. Explicit files in order, last overrides.
    for p in explicit_files {
        let cfg = load_toml(p)?;
        cfg.merge_into_config(&mut merged);
        sources.push(p.clone());
    }

    Ok((merged, sources))
}

// -------------------------------------------------------------------------
// Internal helper trait implementation to reuse same merge logic.
// -------------------------------------------------------------------------

trait MergePartial {
    fn merge_into_config(self, dest: &mut PartialConfig);
}

impl MergePartial for PartialConfig {
    fn merge_into_config(self, dest: &mut PartialConfig) {
        // For every Option field: if `self` has Some, overwrite dest.
        macro_rules! merge_field {
            ($fname:ident) => {
                if self.$fname.is_some() {
                    dest.$fname = self.$fname;
                }
            };
        }

        merge_field!(input_source);
        merge_field!(listing);
        merge_field!(filtering);
        merge_field!(sorting);
        merge_field!(metadata);
        merge_field!(html);
        merge_field!(llm);
        merge_field!(misc);

        // Unknown keys ignored for now.
    }
}
