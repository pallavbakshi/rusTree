// src/core/formatter/html.rs
//
// Basic HTML formatter for RusTree.  It intentionally keeps the output nearly
// identical to the plain-text tree produced by `TextTreeFormatter`, but wraps
// it in minimal HTML so it can be viewed in a browser or embedded in other
// documents.  Future enhancements (hyperlinks, CSS theming, collapsible
// sections, etc.) can build on top of this foundation without affecting the
// public interface of `TreeFormatter`.

use super::base::{TreeFormatter, TreeFormatterCompat};
use super::text_tree::TextTreeFormatter;

use crate::core::error::RustreeError;
use crate::core::options::HtmlOptions;
use crate::core::options::contexts::FormattingContext;
use crate::core::tree::node::NodeInfo;

/// Formatter producing an HTML page that contains the directory tree wrapped
/// in a `<pre>` element.  Characters are HTML-escaped so the ASCII art is
/// preserved.
pub struct HtmlFormatter;

impl TreeFormatter for HtmlFormatter {
    fn format(
        &self,
        nodes: &[NodeInfo],
        formatting_ctx: &FormattingContext,
    ) -> Result<String, RustreeError> {
        let html_opts: &HtmlOptions = formatting_ctx.html;

        // 1. Obtain the lines produced by the text formatter so we can reuse
        //    its indentation logic.  We will post-process each line to turn
        //    the file name portion into a hyperlink (unless links are
        //    disabled).
        let plain_output = TextTreeFormatter.format(nodes, formatting_ctx)?;
        let mut lines: Vec<String> = plain_output.lines().map(|s| s.to_string()).collect();

        // Build a path representing the scan root (same technique as text formatter)
        let scan_root_path_opt = nodes
            .iter()
            .find(|n| n.depth == 1)
            .and_then(|n| n.path.parent().map(|p| p.to_path_buf()));

        if html_opts.include_links {
            for (idx, line) in lines.iter_mut().enumerate() {
                // Skip the root line (idx == 0).  Nodes vector aligns with
                // lines[1..]
                if idx == 0 {
                    continue;
                }

                if idx > nodes.len() {
                    // Defensive: malformed line mapping (shouldn't happen but
                    // occurs when the nodes vector is empty – e.g. test with
                    // only root).
                    continue;
                }

                let node = &nodes[idx - 1];

                // Determine relative path for href
                let mut rel_path = if let Some(scan_root) = &scan_root_path_opt {
                    node.path
                        .strip_prefix(scan_root)
                        .unwrap_or(&node.path)
                        .to_path_buf()
                } else {
                    node.path.clone()
                };

                if html_opts.strip_first_component {
                    let _ = rel_path.iter().next(); // consumed
                    rel_path = rel_path.iter().skip(1).collect::<std::path::PathBuf>();
                }

                // Build href string
                let href = {
                    let rel_str = rel_path.to_string_lossy().replace("\\", "/"); // Windows backslash → slash
                    if let Some(prefix) = &html_opts.base_href {
                        if prefix.ends_with('/') {
                            format!("{}{}", prefix, rel_str)
                        } else {
                            format!("{}/{}", prefix, rel_str)
                        }
                    } else {
                        rel_str
                    }
                };

                // Determine visible label (same logic as text formatter)
                let mut label = if formatting_ctx.listing.show_full_path {
                    rel_path.to_string_lossy().to_string()
                } else {
                    node.name.clone()
                };
                if node.node_type == crate::core::tree::node::NodeType::Directory {
                    label.push('/');
                }

                // HTML-escape label text
                let escaped_label = html_escape(&label);

                let anchor = format!("<a href=\"{}\">{}</a>", html_escape(&href), escaped_label);

                // Replace last occurrence of the label in the line with the anchor.
                if let Some(pos) = line.rfind(&label) {
                    line.replace_range(pos..pos + label.len(), &anchor);
                }
            }
        } else {
            // No links → escape the entire lines (incl. names)
            for line in &mut lines {
                *line = html_escape(line);
            }
        }

        // If links are included we have already escaped prefix parts except the
        // anchor tag itself.  We need to ensure the *non-anchor* parts are
        // escaped:
        if html_opts.include_links {
            for (idx, line) in lines.iter_mut().enumerate() {
                // Skip root line? root line has no anchor maybe; escape fully.
                if idx == 0 {
                    *line = html_escape(line);
                    continue;
                }
                // Split by anchor tag and escape outside parts
                let mut result = String::new();
                let mut remaining = line.as_str();
                while let Some(start) = remaining.find("<a ") {
                    let (before, rest) = remaining.split_at(start);
                    result.push_str(&html_escape(before));
                    if let Some(end) = rest.find("</a>") {
                        let (anchor_part, tail) = rest.split_at(end + 4);
                        result.push_str(anchor_part); // already safe
                        remaining = tail;
                    } else {
                        // malformed, escape everything
                        result.push_str(&html_escape(rest));
                        remaining = "";
                    }
                }
                result.push_str(&html_escape(remaining));
                *line = result;
            }
        }

        // Join lines with newline
        let escaped_body = lines.join("\n");

        // Build intro/outro — propagate I/O errors so users notice bad paths.
        let intro = match &html_opts.custom_intro {
            Some(path) => std::fs::read_to_string(path)?,
            None => default_intro(formatting_ctx),
        };

        let outro = match &html_opts.custom_outro {
            Some(path) => std::fs::read_to_string(path)?,
            None => default_outro(),
        };

        let html_page = format!("{}<pre>{}</pre>{}", intro, escaped_body, outro);
        Ok(html_page)
    }
}

