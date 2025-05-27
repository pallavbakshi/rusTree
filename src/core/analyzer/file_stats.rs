// src/core/analyzer/file_stats.rs

// For easier unit testing, you might have internal pure functions:
pub fn count_lines_from_string(content: &str) -> usize {
    content.lines().count()
}

pub fn count_words_from_string(content: &str) -> usize {
    content.split_whitespace().count()
}


#[cfg(test)]
mod tests {
    use super::*; // Imports functions from the parent module (file_stats)

    #[test]
    fn test_count_lines_empty() {
        assert_eq!(count_lines_from_string(""), 0); // An empty string has 0 lines if it doesn't contain a newline character. Behavior might differ if "" is considered 1 line.
    }
    
    #[test]
    fn test_count_lines_single_no_newline() {
        assert_eq!(count_lines_from_string("hello world"), 1);
    }

    #[test]
    fn test_count_lines_single_with_newline() {
        assert_eq!(count_lines_from_string("hello world\n"), 1); // .lines() iterator behavior
    }


    #[test]
    fn test_count_lines_multiple() {
        assert_eq!(count_lines_from_string("hello\nworld\nfrom rust"), 3);
    }

    #[test]
    fn test_count_lines_trailing_newline_behavior() {
        // .lines() does not count a trailing empty line after the last newline
        assert_eq!(count_lines_from_string("hello\nworld\n"), 2);
        assert_eq!(count_lines_from_string("hello\nworld\n\n"), 3); // two trailing newlines means one empty line
    }
    
    #[test]
    fn test_count_lines_only_newlines() {
        assert_eq!(count_lines_from_string("\n"), 1);
        assert_eq!(count_lines_from_string("\n\n"), 2);
    }


    #[test]
    fn test_count_words_empty() {
        assert_eq!(count_words_from_string(""), 0);
    }

    #[test]
    fn test_count_words_single() {
        assert_eq!(count_words_from_string("hello"), 1);
    }

    #[test]
    fn test_count_words_multiple() {
        assert_eq!(count_words_from_string("hello world from rust"), 4);
    }

    #[test]
    fn test_count_words_extra_whitespace() {
        assert_eq!(count_words_from_string("  hello   world  "), 2);
    }
}