# Command-Line Interface (CLI)

This section provides comprehensive documentation for the `rustree` command-line tool, organized by functional areas for easy navigation.

## Basic Syntax

```bash
rustree [OPTIONS] [PATH]
```

- `[OPTIONS]`: Various flags to control behavior (e.g., depth, sorting, output format, apply functions).
- `[PATH]`: Optional path to the directory to scan. Defaults to the current directory (`.`).

## Key Features

- **Tree visualization** with customizable depth and formatting
- **Metadata display** including sizes, timestamps, line/word counts
- **Enhanced summary reports** with automatic aggregation of metadata totals
- **Apply functions** to analyze file contents and directory statistics
- **Flexible filtering** with patterns, gitignore support, and function-specific filtering
- **Advanced sorting** by various criteria including custom function output
- **Multiple output formats** (text, markdown, JSON, HTML) for different use cases
- **LLM integration** for AI-powered project analysis
- **Directory comparison** and change tracking
- **Configuration file support** for persistent settings

## Documentation Organization

The CLI documentation is organized into the following sections:

### Core Usage
- **[Getting Started](./cli_usage/getting_started.md)** - Basic commands and essential options for new users
- **[Configuration](./cli_usage/configuration.md)** - Config files, shell completions, and persistent settings

### Tree Control
- **[Listing and Traversal](./cli_usage/listing_and_traversal.md)** - Control tree depth, hidden files, and path display
- **[Filtering and Patterns](./cli_usage/filtering_and_patterns.md)** - Include/exclude patterns, gitignore integration, size filtering

### Analysis and Display
- **[Metadata and Analysis](./cli_usage/metadata_and_analysis.md)** - File sizes, timestamps, line/word counts, and content analysis
- **[Apply Functions](./cli_usage/apply_functions.md)** - Built-in and external functions for file and directory analysis
- **[Sorting and Ordering](./cli_usage/sorting_and_ordering.md)** - Sort by various criteria and control output organization

### Output and Integration
- **[Output Formats](./cli_usage/output_formats.md)** - Text, Markdown, JSON, HTML formats and customization
- **[LLM Integration](./cli_usage/llm_integration.md)** - AI-powered analysis with various LLM providers
- **[Diff and Comparison](./cli_usage/diff_and_comparison.md)** - Directory comparison and change tracking

### Reference
- **[Options and Flags](./cli_usage/options.md)** - Comprehensive reference of all available command-line options
- **[Examples](./cli_usage/examples.md)** - Practical examples demonstrating various use cases

## Enhanced Summary Reports

When using metadata flags like `--calculate-lines`, `--calculate-words`, or `--show-size-bytes`, the summary line automatically includes aggregated totals:

```bash
# Instead of just: "3 directories, 15 files"
# You now get: "3 directories, 15 files, 1,234 total lines, 5,678 total words, 2.1 MB total"
```

This feature works with:
- Line counts (`--calculate-lines`)
- Word counts (`--calculate-words`)
- File sizes (`--show-size-bytes`)
- Apply function outputs (when numeric)
- Large numbers are formatted with thousand separators for readability

## Quick Help

To see all available options directly from your terminal:

```bash
rustree --help        # Full help with all options
rustree -h            # Short help summary
rustree --version     # Show version information
```

## Getting Started

If you're new to RusTree, start with the [Getting Started](./cli_usage/getting_started.md) guide for the most commonly used commands and options.
