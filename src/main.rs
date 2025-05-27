// src/main.rs
mod cli; // Make cli module (and its submodules) available

use clap::Parser;
use cli::{args::CliArgs, handler};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli_args = CliArgs::parse();

    // 1. Map CLI args to Library config
    let lib_config = handler::map_cli_to_lib_config(&cli_args);
    let lib_output_format = handler::map_cli_to_lib_output_format(cli_args.output_format.clone());


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

    // 4. Handle output based on CLI options
    if let Some(question) = &cli_args.llm_ask {
        // Prepare for piping (as discussed, user handles the actual pipe)
        println!("---BEGIN RUSTREE OUTPUT---");
        println!("{}", output_string);
        println!("---END RUSTREE OUTPUT---");
        println!("\n---BEGIN LLM QUESTION---");
        println!("{}", question);
        println!("---END LLM QUESTION---");
        eprintln!("\nHint: Pipe the above to your LLM tool.");
    } else {
        println!("{}", output_string);
    }

    ExitCode::SUCCESS
}