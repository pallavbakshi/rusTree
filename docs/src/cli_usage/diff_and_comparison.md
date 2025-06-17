# Diff and Comparison

RusTree includes powerful directory comparison capabilities to track changes over time. The diff feature can detect additions, removals, modifications, moves/renames, and type changes between directory snapshots.

## Basic Diff Operations

### Create and Compare Snapshots

First, create a baseline snapshot in JSON format:

```bash
# Save current state as baseline
rustree --output-format json > baseline.json

# Later, after making changes, compare current state with baseline
rustree --diff baseline.json
```

### Compare Two Snapshots

Compare two saved snapshots without scanning the current directory:

```bash
# Compare two specific snapshots
rustree --diff new_snapshot.json --from-tree-file old_snapshot.json
```

### Input Sources

The diff feature supports various input sources:

```bash
# Compare with current directory (default)
rustree --diff baseline.json

# Compare two files
rustree --diff new.json --from-tree-file old.json

# Compare against different directory
rustree --diff baseline.json /path/to/other/directory
```

## Change Types

RusTree detects several types of changes:

- **Added** `[+]` - New files or directories
- **Removed** `[-]` - Deleted files or directories
- **Modified** `[M]` - Files with changed content, size, or timestamps
- **Moved** `[→]` - Files or directories that were moved/renamed
- **Type Changed** `[T]` - Entries that changed type (file to directory, etc.)
- **Unchanged** `[=]` - No changes (shown only with `--show-unchanged`)

## Diff Output Formats

### Text Format (Default)

Shows changes in a tree structure with change markers:

```bash
rustree --diff baseline.json
```

**Example output:**
```
project/
├── [+] new_file.rs
├── [M] README.md
├── [-] old_config.toml
├── [→] src/main.rs (moved from lib.rs)
└── src/
    └── [T] data (file → directory)

Changes: 2 added, 1 removed, 1 modified, 1 moved, 1 type changed
```

### Markdown Format

Generate markdown reports for documentation:

```bash
rustree --diff baseline.json --output-format markdown > changes.md
```

### JSON Format

Structured data for programmatic processing:

```bash
rustree --diff baseline.json --output-format json > diff_report.json
```

### HTML Format

Interactive viewing in web browsers:

```bash
rustree --diff baseline.json --output-format html > diff_report.html
```

## Filtering Changes

### Show Specific Change Types

Display only certain types of changes:

```bash
# Show only additions and removals
rustree --diff baseline.json --show-only added,removed

# Show only modifications
rustree --diff baseline.json --show-only modified

# Show moves and type changes
rustree --diff baseline.json --show-only moved,type_changed
```

Available change types:
- `added` - New files/directories
- `removed` - Deleted files/directories
- `modified` - Changed files
- `moved` - Moved/renamed files/directories
- `type_changed` - Type changes (file ↔ directory)

### Show Unchanged Files

Include unchanged files in the output:

```bash
# Show all files, including unchanged ones
rustree --diff baseline.json --show-unchanged
```

This provides a complete picture showing what changed and what didn't.

### Statistics Only

Show only summary statistics without the detailed tree:

```bash
# Summary statistics only
rustree --diff baseline.json --stats-only
```

**Example output:**
```
Changes Summary:
- 5 files added
- 2 files removed
- 3 files modified
- 1 file moved
- 0 type changes
Total: 11 changes detected
```

## Move Detection

### Move Detection Settings

Control how aggressively RusTree detects file moves:

```bash
# Default move detection (80% similarity)
rustree --diff baseline.json

# Strict move detection (95% similarity)
rustree --diff baseline.json --move-threshold 0.95

# Loose move detection (50% similarity)
rustree --diff baseline.json --move-threshold 0.5

# Disable move detection entirely
rustree --diff baseline.json --ignore-moves
```

### How Move Detection Works

RusTree uses content similarity to detect moves:
- Compares file sizes and content hashes
- Calculates similarity scores between files
- Files above the threshold are considered moves
- Without move detection, moves appear as separate add/remove operations

### Move Threshold Guidelines

