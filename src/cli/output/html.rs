// src/cli/output/html.rs

//! CLI flags that are specific to the HTML output formatter.
//!
//! They are grouped under the same "Output Options" heading as the generic
//! `--output-format` flag.

use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone, Default)]
pub struct HtmlOutputArgs {
    /// Base URL to prepend to generated links (equivalent to GNU tree's -H).
    #[arg(
        long = "html-base-href",
        value_name = "URL",
        help_heading = "HTML Options"
    )]
    pub html_base_href: Option<String>,

    /// Strip the first path component from generated hyperlinks (mimics `-H -BASEHREF`).
    #[arg(long = "html-strip-first-component", help_heading = "HTML Options")]
    pub html_strip_first_component: bool,

    /// Path to a file that contains HTML to be placed *before* the <pre> block.
    #[arg(
        long = "html-intro-file",
        value_name = "FILE",
        help_heading = "HTML Options"
    )]
    pub html_intro_file: Option<PathBuf>,

    /// Path to a file that contains HTML to be placed *after* the <pre> block.
    #[arg(
        long = "html-outro-file",
        value_name = "FILE",
        help_heading = "HTML Options"
    )]
    pub html_outro_file: Option<PathBuf>,

    /// Disable generation of <a href> hyperlinks inside the HTML tree.
    #[arg(long = "html-no-links", help_heading = "HTML Options")]
    pub html_no_links: bool,
}

// Default derive now covers the previous manual implementation.
