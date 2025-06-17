# Filtering and Patterns

RusTree provides powerful filtering capabilities to show only the files and directories you're interested in. This page covers include/exclude patterns, gitignore integration, and size-based filtering.

## Pattern Syntax

RusTree uses glob patterns for matching files and directories:

- `*` - Matches any number of characters (except `/`)
- `?` - Matches exactly one character  
- `[abc]` - Matches any character in brackets
- `[a-z]` - Matches any character in range
- `[!abc]` - Matches any character NOT in brackets
- `**` - Matches any number of directories (recursive)
- `|` - Separates alternative patterns within one argument

## Include Patterns

### Basic Include Filtering

Show only files/directories that match specific patterns:

```bash
# Show only Rust files
rustree -P "*.rs"

# Show only documentation files
rustree --filter-include "*.md"

# Multiple file types using pipe separator
rustree -P "*.rs|*.toml|*.md"
```

### Multiple Include Patterns

You can specify multiple `-P` options:

```bash
# These are equivalent
rustree -P "*.rs" -P "*.toml" -P "*.md"
rustree -P "*.rs|*.toml|*.md"
```

### Directory-Specific Patterns

Use a trailing `/` to match directories only:

```bash
# Match directories named "src" or "lib"
rustree -P "src/|lib/"

# Match any directory ending with "_test"  
rustree -P "*_test/"
```

### Path-Based Patterns

Use `/` or `**` for path-based matching:

```bash
# Files directly in src directory
rustree -P "src/*.rs"

# Rust files anywhere in the tree
rustree -P "**/*.rs"

# Files in any subdirectory of src
rustree -P "src/**"

# Specific nested path
rustree -P "src/cli/*.rs"
```

## Exclude Patterns

### Basic Exclude Filtering

Hide files/directories that match patterns:

```bash
# Hide log files
rustree -I "*.log"

# Hide build directories
rustree --filter-exclude "target/"

# Hide multiple types
rustree -I "*.tmp|*.log|*.bak"
```

### Multiple Exclude Patterns

```bash
# Multiple exclude options
rustree -I "target/" -I "node_modules/" -I "*.log"

# Pipe-separated in single option
rustree -I "target/|node_modules/|*.log"
```

### Complex Exclusions

```bash
# Hide all hidden directories but keep hidden files
rustree -I ".*/"

# Hide test files but keep test directories  
rustree -I "*test*.rs" --filter-include "*test*/"
```

## Pattern Files

### Include Patterns from File

Store patterns in files for reusability:

Create `include-patterns.txt`:
```
# Source code files
*.rs
*.go
*.js
*.ts

# Configuration files  
*.toml
*.yml
*.yaml

# Documentation
*.md
README*
```

Use the file:
```bash
rustree --filter-include-from ./include-patterns.txt
```

### Exclude Patterns from File

Create `exclude-patterns.txt`:
```
# Build artifacts
target/
build/
dist/

# Dependencies
node_modules/
vendor/

# Temporary files
*.tmp
*.log
*.swp
.DS_Store
```

Use the file:
```bash
rustree --filter-exclude-from ./exclude-patterns.txt
```

### Multiple Pattern Files

```bash
# Load from multiple files
rustree --filter-include-from ./src-patterns.txt \
        --filter-include-from ./docs-patterns.txt \
        --filter-exclude-from ./ignore-patterns.txt
```

### Pattern File Format

- One pattern per line
- Lines starting with `#` are comments
- Empty lines are ignored
- Same glob syntax as command-line patterns

## Gitignore Integration

### Basic Gitignore Support

Respect `.gitignore` files automatically:

```bash
# Use .gitignore rules
rustree --use-gitignore-rules

# Old alias (deprecated but still works)
rustree --gitignore
```

This respects:
- `.gitignore` files in the scanned directories
- Global gitignore file (`~/.config/git/ignore`)
- Repository exclude files (`$GIT_DIR/info/exclude`)

### Custom Ignore Files

Use specific files as gitignore sources:

```bash
# Use custom ignore file
rustree --gitignore-file ./.customignore

# Multiple ignore files
rustree --gitignore-file ./.customignore --gitignore-file ./project.ignores

# Combine with standard gitignore
rustree --use-gitignore-rules --gitignore-file ./.extraignores
```

## Case Sensitivity