- **0.9-1.0**: Very strict - only nearly identical files
- **0.7-0.9**: Balanced - good for most use cases (default: 0.8)
- **0.5-0.7**: Loose - catches more potential moves but may have false positives
- **Below 0.5**: Very loose - likely to produce incorrect move detection

## Change Thresholds

### Size-Based Filtering

Only report changes above certain size thresholds:

```bash
# Only report size changes of 1MB or more
rustree --diff baseline.json --size-threshold 1M

# Report changes above 100KB
rustree --diff baseline.json --size-threshold 100K

# Use with size units (K, M, G)
rustree --diff baseline.json --size-threshold 512K
```

### Time-Based Filtering

Only report changes above time thresholds:

```bash
# Only show files changed more than 1 hour ago
rustree --diff baseline.json --time-threshold 3600

# Files changed more than 1 day ago
rustree --diff baseline.json --time-threshold 86400
```

Time threshold is in seconds since the timestamp difference.

## Integration with Other Features

### Combining with Filtering

Apply standard filtering to focus diff analysis:

```bash
# Diff only source code changes
rustree --diff baseline.json \
        --filter-include "*.rs" "*.toml" \
        --filter-exclude "**/target/**"

# Track documentation changes
rustree --diff baseline.json \
        --filter-include "*.md" "README*" \
        --depth 3

# Use pattern files for complex filtering
rustree --diff baseline.json \
        --filter-include-from src-patterns.txt \
        --filter-exclude-from ignore-patterns.txt
```

### Metadata in Diff

Include metadata to understand change impact:

```bash
# Show size changes
rustree --diff baseline.json \
        --show-size-bytes --human-friendly

# Include modification times
rustree --diff baseline.json \
        --show-last-modified

# Full metadata analysis
rustree --diff baseline.json \
        --show-size-bytes \
        --show-last-modified \
        --calculate-lines \
        --human-friendly
```

### Sorting Diff Results

Sort changes by various criteria:

```bash
# Sort by size (largest changes first)
rustree --diff baseline.json \
        --show-size-bytes \
        --sort-by size --reverse-sort

# Sort by modification time (most recent first)
rustree --diff baseline.json \
        --show-last-modified \
        --sort-by mtime --reverse-sort

# Group directories first
rustree --diff baseline.json --dirs-first
```

## LLM Analysis of Changes

### AI-Powered Change Analysis

Combine diff with LLM analysis:

```bash
# Analyze the impact of changes
rustree --diff baseline.json \
        --llm-ask "What are the most significant changes and their potential impact?"

# Security-focused change review
rustree --diff baseline.json \
        --filter-include "*.rs" "*.toml" \
        --llm-ask "Are there any security concerns in these changes?"

# Performance impact analysis
rustree --diff baseline.json \
        --show-size-bytes \
        --llm-ask "Could these changes affect performance?"
```

## Practical Workflows

### Development Monitoring

```bash
# Save snapshot before starting feature
rustree --output-format json > feature-start.json

# During development, check progress
rustree --diff feature-start.json --stats-only

# Before commit, review all changes
rustree --diff feature-start.json \
        --filter-include "src/**" "tests/**" \
        --show-size-bytes
```

### Release Tracking

```bash
# Compare releases
rustree --diff v1.0-snapshot.json --from-tree-file v1.1-snapshot.json

# Track major changes between versions
rustree --diff v1.0-snapshot.json \
        --show-only added,removed \
        --filter-include "src/**"

# Generate release notes
rustree --diff v1.0-snapshot.json \
        --output-format markdown \
        --show-size-bytes > release-changes.md
```

### Build Monitoring

```bash
# Before build
rustree --output-format json > pre-build.json

# After build - see generated artifacts
rustree --diff pre-build.json \
        --filter-include "target/**" "build/**" \
        --show-only added \
        --show-size-bytes --human-friendly
```

### System Administration

