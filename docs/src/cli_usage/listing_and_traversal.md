# Listing and Traversal

This page covers options that control how RusTree traverses directories and what elements of the tree structure it displays.

## Tree Depth Control

### Maximum Depth

Control how deep RusTree descends into subdirectories:

```bash
# Show only direct children (depth 1)
rustree -L 1

# Scan up to 3 levels deep
rustree --depth 3 ./project

# No depth limit (default behavior)
rustree ./project
```

**Examples:**

```bash
# Quick project overview
rustree -L 2 ~/my_project

# Detailed but limited scan
rustree --depth 4 --show-size-bytes ./src
```

## Hidden Files and Directories

### Including Hidden Files

By default, RusTree ignores files and directories starting with a dot (`.`). Use `-a` to include them:

```bash
# Show hidden files and directories
rustree -a

# Long form
rustree --include-hidden

# Combine with other options
rustree -a -L 2 -s
```

**Important:** When using pattern matching (`-P`), you need `-a` for patterns like `*` to match hidden files. Patterns that explicitly start with `.` (like `.*`) will match hidden files regardless.

```bash
# This WON'T match .hidden_file
rustree -P "*"

# This WILL match .hidden_file  
rustree -a -P "*"

# This WILL match .hidden_file (explicit dot pattern)
rustree -P ".*"
```

## Directory vs File Display

### Directory-Only Mode

Show only directories, excluding all files:

```bash
# Show directory structure only
rustree -d

# Long form
rustree --directory-only

# Combine with depth and metadata
rustree -d -L 3 -s ./workspace
```

This is useful for understanding the overall organization of large projects without being overwhelmed by individual files.

### Full Path Display

Show the complete relative path for each entry:

```bash
# Display full paths
rustree -f

# Long form  
rustree --full-path
```

**Example output:**
```
my_project/
├── README.md
├── src/
│   ├── src/main.rs        # Shows full path from root
│   ├── src/lib.rs
│   └── src/utils/
│       └── src/utils/helper.rs
└── tests/
    └── tests/integration_test.rs
```

This is particularly useful when you need to know exact file locations, especially for documentation or when working with tools that need full paths.

## Path Arguments

### Scanning Multiple Paths

RusTree can scan a specific directory by providing it as an argument:

```bash
# Scan a specific directory
rustree ~/Documents

# Scan relative path
rustree ./src

# Scan with options
rustree -a -L 2 /etc/nginx
```

### Current Directory

When no path is specified, RusTree scans the current directory:

```bash
# These are equivalent
rustree
rustree .
rustree ./
```

## Combining Traversal Options

### Common Combinations

```bash
# Project structure overview
rustree -d -L 2 --full-path ./my_project

# Hidden configuration files
rustree -a -P ".*" /etc

# Detailed source code structure  
rustree -f --include-hidden --depth 4 ./src

# Quick directory tree with sizes
rustree -d -s -L 3 ~/workspace
```

### Performance Considerations

For large directories, consider:

- Using `--depth` to limit traversal depth
- Using `--directory-only` to skip files when analyzing structure
- Combining with [filtering](./filtering_and_patterns.md) to focus on relevant content

## Integration with Other Features

Traversal options work seamlessly with other RusTree features:

- **[Filtering](./filtering_and_patterns.md)**: Control which files are included based on patterns
- **[Metadata](./metadata_and_analysis.md)**: Add file information to the traversal output
- **[Sorting](./sorting_and_ordering.md)**: Control how entries are ordered in the tree
- **[Output Formats](./output_formats.md)**: Change how the traversal results are displayed

## Quick Reference

| Option | Short | Description |
|--------|-------|-------------|
| `--depth <N>` | `-L <N>` | Limit traversal to N levels deep |
| `--include-hidden` | `-a` | Include hidden files/directories (starting with `.`) |
| `--directory-only` | `-d` | Show directories only, exclude files |
| `--full-path` | `-f` | Display complete relative paths for all entries |

## Examples

See the [Examples](./examples.md) page for more detailed usage scenarios and combinations with other features.