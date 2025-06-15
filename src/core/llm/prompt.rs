// src/core/llm/prompt.rs

use crate::core::options::RustreeLibConfig;

pub struct TreePromptFormatter;

impl TreePromptFormatter {
    pub fn format_prompt(
        tree_output: &str,
        user_question: &str,
        tree_config: &RustreeLibConfig,
    ) -> String {
        let metadata_info = Self::extract_metadata_info(tree_output, tree_config);

        format!(
            "You are analyzing a directory tree structure.\n\n\
            <tree_output>\n{}\n</tree_output>\n\n\
            {}\
            <user_request>{}</user_request>",
            tree_output, metadata_info, user_question
        )
    }

    fn extract_metadata_info(tree_output: &str, tree_config: &RustreeLibConfig) -> String {
        let mut info = String::new();

        // Try to extract summary information from the output
        // Look for the last "Summary:" that appears to be a real summary (typically at the end)
        if let Some(summary_start) = tree_output.rfind("Summary:") {
            let summary_text = &tree_output[summary_start..];

            // Extract lines starting from Summary: until end or until we hit another section
            let summary_lines: Vec<&str> = summary_text
                .lines()
                .take_while(|line| {
                    // Take lines until we find something that looks like tree structure again
                    !line.contains("├──") && !line.contains("└──") && !line.contains("│")
                })
                .collect();

            if !summary_lines.is_empty() {
                let clean_summary = summary_lines.join("\n");
                info.push_str("Project Statistics:\n");
                info.push_str(&clean_summary);
                info.push_str("\n\n");
            }
        }

        // Add configuration context
        if tree_config.listing.max_depth.is_some() {
            info.push_str(&format!(
                "Note: Tree depth limited to {} levels\n",
                tree_config.listing.max_depth.unwrap()
            ));
        }

        if tree_config.listing.list_directories_only {
            info.push_str("Note: Showing directories only\n");
        }

        if !info.is_empty() {
            info.push('\n');
        }

        info
    }

    /// Format a prompt specifically for diff analysis with comprehensive context
    pub fn format_diff_prompt(
        diff_output: &str,
        old_tree_output: &str,
        new_tree_output: &str,
        user_question: &str,
        tree_config: &RustreeLibConfig,
    ) -> String {
        let diff_metadata = Self::extract_diff_metadata(diff_output);
        let config_info = Self::format_config_context(tree_config);

        format!(
            "You are analyzing changes between two directory tree snapshots.\n\n\
            <original_tree>\n{}\n</original_tree>\n\n\
            <updated_tree>\n{}\n</updated_tree>\n\n\
            <diff_analysis>\n{}\n</diff_analysis>\n\n\
            {}\
            {}\
            <user_request>{}</user_request>",
            old_tree_output,
            new_tree_output,
            diff_output,
            diff_metadata,
            config_info,
            user_question
        )
    }

    /// Detect if output contains diff information and use enhanced prompt if so
    pub fn format_prompt_auto(
        output: &str,
        user_question: &str,
        tree_config: &RustreeLibConfig,
        old_tree: Option<&str>,
        new_tree: Option<&str>,
    ) -> String {
        if let (Some(old), Some(new)) = (old_tree, new_tree) {
            if Self::is_diff_output(output) {
                Self::format_diff_prompt(output, old, new, user_question, tree_config)
            } else {
                Self::format_prompt(output, user_question, tree_config)
            }
        } else {
            Self::format_prompt(output, user_question, tree_config)
        }
    }

    fn is_diff_output(output: &str) -> bool {
        output.contains("Changes Summary:")
            || output.contains("[+]")
            || output.contains("[-]")
            || output.contains("[M]")
            || output.contains("[~]")
            || output.contains("[T]")
    }

    fn extract_diff_metadata(diff_output: &str) -> String {
        let mut info = String::new();

        // Extract summary information from diff output
        if let Some(summary_start) = diff_output.rfind("Changes Summary:") {
            let summary_text = &diff_output[summary_start..];
            let summary_lines: Vec<&str> = summary_text
                .lines()
                .take(10) // Limit to reasonable summary length
                .collect();

            if !summary_lines.is_empty() {
                info.push_str("Change Summary:\n");
                info.push_str(&summary_lines.join("\n"));
                info.push_str("\n\n");
            }
        }

        // Extract snapshot metadata if available
        if diff_output.contains("snapshot_file") || diff_output.contains("generated_at") {
            info.push_str("Analysis Context: Comparing two directory tree snapshots\n");
        }

        if !info.is_empty() {
            info.push('\n');
        }

        info
    }

    fn format_config_context(tree_config: &RustreeLibConfig) -> String {
        let mut info = String::new();

        if tree_config.listing.max_depth.is_some() {
            info.push_str(&format!(
                "Analysis depth: {} levels\n",
                tree_config.listing.max_depth.unwrap()
            ));
        }

        if tree_config.listing.list_directories_only {
            info.push_str("Scope: Directories only\n");
        }

        if let Some(ref patterns) = tree_config.filtering.match_patterns {
            if !patterns.is_empty() {
                info.push_str(&format!("Include filters: {:?}\n", patterns));
            }
        }

        if let Some(ref patterns) = tree_config.filtering.ignore_patterns {
            if !patterns.is_empty() {
                info.push_str(&format!("Exclude filters: {:?}\n", patterns));
            }
        }

        if !info.is_empty() {
            info.push('\n');
        }

        info
    }

