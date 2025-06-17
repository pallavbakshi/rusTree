# Sorting and Ordering

RusTree provides flexible sorting options to organize tree output by various criteria. You can sort by name, size, timestamps, content analysis results, or custom function output.

## Basic Sorting

### Default Behavior

By default, RusTree sorts entries alphabetically by name:

```bash
# Default alphabetical sorting
rustree
```

### Disable Sorting

Display entries in directory order (as they appear on disk):

```bash
# No sorting - directory order
rustree -U

# Long form
rustree --unsorted

# Alternative form
rustree --sort-by none
```

## Sort by Name

### Alphabetical Sorting

```bash
# Explicit alphabetical sorting (default)
rustree --sort-by name
```

### Version Sorting

Sort by version numbers embedded in filenames:

```bash
# Version-aware sorting (file-1.0.0 before file-1.10.0)
rustree -v

# Long form
rustree --sort-by version
```

This is useful for files with version numbers where you want proper numerical ordering instead of lexicographic ordering.

## Sort by Size

### File Size Sorting

Sort by file size (requires size information):

```bash
# Sort by size (largest first)
rustree --show-size-bytes --sort-by size

# Sort by size (smallest first) 
rustree -s --sort-by size --reverse-sort
```

**Note:** You must enable `--show-size-bytes` for size sorting to work.

## Sort by Time

### Modification Time

Sort by when files were last modified:

```bash
# Sort by modification time (oldest first)
rustree -t

# Long form
rustree --sort-by mtime

# Newest first
rustree -t -r
```

### Change Time

Sort by when file metadata was last changed:

```bash
# Sort by change time (oldest first)
rustree -c

# Long form  
rustree --sort-by change_time

# Show change times when using this sort
rustree -c -D
```

### Creation Time

Sort by file creation time (where supported):

```bash
# Sort by creation time
rustree --sort-by create_time

# Newest created files first
rustree --sort-by create_time --reverse-sort
```

## Sort by Content Analysis

### Line Count Sorting

Sort by number of lines in files (requires line counting):

```bash
# Sort by line count (most lines first)
rustree --calculate-lines --sort-by lines

# Fewest lines first
rustree --calculate-lines --sort-by lines --reverse-sort
```

### Word Count Sorting  

Sort by number of words in files (requires word counting):

```bash
# Sort by word count (most words first)
rustree --calculate-words --sort-by words

# Fewest words first
rustree --calculate-words --sort-by words -r
```

## Sort by Custom Functions

### Apply Function Results

Sort by the output of apply functions:

```bash
# Sort by count-pluses output (most '+' characters first)
rustree --apply-function count-pluses --sort-by custom -r

# Sort by directory statistics
rustree --apply-function dir-stats --sort-by custom

# Sort by external command output
rustree --apply-function-cmd "wc -l" --sort-by custom -r
```

## Reverse Sorting

### Reverse Any Sort Order

Add `-r` or `--reverse-sort` to reverse any sort:

```bash
# Reverse alphabetical (Z to A)
rustree -r

# Largest files first
rustree -s --sort-by size -r

# Newest modifications first  
rustree -t -r

# Most complex files first (by line count)
rustree --calculate-lines --sort-by lines -r
```

## Directory vs File Ordering

### Directories First

Show all directories before files at each level:

```bash
# Directories before files
rustree --dirs-first

# Combine with other sorting
rustree --dirs-first --sort-by size -r
```

### Files First

Show all files before directories at each level:

```bash
# Files before directories
rustree --files-first

# Combine with sorting
rustree --files-first --sort-by mtime -r
```

**Note:** `--dirs-first` and `--files-first` are mutually exclusive.

## Sort Conflicts and Precedence

### Single-Letter Sort Options

The single-letter sort options (`-t`, `-c`, `-v`, `-U`) conflict with each other and with `--sort-by`:

```bash
# These conflict - only the last one applies
rustree -t -c -v    # Only -v (version) applies

# Use --sort-by for explicit control
rustree --sort-by mtime
```

