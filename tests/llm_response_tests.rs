//! Integration tests for LLM response processing

use rustree::core::llm::LlmResponseProcessor;

#[test]
fn test_basic_response_formatting() {
    let response = "This is a test response from the LLM.";
    let question = "What is this project?";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("🤖 LLM Analysis"));
    assert!(formatted.contains("Question: What is this project?"));
    assert!(formatted.contains("This is a test response from the LLM."));
    assert!(formatted.contains("━━━")); // Unicode line separator
}

#[test]
fn test_response_cleaning() {
    let messy_response =
        "\n\n  This is a response with  \n\n  extra whitespace  \n\n  and empty lines  \n\n";
    let question = "Clean this up";

    let formatted = LlmResponseProcessor::format_response(messy_response, question);

    // Should clean up the response
    assert!(formatted.contains("This is a response with"));
    assert!(formatted.contains("extra whitespace"));
    assert!(formatted.contains("and empty lines"));

    // Should not contain multiple consecutive newlines in the cleaned section
    let response_part = formatted
        .lines()
        .skip_while(|line| !line.contains("Question:"))
        .skip(2) // Skip question line and empty line
        .take_while(|line| !line.contains("━━━"))
        .collect::<Vec<_>>()
        .join("\n");

    assert!(!response_part.contains("\n\n\n"));
}

#[test]
fn test_error_formatting() {
    let error_msg = "API rate limit exceeded";
    let question = "What went wrong?";

    let formatted = LlmResponseProcessor::format_error(error_msg, question);

    assert!(formatted.contains("❌ LLM Error"));
    assert!(formatted.contains("Question: What went wrong?"));
    assert!(formatted.contains("Error: API rate limit exceeded"));
    assert!(formatted.contains("━━━"));
}

#[test]
fn test_multiline_response() {
    let response = "This is a multi-line response.\n\nIt has several paragraphs.\n\nAnd some bullet points:\n- Point 1\n- Point 2\n- Point 3";
    let question = "Give me a detailed analysis";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("multi-line response"));
    assert!(formatted.contains("several paragraphs"));
    assert!(formatted.contains("- Point 1"));
    assert!(formatted.contains("- Point 2"));
    assert!(formatted.contains("- Point 3"));
}

#[test]
fn test_response_with_code_blocks() {
    let response = "Here's some code:\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\n\nThis is the explanation.";
    let question = "Show me code";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("Here's some code:"));
    assert!(formatted.contains("```rust"));
    assert!(formatted.contains("fn main()"));
    assert!(formatted.contains("println!"));
    assert!(formatted.contains("This is the explanation"));
}

#[test]
fn test_empty_response() {
    let response = "";
    let question = "Empty test";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("🤖 LLM Analysis"));
    assert!(formatted.contains("Question: Empty test"));
    // Should handle empty response gracefully
}

#[test]
fn test_whitespace_only_response() {
    let response = "   \n\n  \t  \n   ";
    let question = "Whitespace test";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("🤖 LLM Analysis"));
    assert!(formatted.contains("Question: Whitespace test"));
    // The cleaned response section should be minimal/empty
}

#[test]
fn test_long_question() {
    let response = "Short answer.";
    let question = "This is a very long question that goes on and on and might span multiple lines if it were to be displayed in a narrow terminal window and contains lots of details about what exactly I want to know about this project structure and architecture";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("🤖 LLM Analysis"));
    assert!(formatted.contains("This is a very long question"));
    assert!(formatted.contains("Short answer."));
}

#[test]
fn test_special_characters_in_response() {
    let response = "Response with special chars: @#$%^&*(){}[]|\\;:'\",.<>?/~`";
    let question = "Special chars test";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("@#$%^&*(){}[]|\\;:'\",.<>?/~`"));
}

#[test]
fn test_unicode_in_response() {
    let response = "Unicode test: 🚀 🌟 ✨ 文字 файл тест";
    let question = "Unicode question: 测试 тест";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("🚀 🌟 ✨"));
    assert!(formatted.contains("文字 файл тест"));
    assert!(formatted.contains("测试 тест"));
}

#[test]
fn test_response_structure() {
    let response = "Test response";
    let question = "Test question";

    let formatted = LlmResponseProcessor::format_response(response, question);

    let lines: Vec<&str> = formatted.lines().collect();

    // Check structure
    assert!(lines[0].contains("🤖 LLM Analysis"));
    assert!(lines[1].contains("━━━")); // Top border
    assert!(lines[2].contains("Question: Test question"));
    assert!(lines[3].is_empty()); // Empty line after question
    assert!(lines[4].contains("Test response"));
    assert!(lines[5].contains("━━━")); // Bottom border
}

#[test]
fn test_error_structure() {
    let error = "Test error";
    let question = "Test question";

    let formatted = LlmResponseProcessor::format_error(error, question);

    let lines: Vec<&str> = formatted.lines().collect();

    // Check structure
    assert!(lines[0].contains("❌ LLM Error"));
    assert!(lines[1].contains("━━━")); // Top border
    assert!(lines[2].contains("Question: Test question"));
    assert!(lines[3].contains("Error: Test error"));
    assert!(lines[4].contains("━━━")); // Bottom border
}

#[test]
fn test_clean_response_function() {
    // Test the cleaning behavior specifically
    let test_cases = vec![
        ("  hello  ", "hello"),
        ("\n\nhello\n\n", "hello"),
        ("  hello  \n  world  ", "hello\nworld"),
        ("\n\n\n", ""),
        ("hello\n\n\nworld", "hello\nworld"),
        ("  \n  hello  \n  \n  world  \n  ", "hello\nworld"),
    ];

    for (input, expected) in test_cases {
        let formatted = LlmResponseProcessor::format_response(input, "test");

        // Extract just the response part (between the question and bottom border)
        let lines: Vec<&str> = formatted.lines().collect();
        let response_start = lines
            .iter()
            .position(|line| line.contains("Question:"))
            .unwrap()
            + 2; // Skip question and empty line
        let response_end = lines.iter().rposition(|line| line.contains("━━━")).unwrap();

        let cleaned_response = lines[response_start..response_end]
            .join("\n")
            .trim()
            .to_string();

        assert_eq!(cleaned_response, expected, "Failed for input: {:?}", input);
    }
}

#[test]
fn test_response_with_json() {
    let response = r#"Here's a JSON response:
{
  "analysis": "This is a well-structured project",
  "suggestions": ["Add tests", "Update docs"],
  "score": 8.5
}
That's the analysis."#;
    let question = "Analyze as JSON";

    let formatted = LlmResponseProcessor::format_response(response, question);

    assert!(formatted.contains("Here's a JSON response:"));
    assert!(formatted.contains("\"analysis\""));
    assert!(formatted.contains("\"suggestions\""));
    assert!(formatted.contains("That's the analysis."));
}
