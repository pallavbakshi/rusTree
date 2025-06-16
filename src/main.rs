// src/main.rs

//! The `rustree` command-line application.
//!
//! This binary provides a CLI interface to the `rustree` library, allowing users
//! to generate directory tree listings with various analysis and formatting options.
//! It parses command-line arguments, translates them into library configurations,
//! invokes the library's core logic, and prints the results to standard output.

// The CLI module is part of this crate (rustree library crate), but not exposed publicly
use rustree::cli::{
    CliArgs, map_cli_to_diff_options, map_cli_to_lib_config, map_cli_to_lib_output_format,
};
use rustree::core::llm::{
    LlmClientFactory, LlmConfig, LlmError, LlmResponseProcessor, TreePromptFormatter,
};
use rustree::{DiffEngine, DiffMetadata, format_diff};

use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use serde_json::{self, json};
use std::process::ExitCode;

/// Context information for diff operations to support enhanced LLM analysis
#[derive(Debug, Clone)]
struct DiffContext {
    pub old_tree_output: String,
    pub new_tree_output: String,
}

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

    // Handle config template generation and exit
    if cli_args.generate_config {
        print_default_config_template();
        return ExitCode::SUCCESS;
    }

    // 1. Map CLI args to Library config
    let lib_config = match map_cli_to_lib_config(&cli_args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    let lib_output_format = map_cli_to_lib_output_format(cli_args.format.output_format.clone());

    if cli_args.verbose {
        print_config_summary(&lib_config);
    }

    // 2. Call the library to get processed nodes using context-based APIs
    let (nodes, _actual_path) = if cli_args.input.is_from_file() {
        // Read from tree file
        let input_file = match cli_args.input.get_tree_file() {
            Some(file) => file,
            None => {
                eprintln!("Error: Input file path not available");
                return ExitCode::FAILURE;
            }
        };
        let input_format = Some(cli_args.input.get_input_format());
        match rustree::get_tree_nodes_from_source(
            &cli_args.path,
            &lib_config,
            Some(input_file),
            input_format,
        ) {
            Ok(n) => (n, input_file.to_path_buf()),
            Err(e) => {
                eprintln!("Error parsing tree file: {}", e);
                return ExitCode::FAILURE;
            }
        }
    } else {
        // Scan filesystem using optimized context-based API
        let processing_ctx = lib_config.processing_context();
        match rustree::get_tree_nodes_with_context(&cli_args.path, &processing_ctx) {
            Ok(n) => (n, cli_args.path.clone()),
            Err(e) => {
                eprintln!("Error processing directory: {}", e);
                return ExitCode::FAILURE;
            }
        }
    };

    // 2.5. Handle diff mode if requested
    let (output_string, diff_context) = if cli_args.diff.is_diff_mode() {
        if cli_args.input.is_from_file() {
            // Case: --diff <new.json> --from-tree-file <old.json>
            // Compare two snapshots: old.json (previous) vs new.json (current)
            match handle_snapshot_to_snapshot_diff(
                &cli_args,
                &lib_config,
                lib_output_format,
                &nodes,
            ) {
                Ok((output, context)) => (output, Some(context)),
                Err(exit_code) => return exit_code,
            }
        } else {
            // Case: --diff <old.json> (traditional behavior)
            // Compare old.json (previous) vs current filesystem (current)
            match handle_diff_mode(&cli_args, &lib_config, lib_output_format, &nodes) {
                Ok((output, context)) => (output, Some(context)),
                Err(exit_code) => return exit_code,
            }
        }
    } else {
        // 3. Call the library to format the nodes using context-based API
        let formatting_ctx = lib_config.formatting_context();
        let output =
            match rustree::format_nodes_with_context(&nodes, lib_output_format, &formatting_ctx) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error formatting output: {}", e);
                    return ExitCode::FAILURE;
                }
            };
        (output, None)
    };

    // 4. Handle LLM env generation first
    if cli_args.llm.llm_generate_env {
        println!(
            "{}",
            rustree::config::LlmOptions::generate_sample_env_file()
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

        match handle_llm_query(
            &cli_args,
            question,
            &output_string,
            want_json,
            diff_context.as_ref(),
        )
        .await
        {
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

/// Nicely formatted summary of the merged tree configuration (safe for logging).
fn print_config_summary(cfg: &rustree::config::RustreeLibConfig) {
    println!("\n🔧  Effective tree configuration\n────────────────────────────────────────");

    println!("Listing:");
    println!("  max_depth             : {:?}", cfg.listing.max_depth);
    println!("  show_hidden           : {}", cfg.listing.show_hidden);
    println!(
        "  list_directories_only : {}",
        cfg.listing.list_directories_only
    );
    println!("  show_full_path        : {}", cfg.listing.show_full_path);

    println!("\nFiltering:");
    println!(
        "  match_patterns        : {:?}",
        cfg.filtering.match_patterns
    );
    println!(
        "  ignore_patterns       : {:?}",
        cfg.filtering.ignore_patterns
    );
    println!(
        "  use_gitignore_rules   : {}",
        cfg.filtering.use_gitignore_rules
    );
    println!(
        "  prune_empty_directories: {}",
        cfg.filtering.prune_empty_directories
    );

    println!("\nSorting:");
    println!("  sort_by               : {:?}", cfg.sorting.sort_by);
    println!("  reverse_sort          : {}", cfg.sorting.reverse_sort);
    println!(
        "  directory_file_order  : {:?}",
        cfg.sorting.directory_file_order
    );

    println!("\nMetadata:");
    println!("  show_size_bytes       : {}", cfg.metadata.show_size_bytes);
    println!(
        "  show_last_modified    : {}",
        cfg.metadata.show_last_modified
    );
    println!(
        "  calculate_line_count  : {}",
        cfg.metadata.calculate_line_count
    );
    println!(
        "  calculate_word_count  : {}",
        cfg.metadata.calculate_word_count
    );

    println!("\nOutput:");
    // we only have text vs markdown etc from runtime flag; derive from cfg.html etc if needed.
}

/// Prints LLM configuration without leaking secrets.
fn print_llm_summary(llm: &LlmConfig) {
    println!("\n🤖  Effective LLM configuration\n────────────────────────────────────────");
    println!("  provider     : {}", llm.provider.name());
    println!("  model        : {}", llm.model);
    if let Some(ep) = &llm.endpoint {
        println!("  endpoint     : {}", ep);
    }
    println!("  temperature  : {}", llm.temperature);
    println!("  max_tokens   : {}", llm.max_tokens);
    println!("  api_key      : <redacted> (set via env var)");
}

/// Print a commented sample TOML configuration to stdout.
fn print_default_config_template() {
    const TEMPLATE: &str = r#"# RusTree configuration template (save as .rustree/config.toml)

[listing]
# show_hidden = true
# max_depth = 3

[filtering]
# match_patterns  = ["*.rs", "*.md"]
# ignore_patterns = ["target/*", "node_modules/*"]

[sorting]
# sort_by = "size"        # name | size | mtime | ctime | version | none
# reverse = true

[metadata]
# show_size_bytes      = true
# show_last_modified   = true
# calculate_line_count = true

[output]
# format     = "html"     # text | markdown | json | html
# no_summary = false

[llm]
# provider    = "openai"   # openai | anthropic | cohere | ollama
# model       = "gpt-4o"
# api_key_env = "OPENAI_API_KEY"
# temperature = 0.5
"#;

    println!("{}", TEMPLATE);
}

async fn handle_llm_query(
    cli_args: &CliArgs,
    question: &str,
    tree_output: &str,
    json_mode: bool,
    diff_context: Option<&DiffContext>,
) -> Result<String, LlmError> {
    // 1. Merge TOML-based LLM defaults into CLI args
    let merged_llm_args = {
        // Load the same config chain used earlier (explicit + project/global)
        let (partial, cfg_sources) =
            match rustree::config::load_merged_config(&cli_args.config_file, !cli_args.no_config) {
                Ok(t) => t,
                Err(e) => {
                    if cli_args.verbose {
                        eprintln!("Config load error: {e}");
                    }
                    // propagate to caller
                    return Err(LlmError::Config(e.to_string()));
                }
            };

        if cli_args.verbose {
            if !cfg_sources.is_empty() {
                println!(
                    "\n🔗  Config files used: {}",
                    cfg_sources
                        .iter()
                        .map(|p| p.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            } else {
                println!("\n🔗  Config files used: (none)");
            }
        }

        let mut args = cli_args.llm.clone();
        if let Some(llm_p) = partial.llm {
            if args.llm_provider == "openai" {
                if let Some(p) = llm_p.provider {
                    args.llm_provider = p;
                }
            }
            if args.llm_model.is_none() {
                args.llm_model = llm_p.model;
            }
            if args.llm_api_key.is_none() {
                if let Some(env_var) = llm_p.api_key_env {
                    if let Ok(val) = std::env::var(&env_var) {
                        args.llm_api_key = Some(val);
                    }
                }
                if args.llm_api_key.is_none() {
                    args.llm_api_key = llm_p.api_key;
                }
            }
            if args.llm_endpoint.is_none() {
                args.llm_endpoint = llm_p.endpoint;
            }
            if args.llm_temperature.is_none() {
                args.llm_temperature = llm_p.temperature;
            }
            if args.llm_max_tokens.is_none() {
                args.llm_max_tokens = llm_p.max_tokens;
            }
        }
        args
    };

    // 2. Create LLM config using proper CLI → Config → Core flow
    let llm_options = rustree::config::LlmOptions::from_cli_args(&merged_llm_args)
        .map_err(|e| LlmError::Config(e.to_string()))?;

    let core_llm_config = llm_options
        .to_core_config()
        .map_err(|e| LlmError::Config(e.to_string()))?;

    let llm_config = LlmConfig::new(core_llm_config);

    if cli_args.verbose && cli_args.llm.llm_ask.is_some() {
        print_llm_summary(&llm_config);
    }

    // 3. Map CLI args to library config for prompt formatting
    let lib_config = match rustree::cli::map_cli_to_lib_config(cli_args) {
        Ok(config) => config,
        Err(e) => return Err(LlmError::Config(e.to_string())),
    };

    // 4. Format prompt with tree output and question
    let prompt = if let Some(context) = diff_context {
        TreePromptFormatter::format_diff_prompt(
            tree_output,
            &context.old_tree_output,
            &context.new_tree_output,
            question,
            &lib_config,
        )
    } else {
        TreePromptFormatter::format_prompt(tree_output, question, &lib_config)
    };

    use serde_json::json;

    // 5. Handle dry-run: build preview and skip network
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

    // 6. Send to LLM and get response
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

/// Handles diff mode by comparing current nodes with a snapshot file.
fn handle_diff_mode(
    cli_args: &CliArgs,
    lib_config: &rustree::config::RustreeLibConfig,
    output_format: rustree::LibOutputFormat,
    current_nodes: &[rustree::NodeInfo],
) -> Result<(String, DiffContext), std::process::ExitCode> {
    // Get the snapshot file path
    let snapshot_file = cli_args.diff.get_diff_file().unwrap();

    // Load the snapshot nodes
    let snapshot_nodes = match rustree::get_tree_nodes_from_source(
        &cli_args.path,
        lib_config,
        Some(snapshot_file),
        Some(rustree::InputFormat::Json), // Assume JSON for now
    ) {
        Ok(nodes) => nodes,
        Err(e) => {
            eprintln!("Error loading snapshot file: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    // Create diff options
    let diff_options = map_cli_to_diff_options(cli_args, lib_config);

    // Create diff metadata
    let diff_metadata = DiffMetadata {
        generated_at: chrono::Utc::now().to_rfc3339(),
        snapshot_file: snapshot_file.clone(),
        snapshot_date: None, // TODO: Extract from snapshot file if available
        comparison_root: cli_args.path.clone(),
        filters_applied: vec![], // TODO: Extract applied filters
        options: diff_options.clone(),
    };

    // Run the diff engine
    let diff_engine = DiffEngine::new(diff_options);
    let diff_result = match diff_engine.compare(&snapshot_nodes, current_nodes, diff_metadata) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error running diff: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    // Generate tree outputs for LLM context using context-based API
    let formatting_ctx = lib_config.formatting_context();
    let old_tree_output = match rustree::format_nodes_with_context(
        &snapshot_nodes,
        rustree::LibOutputFormat::Text,
        &formatting_ctx,
    ) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error formatting old tree output: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    let new_tree_output = match rustree::format_nodes_with_context(
        current_nodes,
        rustree::LibOutputFormat::Text,
        &formatting_ctx,
    ) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error formatting new tree output: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    let diff_context = DiffContext {
        old_tree_output,
        new_tree_output,
    };

    // Format the diff result
    match format_diff(&diff_result, output_format, lib_config) {
        Ok(output) => Ok((output, diff_context)),
        Err(e) => {
            eprintln!("Error formatting diff: {}", e);
            Err(std::process::ExitCode::FAILURE)
        }
    }
}

/// Handle snapshot-to-snapshot diff mode: --diff <new.json> --from-tree-file <old.json>
fn handle_snapshot_to_snapshot_diff(
    cli_args: &CliArgs,
    lib_config: &rustree::config::RustreeLibConfig,
    output_format: rustree::LibOutputFormat,
    current_nodes: &[rustree::NodeInfo], // These are from --from-tree-file (will be "previous")
) -> Result<(String, DiffContext), std::process::ExitCode> {
    // Get the second snapshot file path from --diff
    let new_snapshot_file = cli_args.diff.get_diff_file().unwrap();

    // Load the "new" snapshot nodes from --diff argument
    let new_snapshot_nodes = match rustree::get_tree_nodes_from_source(
        &cli_args.path,
        lib_config,
        Some(new_snapshot_file),
        Some(rustree::InputFormat::Json), // Assume JSON for now
    ) {
        Ok(nodes) => nodes,
        Err(e) => {
            eprintln!("Error loading new snapshot file: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    // Create diff options
    let diff_options = map_cli_to_diff_options(cli_args, lib_config);

    // Note: old snapshot file is from --from-tree-file (already loaded in current_nodes)
    let _old_snapshot_file = cli_args.input.get_tree_file().unwrap();

    // Create diff metadata
    let diff_metadata = DiffMetadata {
        generated_at: chrono::Utc::now().to_rfc3339(),
        snapshot_file: new_snapshot_file.clone(),
        snapshot_date: None, // TODO: Extract from new snapshot file if available
        comparison_root: cli_args.path.clone(),
        filters_applied: vec![], // TODO: Extract applied filters
        options: diff_options.clone(),
    };

    // Run the diff engine: compare old (previous) vs new (current)
    let diff_engine = DiffEngine::new(diff_options);
    let diff_result = match diff_engine.compare(current_nodes, &new_snapshot_nodes, diff_metadata) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error running snapshot-to-snapshot diff: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    // Generate tree outputs for LLM context using context-based API
    let formatting_ctx = lib_config.formatting_context();
    let old_tree_output = match rustree::format_nodes_with_context(
        current_nodes,
        rustree::LibOutputFormat::Text,
        &formatting_ctx,
    ) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error formatting old tree output: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    let new_tree_output = match rustree::format_nodes_with_context(
        &new_snapshot_nodes,
        rustree::LibOutputFormat::Text,
        &formatting_ctx,
    ) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error formatting new tree output: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    let diff_context = DiffContext {
        old_tree_output,
        new_tree_output,
    };

    // Format the diff result
    match format_diff(&diff_result, output_format, lib_config) {
        Ok(output) => Ok((output, diff_context)),
        Err(e) => {
            eprintln!("Error formatting snapshot-to-snapshot diff: {}", e);
            Err(std::process::ExitCode::FAILURE)
        }
    }
}
