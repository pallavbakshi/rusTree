// src/core/llm/prompt.rs

use crate::config::RustreeLibConfig;

pub struct TreePromptFormatter;

impl TreePromptFormatter {
    pub fn format_prompt(
        tree_output: &str,
        user_question: &str,
        tree_config: &RustreeLibConfig,
    ) -> String {
        let metadata_info = Self::extract_metadata_info(tree_output, tree_config);

        format!(
            "You are analyzing a directory tree structure. Here's the tree output:\n\n\
            ```\n{}\n```\n\n\
            {}\
            Question: {}\n\n\
            Please provide a helpful analysis based on the directory structure above. \
            Focus on architectural patterns, code organization, potential issues, \
            and actionable insights.",
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
}
