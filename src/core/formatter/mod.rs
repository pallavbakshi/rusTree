// src/core/formatter/mod.rs
pub mod base;
pub mod text_tree;
pub mod markdown;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Markdown,
}