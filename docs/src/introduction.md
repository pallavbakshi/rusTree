# Introduction to RusTree

Welcome to RusTree!

RusTree is a command-line tool and Rust library designed to display directory structures in a tree-like format, similar to the classic `tree` command, but with enhanced features for analysis and output customization.

## What can RusTree do?

- **Visualize Directory Structures:** Clearly see the hierarchy of files and folders.
- **Filter and Ignore Entries:**
  - List only files/directories matching specific wildcard patterns (`-P` or `--filter-include`).
  - Exclude files/directories using glob patterns (`-I` or `--filter-exclude`).
  - Respect `.gitignore` files (`--use-gitignore-rules`) and custom ignore files (`--gitignore-file`).
  - Perform case-insensitive pattern matching (`--case-insensitive-filter`).
  - Prune empty directories from the output (`--prune-empty-directories`).
- **Analyze Content:** Get insights like file sizes, modification dates, line counts, and word counts.
- **Apply Custom Logic:** Use built-in functions (or extend with your own) to process file contents and report results.
- **Sort Entries:** Organize the tree output by name, size, modification time, or other criteria.
- **Flexible Output:** Choose between plain text tree format, Markdown, JSON, or a self-contained HTML page.
- **Cross-Platform:** Built with Rust, aiming for compatibility across different operating systems.

## Who is this for?

- **Developers:** Who need to quickly understand the layout of a project or directory.
- **System Administrators:** For inspecting directory contents and sizes.
- **Anyone working with files:** Who wants a more powerful alternative to basic `ls` or `dir` commands.
- **Rustaceans:** Who want to use or contribute to a Rust-based utility.

This documentation will guide you through installing RusTree, using its command-line interface, and leveraging its capabilities as a Rust library in your own projects.