// src/core/formatter/base.rs
use crate::core::node::NodeInfo;
use crate::core::config::RustreeLibConfig;
use crate::core::error::RustreeError;

pub trait TreeFormatter {
    fn format(&self, nodes: &[NodeInfo], config: &RustreeLibConfig) -> Result<String, RustreeError>;
}