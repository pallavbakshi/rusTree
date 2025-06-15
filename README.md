
# 🌳 RusTree

A powerful directory tree generator and analyzer with integrated LLM capabilities for intelligent code analysis.

## Features

### Core Functionality
- **Directory Tree Generation**: Beautiful ASCII tree visualization
- **Flexible Filtering**: Include/exclude patterns, gitignore support, size filters
- **Rich Metadata**: File sizes, modification dates, line counts, and statistics
- **Multiple Output Formats**: Text tree, Markdown, and custom formatting
- **Performance Optimized**: Fast traversal with configurable depth limits

### 🤖 LLM Integration (New!)
- **Direct LLM Analysis**: Ask questions about your codebase structure
- **Multiple Providers**: OpenAI, Anthropic, Cohere, OpenRouter support
- **Smart Context**: Automatically includes project metadata in LLM prompts
- **Export Mode**: Generate formatted queries for external LLM tools
- **Environment Support**: Auto-loads API keys from `.env` files

## Quick Start

### Basic Usage
```bash
# Generate a simple tree
rustree

# With depth limit and metadata
rustree --depth 3 --size --file-stats
```

### LLM Analysis
```bash
# Ask a question about your codebase
rustree --llm-ask "What's the architecture of this project?" --llm-provider openai

# Export for external LLM tools
rustree --llm-export "Analyze security concerns" | claude-cli

# Generate .env template for API keys
rustree --llm-generate-env > .env
```

## LLM Setup

### 1. Install Dependencies
The LLM functionality requires API keys from your chosen provider(s).

### 2. Set API Keys

**Option A: Environment Variables**
```bash
export OPENAI_API_KEY="sk-your-openai-key"
export ANTHROPIC_API_KEY="sk-ant-your-anthropic-key"
export COHERE_API_KEY="your-cohere-key"
export OPENROUTER_API_KEY="sk-your-openrouter-key"
```

**Option B: .env File**
```bash
# Generate template
rustree --llm-generate-env > .env

# Edit .env file
nano .env
```

**Option C: CLI Arguments**
```bash
rustree --llm-ask "Question" --llm-api-key "your-key"
```

### 3. Choose Provider & Model
```bash
# OpenAI (default)
rustree --llm-ask "Analyze this" --llm-provider openai --llm-model gpt-4

# Anthropic
rustree --llm-ask "Code review" --llm-provider anthropic --llm-model claude-3-sonnet

# Custom settings
rustree --llm-ask "Brief analysis" --llm-temperature 0.3 --llm-max-tokens 500
```

## Command Reference

### Core Options
- `--depth <N>`: Limit tree depth
- `--size`: Show file sizes
- `--file-stats`: Show file statistics
- `--dirs-only`: Show directories only
- `--filter-include <pattern>`: Include files matching pattern
- `--filter-exclude <pattern>`: Exclude files matching pattern
- `--filter-include-from <file>`: Read include patterns from file
- `--filter-exclude-from <file>`: Read exclude patterns from file

### LLM Options
- `--llm-ask <question>`: Ask LLM directly
- `--llm-export <question>`: Export formatted query
- `--llm-provider <provider>`: Choose provider (openai, anthropic, cohere, openrouter)
- `--llm-model <model>`: Specify model
- `--llm-temperature <float>`: Response randomness (0.0-2.0)
- `--llm-max-tokens <int>`: Maximum response length
- `--llm-generate-env`: Generate .env template
- `--dry-run`: Preview LLM request without sending (for debugging/rough cost estimation)
- `--human-friendly`: Format dry-run output in readable markdown (requires --dry-run)

## Examples

### Architecture Analysis
```bash
rustree --llm-ask "What architectural patterns do you see in this codebase?" \
  --depth 4 --dirs-only --llm-temperature 0.2
```

### Security Review
```bash
rustree --llm-ask "Are there any potential security concerns?" \
  --filter-include "*.rs" --filter-exclude "**/target/**" --llm-provider anthropic
```

### Performance Analysis  
```bash
rustree --llm-ask "What could be optimized for better performance?" \
  --size --file-stats --llm-max-tokens 300
```

### Export for External Tools
```bash
# Use with external LLM CLI
rustree --llm-export "Suggest refactoring opportunities" | some-llm-tool
```

### Request Preview and Debugging
```bash
# Preview what would be sent to the LLM (no API call)
rustree --llm-ask "What's this project about?" --dry-run

# Human-readable markdown format for better readability
rustree --llm-ask "Analyze the architecture" --dry-run --human-friendly

# Perfect for cost estimation and debugging prompts
rustree --llm-ask "Complex analysis question" --llm-max-tokens 2000 --dry-run

# Note: Token estimates are rough approximations (4:1 char ratio)
# Use for planning only - actual usage may vary by content/provider

# Save for later analysis
rustree --llm-export "Technical debt assessment" > analysis-prompt.txt
```

### File-based Pattern Filtering
```bash
# Create pattern files for reusable filtering
echo "*.rs" > include-rust.txt
echo "*.md" > include-docs.txt
echo "target/" > exclude-build.txt
echo "*.log" >> exclude-build.txt

# Use pattern files
rustree --filter-include-from include-rust.txt ./src

# Combine multiple pattern files
rustree --filter-include-from include-rust.txt \
        --filter-include-from include-docs.txt \
        --filter-exclude-from exclude-build.txt ./

# Mix file patterns with command-line patterns
rustree --filter-include-from include-rust.txt \
        --filter-include "*.toml" \
        --filter-exclude "*.test.rs" ./
```

