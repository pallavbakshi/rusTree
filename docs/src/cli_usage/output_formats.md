# Output Formats

RusTree supports multiple output formats to suit different use cases, from human-readable text to structured data formats and web-ready HTML. This page covers all output format options and customization.

## Available Formats

### Text Format (Default)

The default tree-like text output:

```bash
# Default text format
rustree

# Explicit text format
rustree --output-format text
```

**Example output:**
```
my_project/
├── README.md
├── src/
│   ├── main.rs
│   └── lib.rs
└── tests/
    └── integration.rs

2 directories, 4 files
```

### Markdown Format

List-based Markdown output suitable for documentation:

```bash
# Markdown format
rustree --output-format markdown
```

**Example output:**
```markdown
# my_project

* README.md
* src/
  * main.rs
  * lib.rs
* tests/
  * integration.rs

__2 directories, 4 files total__
```

### JSON Format

Structured JSON data for programmatic processing:

```bash
# JSON format
rustree --output-format json
```

**Example output:**
```json
{
  "nodes": [
    {
      "path": "./README.md",
      "name": "README.md",
      "type": "file",
      "depth": 1,
      "size": 1024
    },
    {
      "path": "./src",
      "name": "src",
      "type": "directory", 
      "depth": 1
    }
  ],
  "summary": {
    "directories": 2,
    "files": 4,
    "total_size": 5120
  }
}
```

### HTML Format

Web-ready HTML with optional hyperlinks and customization:

```bash
# Basic HTML format
rustree --output-format html

# HTML with hyperlinks
rustree --output-format html --html-base-href https://example.com/repo
```

## Summary Report Control

### Disable Summary

Remove the summary line from output:

```bash
# No summary report
rustree --no-summary-report

# Useful for clean Markdown documentation
rustree --output-format markdown --no-summary-report > structure.md
```

### Enhanced Summary with Metadata

When using metadata options, the summary automatically includes totals:

```bash
# Enhanced summary with aggregated metadata
rustree --calculate-lines --calculate-words --show-size-bytes
```

**Output:**
```
my_project/
├── [1024B] [L: 50] [W: 250] README.md
└── src/
    └── [2048B] [L: 100] [W: 500] main.rs

1 directory, 2 files, 150 total lines, 750 total words, 3.1 KB total
```

## HTML Customization

### Base URL for Links

Create clickable links in HTML output:

```bash
# Add base URL to all links
rustree --output-format html --html-base-href https://github.com/user/repo/blob/main

# Local file links
rustree --output-format html --html-base-href file:///path/to/project
```

### Strip Path Components

Remove leading path components from links:

```bash
# Scan subdirectory but link to parent paths
rustree src/ --output-format html \
       --html-base-href https://example.com \
       --html-strip-first-component
```

This is useful when scanning a subdirectory but wanting links to be relative to the parent.

### Disable Links

Generate HTML without hyperlinks:

```bash
# Plain HTML without <a> tags
rustree --output-format html --html-no-links
```

### Custom HTML Templates

#### Custom Header

Replace the default HTML header:

```bash
# Use custom intro HTML
rustree --output-format html --html-intro-file ./templates/header.html

# Suppress header entirely
rustree --output-format html --html-intro-file /dev/null
```

#### Custom Footer

Replace the default HTML footer:

```bash
# Use custom outro HTML  
rustree --output-format html --html-outro-file ./templates/footer.html

# Suppress footer entirely
rustree --output-format html --html-outro-file /dev/null
```

## Format-Specific Features

### JSON with Metadata

JSON format includes all collected metadata:

```bash
# Rich JSON with full metadata
rustree --output-format json \
        --show-size-bytes \
        --calculate-lines \
        --show-last-modified
```

### Markdown with Metadata

Metadata appears inline in Markdown format:

```bash
# Markdown with file sizes and line counts
rustree --output-format markdown \
        --show-size-bytes \
        --calculate-lines \
        --human-friendly
```

### JSON Processing

Use JSON output with external tools:

```bash
# Pretty-print JSON
rustree --output-format json | jq '.'

# Extract specific information
rustree --output-format json | jq '.nodes[] | select(.type == "file") | .name'

# Get file count
rustree --output-format json | jq '.summary.files'

# Find large files
rustree --output-format json --show-size-bytes | \
  jq '.nodes[] | select(.size > 1000000) | .path'
```

## LLM Integration with Formats

### Combined JSON Output

When using LLM features with JSON format, both tree and LLM data are included:

```bash
# Tree + LLM analysis in single JSON
rustree --output-format json \
        --llm-ask "Analyze this project structure" \
        --dry-run
```

**Output:**
```json
{
  "tree": {
    "nodes": [...],
    "summary": {...}
  },
  "llm": {
    "dry_run": true,
    "request": {...},
    "question": "Analyze this project structure"
  }
}
```

## Practical Examples

### Documentation Generation

```bash
# Clean Markdown for documentation
rustree docs/ --output-format markdown \
       --no-summary-report \
       --depth 3 > docs-structure.md

# HTML for web documentation
rustree --output-format html \
        --html-base-href https://docs.example.com \
        --html-intro-file ./templates/docs-header.html \
        --depth 2 > docs-index.html
```

### Data Processing

```bash
# Generate data for analysis
rustree --output-format json \
        --show-size-bytes \
        --calculate-lines > project-data.json

# Process with jq
cat project-data.json | jq '.nodes[] | select(.lines > 100)'
```

### Web Publishing

```bash
# Complete web page with navigation
rustree src/ --output-format html \
       --html-base-href https://github.com/user/repo/tree/main/src \
       --html-intro-file ./web/header.html \
       --html-outro-file ./web/footer.html \
       --show-size-bytes --human-friendly > src-browser.html
```

### Archival and Reporting

```bash
# Comprehensive project snapshot
rustree --output-format json \
        --show-size-bytes \
        --show-last-modified \
        --calculate-lines \
        --calculate-words > "snapshot-$(date +%Y%m%d).json"

# Human-readable report
rustree --output-format markdown \
        --show-size-bytes --human-friendly \
        --calculate-lines \
        --sort-by size -r > project-report.md
```

## Format Compatibility

### Cross-Format Workflows

```bash
# Generate in multiple formats
rustree --output-format json > data.json
rustree --output-format markdown > structure.md  
rustree --output-format html > browser.html

# Convert between formats using external tools
rustree --output-format json | jq -r '.nodes[].path' > file-list.txt
```

### Integration with External Tools

```bash
# Feed to other analysis tools
rustree --output-format json | python analyze-structure.py

# Generate reports
rustree --output-format markdown | pandoc -o report.pdf

# Create web assets
rustree --output-format html --html-no-links | \
  sed 's/<pre>/<pre class="tree-view">/' > styled-tree.html
```

## Quick Reference

### Format Options

| Option | Description |
|--------|-------------|
| `--output-format text` | Default tree-like text output |
| `--output-format markdown` | Markdown list format |
| `--output-format json` | Structured JSON data |
| `--output-format html` | Web-ready HTML |

### Summary Control

| Option | Description |
|--------|-------------|
| `--no-summary-report` | Omit the summary line from output |

### HTML-Specific Options

| Option | Description |
|--------|-------------|
| `--html-base-href <URL>` | Base URL for generated hyperlinks |
| `--html-strip-first-component` | Remove first path component from links |
| `--html-no-links` | Disable hyperlink generation |
| `--html-intro-file <FILE>` | Custom HTML header template |
| `--html-outro-file <FILE>` | Custom HTML footer template |

## Examples

See the [Examples](./examples.md) page for more detailed output format scenarios and integration examples.