```bash
# Daily system monitoring
DATE=$(date +%Y%m%d)
YESTERDAY=$(date -d yesterday +%Y%m%d)

# Monitor important directories
rustree --diff "system-$YESTERDAY.json" /etc \
        --filter-exclude "*.log" "*.tmp" \
        --show-last-modified

# Track configuration changes
rustree --diff baseline-config.json /etc/nginx \
        --show-only modified,added,removed
```

## Automation and Scripting

### Automated Change Detection

```bash
#!/bin/bash
# Daily change monitoring script

PROJECT_DIR="/path/to/project"
DATE=$(date +%Y%m%d)
YESTERDAY=$(date -d yesterday +%Y%m%d)

# Generate today's snapshot
rustree --output-format json "$PROJECT_DIR" > "snapshot-$DATE.json"

# Compare with yesterday if exists
if [ -f "snapshot-$YESTERDAY.json" ]; then
    echo "Changes since yesterday:"
    rustree --diff "snapshot-$YESTERDAY.json" "$PROJECT_DIR" \
            --filter-exclude "*.log" "*.tmp" \
            --show-size-bytes --stats-only

    # Alert on significant changes
    CHANGES=$(rustree --diff "snapshot-$YESTERDAY.json" "$PROJECT_DIR" --stats-only | grep -c "files added\|files removed")
    if [ "$CHANGES" -gt 5 ]; then
        echo "ALERT: Significant changes detected!"
    fi
fi
```

### CI/CD Integration

```bash
# In CI pipeline
BUILD_ID=$(git rev-parse --short HEAD)
BASE_BRANCH="main"

# Create snapshot
rustree --output-format json > "build-$BUILD_ID.json"

# Compare with main branch baseline
if [ -f "baseline-$BASE_BRANCH.json" ]; then
    # Generate change report for review
    rustree --diff "baseline-$BASE_BRANCH.json" \
            --output-format markdown \
            --filter-include "src/**" "tests/**" \
            --show-size-bytes > "changes-$BUILD_ID.md"

    # Check for structural changes
    rustree --diff "baseline-$BASE_BRANCH.json" \
            --show-only added,removed \
            --stats-only
fi
```

## Troubleshooting

### Large Directory Performance

For large projects, optimize diff performance:

```bash
# Use depth limits
rustree --diff baseline.json --depth 3

# Focus on specific areas
rustree --diff baseline.json src/ tests/

# Use filtering to reduce scope
rustree --diff baseline.json \
        --filter-include "*.rs" \
        --filter-exclude "**/target/**" \
        --prune-empty-directories
```

### Debugging Issues

```bash
# Verify snapshot contents
rustree --output-format json . | jq '.nodes | length'

# Check path normalization issues
rustree --diff baseline.json --full-path

# Test move detection settings
rustree --diff baseline.json --move-threshold 0.1  # Very loose
rustree --diff baseline.json --ignore-moves        # Disable moves
```

## Quick Reference

### Core Diff Options

| Option | Description |
|--------|-------------|
| `--diff <FILE>` | Compare current directory with snapshot file |
| `--from-tree-file <FILE>` | Use file as source instead of current directory |
| `--show-only <TYPES>` | Show only specific change types |
| `--show-unchanged` | Include unchanged files in output |
| `--stats-only` | Show only summary statistics |

### Move Detection

| Option | Description |
|--------|-------------|
| `--ignore-moves` | Disable move/rename detection |
| `--move-threshold <FLOAT>` | Similarity threshold for moves (0.0-1.0, default: 0.8) |

### Change Filtering

| Option | Description |
|--------|-------------|
| `--size-threshold <BYTES>` | Only report size changes above threshold |
| `--time-threshold <SECONDS>` | Only report time changes above threshold |

### Change Types

| Type | Marker | Description |
|------|--------|-------------|
| `added` | `[+]` | New files or directories |
| `removed` | `[-]` | Deleted files or directories |
| `modified` | `[M]` | Changed files |
| `moved` | `[→]` | Moved or renamed files/directories |
| `type_changed` | `[T]` | Type change (file ↔ directory) |
| `unchanged` | `[=]` | No changes (with `--show-unchanged`) |

## Examples

See the [Examples](./examples.md) page for more detailed diff scenarios and real-world use cases.