    pub fn format_system_prompt() -> &'static str {
        "You are an expert software architect and code analyst. When analyzing directory \
        structures, focus on:\n\
        - Code organization and architectural patterns\n\
        - Potential maintenance and scalability issues\n\
        - Best practices and improvements\n\
        - Technology stack insights\n\
        - Project health indicators\n\n\
        Provide concise, actionable insights."
    }

    pub fn format_diff_system_prompt() -> &'static str {
        "You are an expert software architect and code analyst specializing in change analysis. \
        When analyzing directory tree changes, focus on:\n\
        - Impact assessment of structural changes\n\
        - Potential risks and benefits of modifications\n\
        - Code organization improvements or regressions\n\
        - Development workflow insights\n\
        - Migration and refactoring patterns\n\
        - Breaking changes and compatibility issues\n\n\
        Analyze the before/after states and provide actionable insights about the changes."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::RustreeLibConfig;

    fn create_test_config() -> RustreeLibConfig {
        RustreeLibConfig::default()
    }

    #[test]
    fn test_format_basic_prompt() {
        let tree_output = "./\n├── src/\n│   └── main.rs\n└── Cargo.toml";
        let question = "What's the structure of this project?";
        let config = create_test_config();

        let prompt = TreePromptFormatter::format_prompt(tree_output, question, &config);

        assert!(prompt.contains("<tree_output>"));
        assert!(prompt.contains(tree_output));
        assert!(prompt.contains("<user_request>"));
        assert!(prompt.contains(question));
        assert!(prompt.contains("analyzing a directory tree structure"));
    }

    #[test]
    fn test_format_diff_prompt() {
        let diff_output = "./\n├── [+] new_file.rs\n└── [-] old_file.rs\n\nChanges Summary:\n  1 files added (+)\n  1 files removed (-)";
        let old_tree = "./\n└── old_file.rs";
        let new_tree = "./\n└── new_file.rs";
        let question = "What changed in this project?";
        let config = create_test_config();

        let prompt = TreePromptFormatter::format_diff_prompt(
            diff_output,
            old_tree,
            new_tree,
            question,
            &config,
        );

        assert!(prompt.contains("<original_tree>"));
        assert!(prompt.contains(old_tree));
        assert!(prompt.contains("<updated_tree>"));
        assert!(prompt.contains(new_tree));
        assert!(prompt.contains("<diff_analysis>"));
        assert!(prompt.contains(diff_output));
        assert!(prompt.contains("<user_request>"));
        assert!(prompt.contains(question));
        assert!(prompt.contains("analyzing changes between two directory tree snapshots"));
    }

    #[test]
    fn test_is_diff_output() {
        assert!(TreePromptFormatter::is_diff_output(
            "Changes Summary: 1 files added"
        ));
        assert!(TreePromptFormatter::is_diff_output("├── [+] new_file.rs"));
        assert!(TreePromptFormatter::is_diff_output(
            "├── [-] deleted_file.rs"
        ));
        assert!(TreePromptFormatter::is_diff_output("├── [M] modified_dir/"));
        assert!(TreePromptFormatter::is_diff_output("├── [~] renamed.rs"));
        assert!(TreePromptFormatter::is_diff_output("├── [T] type_change"));

        assert!(!TreePromptFormatter::is_diff_output(
            "./\n├── src/\n└── Cargo.toml"
        ));
        assert!(!TreePromptFormatter::is_diff_output("normal tree output"));
    }

    #[test]
    fn test_extract_diff_metadata() {
        let diff_output = "./\n├── [+] new_file.rs\n\nChanges Summary:\n  1 files added (+)\n  2 files removed (-)";

        let metadata = TreePromptFormatter::extract_diff_metadata(diff_output);

        assert!(metadata.contains("Change Summary:"));
        assert!(metadata.contains("1 files added"));
        assert!(metadata.contains("2 files removed"));
    }

    #[test]
    fn test_format_prompt_auto_with_diff() {
        let diff_output = "./\n├── [+] new_file.rs\n\nChanges Summary:\n  1 files added";
        let old_tree = "./\n└── old_file.rs";
        let new_tree = "./\n└── new_file.rs";
        let question = "What changed?";
        let config = create_test_config();

        let prompt = TreePromptFormatter::format_prompt_auto(
            diff_output,
            question,
            &config,
            Some(old_tree),
            Some(new_tree),
        );

        // Should use diff prompt format
        assert!(prompt.contains("<original_tree>"));
        assert!(prompt.contains("<updated_tree>"));
        assert!(prompt.contains("<diff_analysis>"));
    }

    #[test]
    fn test_format_prompt_auto_without_diff() {
        let tree_output = "./\n├── src/\n└── Cargo.toml";
        let question = "What's this project?";
        let config = create_test_config();

        let prompt =
            TreePromptFormatter::format_prompt_auto(tree_output, question, &config, None, None);

        // Should use regular prompt format
        assert!(prompt.contains("<tree_output>"));
        assert!(!prompt.contains("<original_tree>"));
        assert!(!prompt.contains("<updated_tree>"));
    }
}
