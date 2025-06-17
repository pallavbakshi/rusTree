# Apply Functions

Apply functions allow you to execute operations on files and directories during the tree traversal. RusTree includes powerful built-in functions for analysis and supports external commands for custom processing.

## Built-In Functions

### File Content Functions

These functions operate on individual file contents:

#### Cat Function

Display the complete contents of files after the tree structure:

```bash
# Show tree structure, then file contents
rustree --apply-function cat

# Show contents of specific file types only
rustree --apply-function cat -P "*.md|*.txt"
```

**Output format:**
```
project/
├── README.md
├── config.toml
└── src/
    └── main.rs

2 directories, 3 files

--- File Contents ---

=== README.md ===
# My Project
This is a sample project...

=== config.toml ===
[package]
name = "my-project"
version = "0.1.0"

=== src/main.rs ===
fn main() {
    println!("Hello, world!");
}
```

#### Count Pluses Function

Count occurrences of the '+' character in each file:

```bash
# Count '+' characters in files
rustree --apply-function count-pluses

# Focus on specific file types
rustree --apply-function count-pluses -P "*.rs|*.js"
```

**Output format:**
```
project/
├── [+: 5] README.md
├── [+: 0] config.toml
└── src/
    └── [+: 12] main.rs
```

### Directory Analysis Functions

These functions analyze directory contents:

#### Directory Statistics

Get comprehensive statistics for each directory:

```bash
# Show files, subdirs, and total size for each directory
rustree --apply-function dir-stats --show-size-bytes
```

**Output format:**
```
project/
├── [F: "2f,1d,1024B"] src/
│   └── main.rs
└── [F: "1f,0d,512B"] docs/
    └── README.md
```

Format: `[F: "XFiles,YDirs,ZBytes"]`

#### Count Files

Count the number of files in each directory:

```bash
# Count files in each directory
rustree --apply-function count-files
```

**Output format:**
```
project/
├── [F: "3"] src/
└── [F: "1"] docs/
```

#### Count Directories

Count the number of subdirectories in each directory:

```bash
# Count subdirectories  
rustree --apply-function count-dirs
```

#### Size Total

Calculate total size of all files in each directory:

```bash
# Calculate total size (requires --show-size-bytes)
rustree --apply-function size-total --show-size-bytes
```

**Note:** `--show-size-bytes` must be enabled for size calculations to work.

## External Commands

### Custom Command Execution

Execute any external command on file contents:

```bash
# Run wc -l on each file (count lines)
rustree --apply-function-cmd "wc -l"

# Run grep to count pattern matches
rustree --apply-function-cmd "grep -c 'TODO'"

# Run custom analysis script
rustree --apply-function-cmd "./analyze-file.sh"
```

### Command Result Types

Specify how to interpret command output:

```bash
# Treat output as text (default)
rustree --apply-function-cmd "file" --apply-function-cmd-kind text

# Treat output as a number  
rustree --apply-function-cmd "wc -l" --apply-function-cmd-kind number

# Treat output as raw bytes
rustree --apply-function-cmd "md5sum" --apply-function-cmd-kind bytes
```

### Command Timeout

Set timeout for external commands:

```bash
# Set 10-second timeout
rustree --apply-function-cmd "slow-analysis" --apply-timeout 10

# Quick timeout for fast commands
rustree --apply-function-cmd "grep -c pattern" --apply-timeout 2
```

Default timeout is 5 seconds.

## Function-Specific Filtering

### Include Patterns

Apply functions only to files/directories matching patterns:

```bash
# Apply cat only to configuration files
rustree --apply-function cat --apply-include "*.toml|*.yml|*.json"

# Count pluses only in source code
rustree --apply-function count-pluses --apply-include "*.rs|*.js|*.py"

# Directory stats for source directories only  
rustree --apply-function dir-stats --apply-include "src*|lib*"
```

### Exclude Patterns

Skip applying functions to matching files/directories:

```bash
# Apply cat but exclude temporary files
rustree --apply-function cat --apply-exclude "*.tmp|*.log"

# Directory stats excluding build directories
rustree --apply-function dir-stats --apply-exclude "target/*|build/*"
```

### Pattern Files

Use files to specify include/exclude patterns:

Create `apply-include.txt`:
```
# Include source files
*.rs
*.js
*.py

# Include configs  
*.toml
*.yml
```

Create `apply-exclude.txt`:
```
# Exclude tests
*test*
*Test*

# Exclude temp files
*.tmp
*.bak
```

Use the files:
```bash
rustree --apply-function cat \
        --apply-include-from ./apply-include.txt \
        --apply-exclude-from ./apply-exclude.txt
```

