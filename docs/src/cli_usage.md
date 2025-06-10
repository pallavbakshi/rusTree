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
- **Apply functions** to analyze file contents and directory statistics
- **Flexible filtering** with patterns, gitignore support, and function-specific filtering
- **Advanced sorting** by various criteria including custom function output
- **Multiple output formats** (text, markdown) for different use cases

## Sub-sections:

- [Options and Flags](./cli_usage/options.md): A comprehensive list of all available command-line options.
- [Examples](./cli_usage/examples.md): Practical examples demonstrating various use cases.

To see all available options directly from your terminal, run:

```bash
rustree --help
```
