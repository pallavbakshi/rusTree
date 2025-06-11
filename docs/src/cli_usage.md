# Command-Line Interface (CLI)

This section details the usage of the `rustree` command-line tool.

The basic syntax is:

```bash
rustree [OPTIONS] [PATH]
```

- `[OPTIONS]`: Various flags to control behavior (e.g., depth, sorting, output format, apply functions).
- `[PATH]`: Optional path to the directory to scan. Defaults to the current directory (`.`).

## Key Features

- **Tree visualization** with customizable depth and formatting
- **Metadata display** including sizes, timestamps, line/word counts
- **Enhanced summary report** with automatic aggregation of metadata totals
- **Apply functions** to analyze file contents and directory statistics
- **Flexible filtering** with patterns, gitignore support, and function-specific filtering
- **Advanced sorting** by various criteria including custom function output
- **Multiple output formats** (text, markdown) for different use cases

### Enhanced Summary Reports

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

## Sub-sections:

- [Options and Flags](./cli_usage/options.md): A comprehensive list of all available command-line options.
- [Examples](./cli_usage/examples.md): Practical examples demonstrating various use cases.

To see all available options directly from your terminal, run:

```bash
rustree --help
```
