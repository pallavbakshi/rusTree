// src/core/node.rs
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub path: PathBuf,
    pub name: String, // Added field
    pub node_type: NodeType,
    pub depth: usize,
    pub size: Option<u64>,
    pub permissions: Option<String>, // Or a more structured type
    pub mtime: Option<SystemTime>,
    pub line_count: Option<usize>,
    pub word_count: Option<usize>,
    pub custom_function_output: Option<Result<String, crate::core::analyzer::apply_fn::ApplyFnError>>, // Updated type
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
    Symlink,
}