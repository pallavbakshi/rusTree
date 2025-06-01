//! Filtering and pattern matching functionality.
//!
//! This module contains logic for filtering file system entries based on various
//! criteria including glob patterns, gitignore rules, and other filtering mechanisms.

pub mod composite;
pub mod gitignore;
pub mod matcher;
pub mod pattern;
pub mod size_filter;