/// Implement backward compatibility trait
impl TreeFormatterCompat for HtmlFormatter {}

fn html_escape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

fn default_intro(formatting_ctx: &FormattingContext) -> String {
    let raw_title = if !formatting_ctx.input_source.root_display_name.is_empty() {
        &formatting_ctx.input_source.root_display_name
    } else {
        "Directory Tree"
    };

    let safe_title = html_escape(raw_title);

    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"utf-8\">\n  <title>{safe_title}</title>\n  <style>body{{font-family:monospace;}}</style>\n</head>\n<body>\n"
    )
}

fn default_outro() -> String {
    "\n</body>\n</html>\n".to_string()
}

// --------------------------------------------------
// Tests
// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RustreeLibConfig;
    use crate::core::tree::node::{NodeInfo, NodeType};
    use std::path::PathBuf;

    #[test]
    fn basic_html_contains_expected_wrappers() {
        let nodes = vec![NodeInfo {
            path: PathBuf::from("root"),
            name: "root".into(),
            node_type: NodeType::Directory,
            depth: 0,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        }];

        let cfg = RustreeLibConfig::default();
        let html = HtmlFormatter.format_compat(&nodes, &cfg).unwrap();

        // Basic sanity checks
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<pre>"));
        assert!(html.ends_with("</html>\n"));
    }

    #[test]
    fn special_chars_are_escaped() {
        let nodes = vec![NodeInfo {
            path: PathBuf::from("root/file<1>&.txt"),
            name: "file<1>&.txt".into(),
            node_type: NodeType::File,
            depth: 1,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        }];

        let cfg = RustreeLibConfig::default();
        let html = HtmlFormatter.format_compat(&nodes, &cfg).unwrap();

        // The raw characters should be escaped inside the pre block.
        assert!(html.contains("&lt;1&gt;&amp;"));
    }

    #[test]
    fn base_href_and_links_work() {
        use crate::core::options::HtmlOptions;
        use std::path::PathBuf;

        let nodes = vec![NodeInfo {
            path: PathBuf::from("root/sub/file.txt"),
            name: "file.txt".into(),
            node_type: NodeType::File,
            depth: 2,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        }];

        let cfg = RustreeLibConfig {
            html: HtmlOptions {
                base_href: Some("https://example.com".into()),
                include_links: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let html = HtmlFormatter.format_compat(&nodes, &cfg).unwrap();
        assert!(html.contains("https://example.com/root/sub/file.txt"));
    }

    #[test]
    fn no_links_flag_works() {
        use std::path::PathBuf;

        let nodes = vec![NodeInfo {
            path: PathBuf::from("root/alpha.txt"),
            name: "alpha.txt".into(),
            node_type: NodeType::File,
            depth: 1,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        }];

        let cfg = RustreeLibConfig {
            html: HtmlOptions {
                include_links: false,
                ..Default::default()
            },
            ..Default::default()
        };

        let html = HtmlFormatter.format_compat(&nodes, &cfg).unwrap();
        assert!(!html.contains("<a href="));
        assert!(html.contains("alpha.txt"));
    }
}
