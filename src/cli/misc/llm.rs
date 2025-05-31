// src/cli/misc/llm.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct LlmArgs {
    /// Ask a question to an LLM, providing the `rustree` output as context.
    /// The output will be specially formatted for easy piping to an LLM tool.
    #[arg(long)]
    pub llm_ask: Option<String>,
}
