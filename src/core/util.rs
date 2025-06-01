//! Utility functions for the core module.
//!
//! This module contains general-purpose utility functions that are used
//! across multiple core modules but don't belong to any specific domain.

use std::path::Path;

/// Determines if a path represents a hidden file or directory.
///
/// A file or directory is considered hidden if its name starts with a dot (.).
/// This follows Unix convention and is commonly used across platforms.
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// `true` if the path represents a hidden file/directory, `false` otherwise.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// # use rustree::core::util::is_hidden;
///
/// assert_eq!(is_hidden(Path::new(".hidden")), true);
/// assert_eq!(is_hidden(Path::new("visible.txt")), false);
/// assert_eq!(is_hidden(Path::new("/path/to/.hidden")), true);
/// ```
pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

/// Formats a file size in bytes to a human-readable string.
///
/// This function converts byte counts to appropriate units (B, KB, MB, GB, TB)
/// with reasonable precision for display purposes.
///
/// # Arguments
///
/// * `bytes` - The size in bytes
///
/// # Returns
///
/// A formatted string representing the size with appropriate units.
///
/// # Examples
///
/// ```
/// # use rustree::core::util::format_size;
///
/// assert_eq!(format_size(1024), "1.0 KB");
/// assert_eq!(format_size(1536), "1.5 KB");
/// assert_eq!(format_size(1048576), "1.0 MB");
/// assert_eq!(format_size(512), "512 B");
/// ```
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes_f = bytes as f64;
    let unit_index = (bytes_f.log2() / THRESHOLD.log2()) as usize;

    if unit_index == 0 {
        format!("{} B", bytes)
    } else if unit_index < UNITS.len() {
        let size = bytes_f / THRESHOLD.powi(unit_index as i32);
        format!("{:.1} {}", size, UNITS[unit_index])
    } else {
        // For extremely large files, use TB with higher precision
        let size = bytes_f / THRESHOLD.powi((UNITS.len() - 1) as i32);
        format!("{:.2} {}", size, UNITS[UNITS.len() - 1])
    }
}

/// Safely truncates a string to a maximum length, adding ellipsis if necessary.
///
/// This function ensures that displayed strings don't exceed specified lengths
/// while providing visual indication when content has been truncated.
///
/// # Arguments
///
/// * `s` - The string to potentially truncate
/// * `max_len` - Maximum allowed length (including ellipsis if added)
///
/// # Returns
///
/// A string that is at most `max_len` characters long.
///
/// # Examples
///
/// ```
/// # use rustree::core::util::truncate_string;
///
/// assert_eq!(truncate_string("short", 10), "short");
/// assert_eq!(truncate_string("this is a very long filename.txt", 15), "this is a ve...");
/// assert_eq!(truncate_string("exact", 5), "exact");
/// ```
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if max_len <= 3 {
        // If max_len is too small for ellipsis, just truncate hard
        s.chars().take(max_len).collect()
    } else if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden(Path::new(".hidden")));
        assert!(is_hidden(Path::new(".config")));
        assert!(is_hidden(Path::new("/path/to/.hidden")));

        assert!(!is_hidden(Path::new("visible.txt")));
        assert!(!is_hidden(Path::new("normal_file")));
        assert!(!is_hidden(Path::new("/path/to/normal")));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("exact", 5), "exact");
        assert_eq!(truncate_string("toolong", 5), "to...");
        assert_eq!(
            truncate_string("this is a very long string", 15),
            "this is a ve..."
        );

        // Edge case: max_len too small for ellipsis
        assert_eq!(truncate_string("test", 2), "te");
    }
}