### Sort Order Precedence

1. `--sort-by` takes precedence over single-letter options
2. `-U` (unsorted) disables all other sorting
3. `-r` applies to whatever sort is active

## Complex Sorting Examples

### Multi-Criteria Analysis

```bash
# Large source files with most lines first
rustree --show-size-bytes --calculate-lines \
        -P "*.rs|*.js|*.py" \
        --sort-by lines -r \
        --dirs-first

# Recently modified large files
rustree -s -D --sort-by mtime -r \
        --min-file-size 1M \
        --files-first

# Complex directories by statistics
rustree --apply-function dir-stats \
        --directory-only \
        --sort-by custom -r \
        --depth 2
```

### Combining with Filtering

```bash
# Largest documentation files
rustree -s -P "*.md|*.txt" \
        --sort-by size -r \
        --max-file-size 1M

# Most recently changed configuration
rustree -c -D --sort-by change_time -r \
        -P "*.toml|*.yml|*.json"

# Source files by complexity
rustree --calculate-lines --calculate-words \
        -P "*.rs" \
        --sort-by lines -r \
        --apply-exclude "**/target/**"
```

## Performance Considerations

### Expensive Sorts

Some sorts require additional processing:

- **Size sorting**: Requires reading file metadata
- **Content analysis sorting**: Requires reading and analyzing file contents  
- **Custom function sorting**: Depends on function complexity

Consider using [filtering](./filtering_and_patterns.md) to reduce the number of files processed:

```bash
# Efficient: filter first, then sort
rustree --calculate-lines -P "*.rs" --sort-by lines -r

# Less efficient: sort everything, then filter might happen later
rustree --calculate-lines --sort-by lines -r
```

### Large Directory Optimization

For large directories:

```bash
# Limit depth to improve performance
rustree --sort-by size -r --depth 2

# Use directory-only for structure analysis
rustree --sort-by size -r --directory-only

# Combine with pruning
rustree --sort-by lines -r --prune-empty-directories
```

## Integration with Other Features

### Output Formats

Sorting works with all output formats:

```bash
# Sorted JSON output
rustree --sort-by size -r --output-format json

# Sorted Markdown
rustree --sort-by mtime -r --output-format markdown
```

### Metadata Display

Show relevant metadata for your sort criteria:

```bash
# Show sizes when sorting by size
rustree -s --sort-by size -r

# Show times when sorting by time
rustree -D --sort-by mtime -r

# Show line counts when sorting by lines
rustree --calculate-lines --sort-by lines -r
```

## Quick Reference

### Sort Keys

| Option | Short | Description |
|--------|-------|-------------|
| `--sort-by name` | (default) | Alphabetical sorting |
| `--sort-by version` | `-v` | Version-aware sorting |
| `--sort-by size` | | Sort by file size (needs `--show-size-bytes`) |
| `--sort-by mtime` | `-t` | Sort by modification time |
| `--sort-by change_time` | `-c` | Sort by change time |
| `--sort-by create_time` | | Sort by creation time |
| `--sort-by lines` | | Sort by line count (needs `--calculate-lines`) |
| `--sort-by words` | | Sort by word count (needs `--calculate-words`) |
| `--sort-by custom` | | Sort by apply function output |
| `--sort-by none` | `-U` | No sorting (directory order) |

### Sort Modifiers

| Option | Short | Description |
|--------|-------|-------------|
| `--reverse-sort` | `-r` | Reverse the sort order |
| `--dirs-first` | | Show directories before files |
| `--files-first` | | Show files before directories |

### Sort Direction Defaults

- **Name/Version**: A to Z (use `-r` for Z to A)
- **Size**: Largest first (use `-r` for smallest first)  
- **Time**: Oldest first (use `-r` for newest first)
- **Lines/Words**: Most first (use `-r` for fewest first)
- **Custom**: Depends on function output type

## Examples

See the [Examples](./examples.md) page for more detailed sorting scenarios and real-world use cases.