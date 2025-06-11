// src/core/llm/response.rs

pub struct LlmResponseProcessor;

impl LlmResponseProcessor {
    pub fn format_response(response: &str, question: &str) -> String {
        // Clean up response and add nice formatting
        let cleaned_response = Self::clean_response(response);

        format!(
            "🤖 LLM Analysis\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            Question: {}\n\n\
            {}\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            question, cleaned_response
        )
    }

    fn clean_response(response: &str) -> String {
        // Remove extra whitespace and format nicely
        response
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn format_error(error: &str, question: &str) -> String {
        format!(
            "❌ LLM Error\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            Question: {}\n\
            Error: {}\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            question, error
        )
    }
}
