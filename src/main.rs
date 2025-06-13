// src/main.rs

//! The `rustree` command-line application.
//!
//! This binary provides a CLI interface to the `rustree` library, allowing users
//! to generate directory tree listings with various analysis and formatting options.
//! It parses command-line arguments, translates them into library configurations,
//! invokes the library's core logic, and prints the results to standard output.

// The CLI module is part of this crate (rustree library crate), but not exposed publicly
use rustree::cli::{CliArgs, map_cli_to_lib_config, map_cli_to_lib_output_format};
use rustree::core::llm::{
    LlmClientFactory, LlmConfig, LlmError, LlmResponseProcessor, TreePromptFormatter,
};

use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use serde_json::{self, json};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    // Early custom help handling (e.g. `rustree help apply`)
    if let Some(section) = detect_section_help() {
        print_section_help(&section);
        return ExitCode::SUCCESS;
    }

    let cli_args = CliArgs::parse();

    // Handle shell-completion generation and exit early
    if let Some(shell) = cli_args.generate_completions {
        generate_shell_completions(shell);
        return ExitCode::SUCCESS;
    }

    // 1. Map CLI args to Library config
    let lib_config = match map_cli_to_lib_config(&cli_args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error reading pattern files: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let lib_output_format = map_cli_to_lib_output_format(cli_args.format.output_format.clone());

    // 2. Call the library to get processed nodes
    let nodes = match rustree::get_tree_nodes(&cli_args.path, &lib_config) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error processing directory: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // 3. Call the library to format the nodes
    let output_string = match rustree::format_nodes(&nodes, lib_output_format, &lib_config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error formatting output: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // 4. Handle LLM env generation first
    if cli_args.llm.llm_generate_env {
        println!(
            "{}",
            rustree::core::llm::LlmConfig::generate_sample_env_file()
        );
        eprintln!("💡 Save this content to a .env file in your project root or current directory");
        return ExitCode::SUCCESS;
    }

    // 5. Handle output based on CLI options
    if let Some(question) = &cli_args.llm.llm_export {
        let want_json = matches!(
            cli_args.format.output_format,
            Some(rustree::cli::output::CliOutputFormat::Json)
        );

        if want_json {
            let tree_json: serde_json::Value =
                serde_json::from_str(&output_string).unwrap_or_else(|_| json!(output_string));
            let out_val = json!({
                "tree": tree_json,
                "export_question": question
            });
            println!("{}", serde_json::to_string_pretty(&out_val).unwrap());
        } else {
            // Original text blocks
            println!("---BEGIN RUSTREE OUTPUT---");
            println!("{}", output_string);
            println!("---END RUSTREE OUTPUT---");
            println!("\n---BEGIN LLM QUESTION---");
            println!("{}", question);
            println!("---END LLM QUESTION---");
            eprintln!("\nHint: Pipe the above to your LLM tool.");
        }
    } else if let Some(question) = &cli_args.llm.llm_ask {
        // Send directly to LLM service
        let want_json = matches!(
            cli_args.format.output_format,
            Some(rustree::cli::output::CliOutputFormat::Json)
        );

        match handle_llm_query(&cli_args, question, &output_string, want_json).await {
            Ok(output_json_or_text) => {
                if !output_json_or_text.is_empty() {
                    println!("{}", output_json_or_text);
                }
            }
            Err(e) => {
                eprintln!("LLM Error: {}", e);
                return ExitCode::FAILURE;
            }
        }
    } else if cli_args.llm.dry_run {
        // --dry-run without --llm-ask
        eprintln!(
            "⚠️  --dry-run flag has no effect without --llm-ask. Showing tree output only.\n"
        );
        println!("{}", output_string);
    } else {
        println!("{}", output_string);
    }

    ExitCode::SUCCESS
}

/// Detects `rustree help <section>` style invocation before clap parsing.
fn detect_section_help() -> Option<String> {
    let mut args = std::env::args().skip(1); // skip bin name
    if let Some(first) = args.next() {
        if first == "help" {
            if let Some(section) = args.next() {
                return Some(section);
            }
        }
    }
    None
}

/// Prints only the requested help section.
fn print_section_help(section: &str) {
    let mut cmd = CliArgs::command();
    let help = cmd.render_long_help().to_string();

    let section_lc = section.to_lowercase();
    let mut printing = false;
    for line in help.lines() {
        let line_lc = strip_ansi_codes(line).to_lowercase();
        if line_lc.starts_with(&section_lc) && line_lc.ends_with(":") {
            printing = true;
            println!("{}", line);
            continue;
        }

        if printing {
            // another heading? detect by pattern "<Title>:"
            if line.trim_end().ends_with(":") && !line.starts_with(" ") {
                break;
            }
            println!("{}", line);
        }
    }
}

/// Naïve removal of ANSI colour escape sequences.
fn strip_ansi_codes(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    #[allow(clippy::while_let_on_iterator)]
    {
        let mut chars = input.chars();
        while let Some(c) = chars.next() {
            if c == '\u{1b}' {
                // skip until 'm'
                for nc in &mut chars {
                    if nc == 'm' {
                        break;
                    }
                }
            } else {
                output.push(c);
            }
        }
    }
    output
}

/// Generate shell completions to stdout
fn generate_shell_completions(shell: Shell) {
    let mut cmd = CliArgs::command();
    generate(shell, &mut cmd, "rustree", &mut std::io::stdout());
}

async fn handle_llm_query(
    cli_args: &CliArgs,
    question: &str,
    tree_output: &str,
    json_mode: bool,
) -> Result<String, LlmError> {
    // 1. Create LLM config from CLI args
    let llm_config = LlmConfig::from_cli_args(&cli_args.llm)?;

    // 2. Map CLI args to library config for prompt formatting
    let lib_config = match rustree::cli::map_cli_to_lib_config(cli_args) {
        Ok(config) => config,
        Err(e) => return Err(LlmError::Config(e.to_string())),
    };

    // 3. Format prompt with tree output and question
    let prompt = TreePromptFormatter::format_prompt(tree_output, question, &lib_config);

    use serde_json::json;

    // 4. Handle dry-run: build preview and skip network
    if cli_args.llm.dry_run {
        let preview = rustree::core::llm::RequestPreview::from_config(&llm_config, &prompt);

        if json_mode {
            // Attempt to parse tree_output as JSON, fallback to string
            let tree_json: serde_json::Value =
                serde_json::from_str(tree_output).unwrap_or_else(|_| json!(tree_output));

            let out_val = json!({
                "tree": tree_json,
                "llm": {
                    "dry_run": true,
                    "request": preview,
                    "question": question
                }
            });
            return Ok(serde_json::to_string_pretty(&out_val).unwrap());
        }

        let output = if cli_args.llm.human_friendly {
            preview.pretty_print_markdown()
        } else {
            preview.pretty_print()
        };
        println!("{}", output);
        return Ok(String::new());
    }

    // 5. Send to LLM and get response
    let response = LlmClientFactory::create_and_query(&llm_config, &prompt).await?;

    if json_mode {
        let tree_json: serde_json::Value =
            serde_json::from_str(tree_output).unwrap_or_else(|_| json!(tree_output));
        let out_val = json!({
            "tree": tree_json,
            "llm": {
                "dry_run": false,
                "provider": llm_config.provider.name(),
                "model": llm_config.model,
                "question": question,
                "response": response
            }
        });
        Ok(serde_json::to_string_pretty(&out_val).unwrap())
    } else {
        // 6. Format response for display
        Ok(LlmResponseProcessor::format_response(&response, question))
    }
}
