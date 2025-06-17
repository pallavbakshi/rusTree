# Metadata and Analysis

RusTree can collect and display various types of metadata about files and directories, from basic size and timestamp information to detailed content analysis. This page covers all metadata and analysis options.

## File Size Information

### Basic Size Display

Show file sizes alongside the tree structure:

```bash
# Show sizes in bytes
rustree -s

# Long form
rustree --show-size-bytes
```

### Human-Readable Sizes

Format sizes in a more readable format:

```bash
# Show sizes in human-readable format (1.2 MB instead of 1234567B)
rustree -s --human-friendly

# Combine with other metadata
rustree -s --human-friendly -D --calculate-lines
```

The `--human-friendly` flag converts raw byte counts to readable formats like:
- `1.2 KB` instead of `1234B`
- `3.4 MB` instead of `3456789B`  
- `2.1 GB` instead of `2147483648B`

## Timestamp Information

### Modification Times

Display when files were last modified:

```bash
# Show modification times
rustree -D

# Long form
rustree --show-last-modified
```

### Change Times

When combined with change time sorting, `-D` shows change times instead:

```bash
# Sort by change time and show change times
rustree -c -D

# Or using long form
rustree --sort-by change_time --show-last-modified
```

### Creation Times

Show file creation times (where supported by the filesystem):

```bash
# Show creation times (may not be available on all systems)
rustree --show-creation-time
```

**Note:** Creation time support varies by operating system and filesystem. It's most reliable on Windows and macOS, less so on Linux.

## Content Analysis

### Line Counting

Count the number of lines in text files:

```bash
# Calculate and display line counts
rustree --calculate-lines

# Combine with other analysis
rustree --calculate-lines --show-size-bytes
```

### Word Counting

Count words in text files:

```bash
# Calculate and display word counts  
rustree --calculate-words

# Combine with line counting
rustree --calculate-lines --calculate-words
```

### Content Analysis with Filtering

Content analysis works with any file RusTree can read as text:

```bash
# Analyze only source code files
rustree --calculate-lines --calculate-words -P "*.rs|*.js|*.py"

# Analyze documentation
rustree --calculate-lines --calculate-words -P "*.md|*.txt"
```

## Enhanced Summary Reports

When using metadata options, RusTree automatically aggregates totals in the summary line:

```bash
# This shows enhanced summary with totals
rustree --calculate-lines --calculate-words --show-size-bytes
```

Example output:
```
my_project/
├── [   1024B] [L:  50] [W: 250] README.md
├── [   2048B] [L: 100] [W: 500] main.rs
└── src/
    ├── [   3072B] [L: 150] [W: 750] lib.rs
    └── [   1536B] [L:  75] [W: 375] util.rs

2 directories, 4 files, 375 total lines, 1,875 total words, 7.7 KB total
```

The summary automatically includes:
- Total line counts (when `--calculate-lines` is used)
- Total word counts (when `--calculate-words` is used)  
- Total file sizes (when `--show-size-bytes` is used)
- Large numbers formatted with thousand separators

## Combining Metadata Options

### Comprehensive Analysis

```bash
# Full metadata analysis
rustree --show-size-bytes --show-last-modified --calculate-lines --calculate-words --human-friendly

# Focus on source code with full analysis
rustree -P "*.rs" --show-size-bytes --calculate-lines --calculate-words --sort-by size -r
```

### Performance Considerations

Content analysis (line/word counting) requires reading file contents, which can be slower for:
- Very large files
- Large numbers of files
- Network-mounted filesystems

Consider using [filtering](./filtering_and_patterns.md) to limit analysis to relevant files:

```bash
# Analyze only source files to improve performance
rustree --calculate-lines -P "*.rs|*.js|*.py" --depth 3
```

## Metadata Display Format

### Size Formatting

Sizes are displayed in brackets before the filename:

```bash
rustree -s
# Output: ├── [   1024B] config.toml
```

With `--human-friendly`:

```bash
rustree -s --human-friendly  
# Output: ├── [  1.0 KB] config.toml
```

### Line and Word Count Display

Line and word counts appear in brackets with prefixes:

```bash
rustree --calculate-lines --calculate-words
# Output: ├── [L: 150] [W: 750] main.rs
```

### Timestamp Display

Timestamps are shown in a readable format:

```bash
rustree -D
# Output: ├── [2024-01-15 14:30:25] main.rs
```

## Integration with Other Features

### Sorting by Metadata

Use metadata for sorting (see [Sorting and Ordering](./sorting_and_ordering.md)):

```bash
# Sort by file size (largest first)
rustree -s --sort-by size -r

# Sort by line count (most lines first)  
rustree --calculate-lines --sort-by lines -r

# Sort by modification time (newest first)
rustree -D --sort-by mtime -r
```

### Filtering with Metadata

Combine with [size-based filtering](./filtering_and_patterns.md#size-based-filtering):

```bash
# Show metadata for large files only
rustree -s --min-file-size 1M --calculate-lines

# Analyze medium-sized source files
rustree --calculate-lines --calculate-words \
        -P "*.rs" \
        --min-file-size 1K --max-file-size 100K
```

### Output Formatting

Metadata works with all [output formats](./output_formats.md):

```bash
# JSON output with metadata
rustree --show-size-bytes --calculate-lines --output-format json

# Markdown documentation with analysis
rustree --calculate-lines -P "*.md" --output-format markdown
```

## Project Analysis Examples

### Code Complexity Analysis

```bash
# Analyze source code complexity
rustree --calculate-lines --calculate-words --show-size-bytes \
        -P "*.rs|*.js|*.py" \
        --sort-by lines -r \
        --depth 3
```

### Documentation Overview

```bash  
# Analyze documentation completeness
rustree --calculate-words --show-size-bytes \
        -P "*.md|*.txt|README*" \
        --sort-by words -r
```

### Large File Detection

```bash
# Find large files with metadata
rustree --show-size-bytes --human-friendly \
        --min-file-size 1M \
        --sort-by size -r \
        --full-path
```

### Module Size Comparison

```bash
# Compare module sizes in a project
rustree --show-size-bytes --calculate-lines \
        --directory-only --depth 2 \
        --sort-by size -r
```

## Quick Reference

| Option | Short | Description |
|--------|-------|-------------|
| `--show-size-bytes` | `-s` | Display file sizes in bytes |
| `--show-last-modified` | `-D` | Show modification times (or change times with `-c`) |
| `--calculate-lines` | | Count and display lines in text files |
| `--calculate-words` | | Count and display words in text files |
| `--human-friendly` | | Format sizes in readable units (KB, MB, GB) |
| `--show-creation-time` | | Show file creation times (where supported) |

## Examples

See the [Examples](./examples.md) page for more detailed metadata analysis scenarios and real-world use cases.