## 🔄 Tree Diff Feature

Compare directory snapshots to track changes over time. Perfect for development monitoring, backup verification, and change analysis.

### Quick Start

```bash
# Save current state as baseline
rustree --output-format json > baseline.json

# Later, compare current state with baseline
rustree --diff baseline.json

# Compare two snapshots
rustree --diff new_snapshot.json --from-tree-file old_snapshot.json
```

### Core Diff Features

**Change Types Detected:**
- `[+]` **Added**: New files and directories
- `[-]` **Removed**: Deleted files and directories  
- `[M]` **Modified**: Directories with changed contents
- `[~]` **Moved**: Files/directories relocated or renamed
- `[T]` **Type Changed**: File ↔ Directory conversions
- `[=]` **Unchanged**: Files with no changes (when `--show-unchanged`)

**Smart Move Detection:**
- Automatic detection of moved/renamed files
- Similarity scoring based on name, size, and modification time
- Configurable similarity threshold (`--move-threshold`)
- Option to disable move detection (`--ignore-moves`)

### Diff Examples

#### Development Workflow Monitoring
```bash
# Track changes since last release
rustree --diff release-v1.0.json --filter-include "src/**" "docs/**"

# Monitor build artifact generation
rustree --diff pre-build.json --filter-include "target/**" --show-size-bytes

# Track dependency updates  
rustree --diff before-update.json node_modules/ --show-size-bytes --human-friendly
```

#### System Administration
```bash
# Monitor configuration changes
rustree --diff system-baseline.json /etc/ --show-last-modified

# Track daily changes (ignore logs)
rustree --diff daily-snapshot.json --filter-exclude "*.log" "*.tmp"

# Audit with detailed output
rustree --diff security-baseline.json --sort-by mod_time --dirs-first
```

#### Code Review Support
```bash
# Understand structural changes
rustree --diff feature-start.json --llm-ask "Summarize the structural changes"

# Track refactoring progress
rustree --diff before-refactor.json src/ --output-format markdown

# Analyze large-scale moves
rustree --diff old-structure.json --show-only moved --move-threshold 0.9
```

### Advanced Diff Options

```bash
# Diff-specific options
--diff <file>                    # Compare with snapshot file
--show-only <types>             # Show only: added,removed,modified,moved,type_changed
--ignore-moves                  # Don't detect moves, treat as add+remove  
--move-threshold <0.0-1.0>      # Similarity threshold (default: 0.8)
--show-unchanged               # Include unchanged files in output
--stats-only                   # Show only summary statistics

# Size and time thresholds
--size-threshold <bytes>        # Minimum size change to report
--time-threshold <seconds>      # Minimum time change to report
```

### Diff Output Formats

#### Text Format (Default)
```bash
rustree --diff old.json
# ./
# ├── [+] new_feature/
# │   ├── [+] mod.rs (2.1 KB)
# │   └── [+] tests.rs (1.5 KB)
# ├── [-] deprecated/
# ├── [~] renamed.rs ← original.rs
# └── [M] src/
#     ├── [+] new_module.rs
#     └── [-] old_module.rs
# 
# Changes Summary:
#   3 files added (+)
#   2 files removed (-)
#   1 files moved/renamed (~)
```

#### Markdown Format
```bash
rustree --diff old.json --output-format markdown
# # Directory Changes
# 
# ## Added Files (+)
# - `new_feature/mod.rs` (2.1 KB)
# - `new_feature/tests.rs` (1.5 KB)
# 
# ## Removed Files (-)
# - `deprecated/` (entire directory)
# 
# ## Moved/Renamed (~)
# - `renamed.rs` ← was `original.rs`
```

#### JSON Format
```bash
rustree --diff old.json --output-format json
# {
#   "diff_summary": {
#     "added": 3,
#     "removed": 2,
#     "moved": 1,
#     "compared_at": "2024-01-15T10:30:00Z"
#   },
#   "changes": [...]
# }
```

### Reuse All Existing Features

The diff feature works seamlessly with all existing options:

```bash
# Filtering
rustree --diff old.json --filter-include "*.rs" --use-gitignore-rules

# Depth control
rustree --diff old.json --depth 3

# Path scope  
rustree --diff old.json src/

# Metadata
rustree --diff old.json --show-size-bytes --show-last-modified --full-path

# Sorting
rustree --diff old.json --sort-by size --dirs-first

# LLM integration
rustree --diff old.json --llm-ask "What are the most significant changes?"
rustree --diff old.json --llm-export "code-review" > review.md

# Combined complex example
rustree --diff yesterday.json \
  --filter-include "src/**" "docs/**" \
  --filter-exclude "*.tmp" \
  --show-size-bytes \
  --sort-by mod_time \
  --human-friendly \
  --output-format markdown \
  --llm-ask "Analyze the impact of these changes"
```

## For Developers

### Code formatter and linter

```sh
# formatter
cargo fmt

# linter
cargo clippy
```

### Run code

```sh
cargo run
```

### Run tests

```sh
# Run all tests
cargo test

# Run only library tests (unit and integration)
cargo test --lib

# Run tests specifically for the binary (if you had any, typically less common unless the binary itself has complex logic not in the lib)
cargo test --bin rustree

# Run a specific test function
cargo test my_test_function_name

# Run all tests in that module
cargo test core::analyzer::file_stats
```# Test change for smart-pr
