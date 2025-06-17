# Getting Started with RusTree CLI

This page covers the most basic and commonly used RusTree commands to get you started quickly.

## Basic Usage

The simplest way to use RusTree is just to run it without any arguments:

```bash
rustree
```

This displays a tree of the current directory with default settings.

## Essential Options

### Specify a Directory

Point RusTree at any directory:

```bash
rustree /path/to/directory
rustree ~/Documents
rustree ./src
```

### Control Tree Depth

Limit how deep into subdirectories RusTree scans:

```bash
# Show only the root and direct children (depth 1)
rustree -L 1

# Show up to 3 levels deep
rustree --depth 3 ./my_project
```

### Show Hidden Files

Include files and directories that start with a dot:

```bash
# Show hidden files
rustree -a

# Long form
rustree --include-hidden
```

### Display File Sizes

Show the size of each file:

```bash
# Show sizes in bytes
rustree -s

# Long form
rustree --show-size-bytes
```

### Show Modification Times

Display when files were last modified:

```bash
# Show modification times
rustree -D

# Long form  
rustree --show-last-modified
```

## Common Combinations

### Explore a Project Structure

```bash
# Get an overview of a project (2 levels deep, with sizes)
rustree -L 2 -s ./my_project
```

### Analyze Source Code

```bash
# Look at source files with metadata
rustree -a -s -D --depth 3 ./src
```

### Quick Directory Overview

```bash
# See directory structure only (no files)
rustree -d -L 2 ./workspace
```

## Simple Filtering

### Show Only Specific File Types

```bash
# Show only Rust files
rustree -P "*.rs"

# Show only documentation files
rustree -P "*.md|*.txt"
```

### Hide Specific Files or Directories

```bash
# Hide log files
rustree -I "*.log"

# Hide build directories
rustree -I "target/|build/"
```

## Getting Help

- Use `rustree --help` for a complete list of options
- Use `rustree -h` for a shorter summary
- Check `rustree --version` to see your version

## Next Steps

Once you're comfortable with these basics, explore more advanced features:

- **[Listing and Traversal](./listing_and_traversal.md)** - Advanced tree structure control
- **[Filtering and Patterns](./filtering_and_patterns.md)** - Powerful include/exclude capabilities  
- **[Metadata and Analysis](./metadata_and_analysis.md)** - File analysis and information display
- **[Examples](./examples.md)** - Comprehensive examples for all features

## Quick Reference

| Option | Short | Description |
|--------|-------|-------------|
| `--depth <N>` | `-L <N>` | Limit tree depth to N levels |
| `--include-hidden` | `-a` | Show hidden files/directories |
| `--show-size-bytes` | `-s` | Display file sizes |
| `--show-last-modified` | `-D` | Show modification times |
| `--directory-only` | `-d` | Show directories only |
| `--filter-include <PATTERN>` | `-P <PATTERN>` | Include only matching files |
| `--filter-exclude <PATTERN>` | `-I <PATTERN>` | Exclude matching files |
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Show version information |