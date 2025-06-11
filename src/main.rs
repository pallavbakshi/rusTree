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

use clap::Parser;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let cli_args = CliArgs::parse();

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
        // Export formatted query for external LLM tools (preserves existing behavior)
        println!("---BEGIN RUSTREE OUTPUT---");
        println!("{}", output_string);
        println!("---END RUSTREE OUTPUT---");
        println!("\n---BEGIN LLM QUESTION---");
        println!("{}", question);
        println!("---END LLM QUESTION---");
        eprintln!("\nHint: Pipe the above to your LLM tool.");
    } else if let Some(question) = &cli_args.llm.llm_ask {
        // Send directly to LLM service (new functionality)
        match handle_llm_query(&cli_args, question, &output_string).await {
            Ok(response) => println!("{}", response),
            Err(e) => {
                eprintln!("LLM Error: {}", e);
                return ExitCode::FAILURE;
            }
        }
    } else {
        println!("{}", output_string);
    }

    ExitCode::SUCCESS
}

async fn handle_llm_query(
    cli_args: &CliArgs,
    question: &str,
    tree_output: &str,
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

    // 4. Send to LLM and get response
    let response = LlmClientFactory::create_and_query(&llm_config, &prompt).await?;

    // 5. Format response for display
    Ok(LlmResponseProcessor::format_response(&response, question))
}
