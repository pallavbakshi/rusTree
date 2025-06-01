//! Time and date formatting utilities.
//!
//! This module contains functionality for formatting timestamps and dates
//! in various formats for display purposes.

use std::time::{SystemTime, UNIX_EPOCH};

/// Formats a `SystemTime` as a Unix timestamp (seconds since epoch).
///
/// This provides a simple numeric representation of time that's useful
/// for sorting and basic display purposes.
///
/// # Arguments
///
/// * `time` - The `SystemTime` to format
///
/// # Returns
///
/// The number of seconds since Unix epoch, or 0 if the time is invalid.
///
/// # Examples
///
/// ```
/// use std::time::{SystemTime, UNIX_EPOCH, Duration};
/// # use rustree::core::metadata::time_formatter::format_timestamp;
///
/// let time = UNIX_EPOCH + Duration::from_secs(1000);
/// assert_eq!(format_timestamp(time), 1000);
/// ```
pub fn format_timestamp(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Formats a `SystemTime` as a human-readable relative time string.
///
/// This provides user-friendly time descriptions like "2 minutes ago",
/// "1 hour ago", etc.
///
/// # Arguments
///
/// * `time` - The `SystemTime` to format
/// * `reference` - The reference time to compare against (usually `SystemTime::now()`)
///
/// # Returns
///
/// A string describing the relative time difference.
///
/// # Examples
///
/// ```
/// use std::time::{SystemTime, Duration};
/// # use rustree::core::metadata::time_formatter::format_relative_time;
///
/// let now = SystemTime::now();
/// let past = now - Duration::from_secs(3600); // 1 hour ago
/// let relative = format_relative_time(past, now);
/// assert!(relative.contains("hour"));
/// ```
pub fn format_relative_time(time: SystemTime, reference: SystemTime) -> String {
    match reference.duration_since(time) {
        Ok(duration) => {
            let secs = duration.as_secs();

            if secs < 60 {
                format!("{} second{} ago", secs, if secs == 1 { "" } else { "s" })
            } else if secs < 3600 {
                let mins = secs / 60;
                format!("{} minute{} ago", mins, if mins == 1 { "" } else { "s" })
            } else if secs < 86400 {
                let hours = secs / 3600;
                format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
            } else {
                let days = secs / 86400;
                format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
            }
        }
        Err(_) => {
            // Time is in the future relative to reference
            match time.duration_since(reference) {
                Ok(duration) => {
                    let secs = duration.as_secs();
                    if secs < 60 {
                        "in the future".to_string()
                    } else {
                        "future".to_string()
                    }
                }
                Err(_) => "unknown".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_format_timestamp() {
        let time = UNIX_EPOCH + Duration::from_secs(1234567);
        assert_eq!(format_timestamp(time), 1234567);

        // Test edge case with time before epoch (should return 0)
        if let Some(past) = UNIX_EPOCH.checked_sub(Duration::from_secs(100)) {
            assert_eq!(format_timestamp(past), 0);
        }
    }

    #[test]
    fn test_format_relative_time() {
        let now = SystemTime::now();

        // Test seconds ago
        let secs_ago = now - Duration::from_secs(30);
        let relative = format_relative_time(secs_ago, now);
        assert!(relative.contains("30 seconds ago"));

        // Test minutes ago
        let mins_ago = now - Duration::from_secs(120);
        let relative = format_relative_time(mins_ago, now);
        assert!(relative.contains("2 minutes ago"));

        // Test hours ago
        let hours_ago = now - Duration::from_secs(7200);
        let relative = format_relative_time(hours_ago, now);
        assert!(relative.contains("2 hours ago"));

        // Test days ago
        let days_ago = now - Duration::from_secs(172800);
        let relative = format_relative_time(days_ago, now);
        assert!(relative.contains("2 days ago"));
    }
}
