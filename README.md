
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
- `--include <pattern>`: Include files matching pattern
- `--exclude <pattern>`: Exclude files matching pattern

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
  --include "*.rs" --exclude "**/target/**" --llm-provider anthropic
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