## Advanced Usage

### Combining with Other Features

Apply functions work seamlessly with other RusTree features:

```bash
# Apply function with filtering and sorting
rustree --apply-function count-pluses \
        -P "*.rs" \
        --sort-by custom -r \
        --depth 3

# External command with metadata
rustree --apply-function-cmd "wc -l" \
        --apply-function-cmd-kind number \
        --show-size-bytes \
        --sort-by custom

# Directory analysis with full metadata
rustree --apply-function dir-stats \
        --show-size-bytes \
        --show-last-modified \
        --sort-by custom -r
```

### Performance Optimization

For large projects, optimize apply function usage:

```bash
# Limit depth to improve performance
rustree --apply-function cat --depth 2

# Use specific filtering to reduce scope
rustree --apply-function-cmd "analyze" \
        --apply-include "*.rs" \
        --apply-exclude "**/target/**"

# Set shorter timeout for fast commands
rustree --apply-function-cmd "grep -c pattern" \
        --apply-timeout 1
```

## Sorting by Function Output

Sort the tree by apply function results:

```bash
# Sort by count-pluses output (highest first)
rustree --apply-function count-pluses --sort-by custom -r

# Sort by directory statistics  
rustree --apply-function dir-stats --sort-by custom

# Sort by external command output
rustree --apply-function-cmd "wc -l" --sort-by custom -r
```

See [Sorting and Ordering](./sorting_and_ordering.md) for more details.

## Real-World Examples

### Code Analysis

```bash
# Find files with the most TODO comments
rustree --apply-function-cmd "grep -c 'TODO'" \
        --apply-function-cmd-kind number \
        -P "*.rs|*.js|*.py" \
        --sort-by custom -r

# Analyze complexity by line count
rustree --apply-function-cmd "wc -l" \
        --apply-function-cmd-kind number \
        --apply-include "*.rs" \
        --sort-by custom -r
```

### Configuration Management

```bash
# Display all configuration files
rustree --apply-function cat \
        --apply-include "*.toml|*.yml|*.json|*.ini" \
        --depth 2

# Find large config files
rustree --apply-function-cmd "wc -c" \
        --apply-function-cmd-kind number \
        --apply-include "*.config|*.conf" \
        --sort-by custom -r
```

### Project Statistics

```bash
# Comprehensive directory analysis
rustree --apply-function dir-stats \
        --show-size-bytes \
        --apply-exclude "**/target/**" \
        --apply-exclude "**/node_modules/**" \
        --sort-by custom -r

# Find directories with most files
rustree --apply-function count-files \
        --directory-only \
        --sort-by custom -r \
        --depth 3
```

### Documentation Review

```bash
# Review all markdown documentation
rustree --apply-function cat \
        --apply-include "*.md|README*" \
        --depth 3

# Find largest documentation files
rustree --apply-function-cmd "wc -w" \
        --apply-function-cmd-kind number \
        --apply-include "*.md" \
        --sort-by custom -r
```

## Error Handling

### Command Failures

When external commands fail:
- Error output is captured and displayed
- Tree generation continues for other files
- Failed commands show error message in output

### Timeout Handling

When commands exceed timeout:
- Command is terminated
- Timeout message is displayed
- Processing continues with remaining files

### File Access Issues

When files can't be read:
- Built-in functions skip unreadable files
- External commands receive empty input
- Warning messages may be displayed

## Quick Reference

| Option | Description |
|--------|-------------|
| `--apply-function <FUNCTION>` | Apply built-in function (cat, count-pluses, dir-stats, etc.) |
| `--apply-function-cmd <CMD>` | Apply external command to files |
| `--apply-function-cmd-kind <KIND>` | Command output type (text, number, bytes) |
| `--apply-timeout <SECONDS>` | Timeout for external commands (default: 5) |
| `--apply-include <PATTERN>` | Apply function only to matching files/dirs |
| `--apply-exclude <PATTERN>` | Skip function for matching files/dirs |
| `--apply-include-from <FILE>` | Read include patterns from file |
| `--apply-exclude-from <FILE>` | Read exclude patterns from file |

### Built-in Functions

| Function | Type | Description |
|----------|------|-------------|
| `cat` | File | Display complete file contents |
| `count-pluses` | File | Count '+' characters in files |
| `count-files` | Directory | Count files in directories |
| `count-dirs` | Directory | Count subdirectories |
| `size-total` | Directory | Calculate total size (needs `--show-size-bytes`) |
| `dir-stats` | Directory | Combined stats (files, dirs, size) |

## Examples

See the [Examples](./examples.md) page for more detailed apply function scenarios and real-world use cases.