### Case-Insensitive Matching

Make all pattern matching case-insensitive:

```bash
# Match *.TXT, *.txt, *.Txt, etc.
rustree -P "*.txt" --case-insensitive-filter

# Apply to all filter types
rustree -P "*.RS" -I "TARGET/" --case-insensitive-filter --use-gitignore-rules
```

Affects:
- Include patterns (`-P`, `--filter-include`)  
- Exclude patterns (`-I`, `--filter-exclude`)
- Pattern files (`--filter-include-from`, `--filter-exclude-from`)
- Gitignore patterns (`--use-gitignore-rules`, `--gitignore-file`)

## Size-Based Filtering

### Minimum File Size

Include only files above a certain size:

```bash
# Files at least 100KB
rustree --min-file-size 100K

# Files at least 2MB
rustree --min-file-size 2M

# Files at least 1GB
rustree --min-file-size 1G
```

### Maximum File Size

Include only files below a certain size:

```bash
# Files no larger than 500KB  
rustree --max-file-size 500K

# Small files only (under 1MB)
rustree --max-file-size 1M
```

### Size Ranges

Combine min and max for size ranges:

```bash
# Files between 10KB and 1MB
rustree --min-file-size 10K --max-file-size 1M

# Medium-sized files (100KB to 10MB)
rustree --min-file-size 100K --max-file-size 10M
```

### Size Units

Supported size suffixes (base-1024):
- `K` - Kibibytes (1024 bytes)
- `M` - Mebibytes (1024² bytes)  
- `G` - Gibibytes (1024³ bytes)

**Note:** Size filtering only applies to files, not directories.

## Empty Directory Pruning

### Remove Empty Directories

Hide directories that become empty after filtering:

```bash
# Remove empty directories
rustree --prune-empty-directories

# Short alias
rustree --prune

# Combine with filtering
rustree -P "*.rs" --prune
```

This is applied after all other filtering, so a directory containing only filtered-out files will be considered empty and pruned.

## Hidden Files and Patterns

### Pattern Matching with Hidden Files

By default, patterns like `*` don't match hidden files:

```bash
# This WON'T match .hidden_file
rustree -P "*"

# This WILL match .hidden_file
rustree -a -P "*"

# This WILL match .hidden_file (explicit dot)
rustree -P ".*"
```

Use `-a` with general patterns to include hidden files in pattern matching.

## Combining Filters

### Complex Filtering Examples

```bash
# Source code only, no build artifacts
rustree -P "*.rs|*.toml" -I "target/" --use-gitignore-rules

# Large documentation files
rustree -P "*.md" --min-file-size 10K --max-file-size 1M

# Configuration files with custom ignore
rustree -P "*.yml|*.yaml|*.toml|*.json" \
        --filter-exclude-from ./temp-ignores.txt \
        --case-insensitive-filter

# Include from file, exclude specific patterns
rustree --filter-include-from ./source-patterns.txt \
        -I "*.test.*" -I "*_backup*" \
        --prune-empty-directories
```

## Integration with Other Features

Filtering works with all other RusTree features:

- **[Apply Functions](./apply_functions.md)**: Apply functions only to filtered files
- **[Metadata Analysis](./metadata_and_analysis.md)**: Show metadata for filtered files  
- **[Sorting](./sorting_and_ordering.md)**: Sort filtered results
- **[Output Formats](./output_formats.md)**: Format filtered output

## Quick Reference

| Option | Short | Description |
|--------|-------|-------------|
| `--filter-include <PATTERN>` | `-P` | Include only files matching pattern |
| `--filter-exclude <PATTERN>` | `-I` | Exclude files matching pattern |
| `--filter-include-from <FILE>` | | Read include patterns from file |
| `--filter-exclude-from <FILE>` | | Read exclude patterns from file |
| `--use-gitignore-rules` | | Respect .gitignore files |
| `--gitignore-file <FILE>` | | Use specific file as gitignore source |
| `--case-insensitive-filter` | | Make all pattern matching case-insensitive |
| `--min-file-size <SIZE>` | | Include only files at least this size |
| `--max-file-size <SIZE>` | | Include only files no larger than this size |
| `--prune-empty-directories` | | Remove directories that become empty after filtering |

## Examples

See the [Examples](./examples.md) page for more detailed filtering scenarios and real-world use cases.