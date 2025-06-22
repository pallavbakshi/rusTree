# LLM Integration

RusTree includes powerful AI integration capabilities, allowing you to analyze project structures using Large Language Models (LLMs). You can either export formatted queries for external tools or directly interact with LLM services.

## Quick Start

### Basic LLM Query

Ask a question directly to an LLM service:

```bash
# Basic analysis with OpenAI (default provider)
rustree --llm-ask "What's the architecture of this project?"

# With environment variable for API key
export OPENAI_API_KEY="your-api-key"
rustree --llm-ask "Analyze the code organization"
```

### Generate API Key Template

Set up your environment variables:

```bash
# Generate .env template
rustree --llm-generate-env > .env

# Edit the .env file to add your API keys
# Then source it: source .env
```

## LLM Providers

### OpenAI (Default)

```bash
# Default OpenAI usage
rustree --llm-ask "Describe this project structure"

# Specify model explicitly
rustree --llm-ask "Code review analysis" --llm-model gpt-4

# Use GPT-3.5 for faster/cheaper analysis
rustree --llm-ask "Quick overview" --llm-model gpt-3.5-turbo
```

**Environment variable:** `OPENAI_API_KEY`

### Anthropic (Claude)

```bash
# Use Claude for analysis
rustree --llm-ask "What patterns do you see?" --llm-provider anthropic

# Specify Claude model
rustree --llm-ask "Security review" \
        --llm-provider anthropic \
        --llm-model claude-3-sonnet-20240229
```

**Environment variable:** `ANTHROPIC_API_KEY`

### Cohere

```bash
# Use Cohere for analysis
rustree --llm-ask "Summarize the codebase" --llm-provider cohere

# Specify Cohere model
rustree --llm-ask "Architecture analysis" \
        --llm-provider cohere \
        --llm-model command-r
```

**Environment variable:** `COHERE_API_KEY`

### OpenRouter (Multi-Provider)

```bash
# Use OpenRouter for access to multiple models
rustree --llm-ask "Compare this with best practices" \
        --llm-provider openrouter

# Use specific model through OpenRouter
rustree --llm-ask "Performance analysis" \
        --llm-provider openrouter \
        --llm-model anthropic/claude-3-haiku
```

**Environment variable:** `OPENROUTER_API_KEY`

## Configuration Options

### API Key Management

Multiple ways to provide API keys:

```bash
# Command line (not recommended for security)
rustree --llm-ask "Question" --llm-api-key "your-key"

# Environment variable (recommended)
export OPENAI_API_KEY="your-key"
rustree --llm-ask "Question"

# .env file (recommended for development)
echo 'OPENAI_API_KEY=your-key' >> .env
rustree --llm-ask "Question"
```

### Response Control

#### Temperature

Control response randomness (0.0 = deterministic, 2.0 = very random):

```bash
# Precise, deterministic analysis
rustree --llm-ask "Security audit" --llm-temperature 0.1

# Balanced analysis (default: 0.7)
rustree --llm-ask "Code review"

# Creative, varied analysis  
rustree --llm-ask "Suggest improvements" --llm-temperature 1.2
```

#### Response Length

Control maximum response length:

```bash
# Brief response
rustree --llm-ask "Quick summary" --llm-max-tokens 200

# Standard response (default: 1000)
rustree --llm-ask "Analyze architecture"

# Detailed analysis
rustree --llm-ask "Comprehensive review" --llm-max-tokens 3000
```

### Custom Endpoints

Use custom or self-hosted endpoints:

```bash
# Custom OpenAI-compatible endpoint
rustree --llm-ask "Analysis" \
        --llm-endpoint "https://api.custom.com/v1"

# Local AI service
rustree --llm-ask "Local analysis" \
        --llm-endpoint "http://localhost:8080/v1"
```

## Export for External Tools

### Generate Formatted Queries

Create formatted queries for external LLM tools:

```bash
# Export for external tools
rustree --llm-export "What are the main components?"

# Pipe to external LLM tools
rustree --llm-export "Code quality assessment" | claude-cli
rustree --llm-export "Security review" | ollama run mistral
rustree --llm-export "Performance analysis" > analysis-prompt.txt
```

### Export vs Direct Query

- **`--llm-export`**: Formats output for external tools, preserves original behavior
- **`--llm-ask`**: Direct integration with LLM services

## Request Preview and Debugging

### Dry Run Mode

Preview requests without making API calls:

```bash
# Preview what would be sent (no cost)
rustree --llm-ask "Analyze architecture" --dry-run

# Human-readable preview format
rustree --llm-ask "Security review" --dry-run --human-friendly

# Preview with specific settings
rustree --llm-ask "Performance analysis" \
        --llm-provider anthropic \
        --llm-model claude-3-haiku \
        --llm-temperature 0.3 \
        --dry-run --human-friendly
```

### Token Estimation

Dry run mode provides rough token estimates:

```bash
rustree --llm-ask "Detailed analysis" \
        --llm-max-tokens 2000 \
        --dry-run
```

**Note:** Token estimates use a 4:1 character-to-token ratio and are approximate. Actual usage may vary significantly.

## Combining with Tree Features

### Filtered Analysis

Analyze specific parts of your project:

```bash
# Analyze only source code
rustree --llm-ask "Review the code quality" \
        --filter-include "*.rs" \
        --filter-exclude "**/target/**"

# Focus on documentation
rustree --llm-ask "How well documented is this?" \
        --filter-include "*.md" "README*"

# Analyze configuration
rustree --llm-ask "Review configuration setup" \
        --filter-include "*.toml" "*.yml" "*.json"
```

### Depth-Limited Analysis

Control scope for large projects:

```bash
# High-level architecture analysis
rustree --llm-ask "What's the overall structure?" \
        --depth 2 --directory-only

# Detailed module analysis
rustree --llm-ask "Analyze module organization" \
        --depth 4 --filter-include "src/**"
```

### Metadata-Enhanced Analysis

Include file metadata in analysis:

```bash
# Size-aware analysis
rustree --llm-ask "Which files seem overly large?" \
        --show-size-bytes --human-friendly \
        --sort-by size --reverse-sort

# Complexity analysis
rustree --llm-ask "Identify complex modules" \
        --calculate-lines --calculate-words \
        --sort-by lines --reverse-sort

# Recent changes analysis
rustree --llm-ask "What was recently modified?" \
        --show-last-modified \
        --sort-by mtime --reverse-sort
```

## Advanced Use Cases

### Multi-Step Analysis

```bash
# Step 1: Preview and refine
rustree --llm-ask "Initial assessment" --dry-run --human-friendly

# Step 2: Export for review
rustree --llm-export "Refined question" > prompt-for-review.txt

# Step 3: Direct analysis
rustree --llm-ask "Focused analysis based on preview" \
        --llm-temperature 0.3 \
        --llm-max-tokens 1500

# Step 4: Compare with external tools
rustree --llm-export "Cross-check analysis" | external-llm-tool
```

### Different Providers for Different Tasks

```bash
# Quick overview with fast model
rustree --llm-ask "Project overview" \
        --llm-provider openai \
        --llm-model gpt-3.5-turbo

# Detailed security analysis with advanced model
rustree --llm-ask "Comprehensive security review" \
        --llm-provider anthropic \
        --llm-model claude-3-sonnet \
        --llm-temperature 0.2

# Creative improvement suggestions
rustree --llm-ask "How would you reorganize this?" \
        --llm-provider cohere \
        --llm-temperature 1.0
```

### JSON Output with LLM

Combine structured data with AI analysis:

```bash
# Get both tree data and analysis in JSON
rustree --output-format json \
        --llm-ask "Analyze the structure" \
        --dry-run | jq '.'

# Process tree data, then analyze
rustree --output-format json --show-size-bytes > data.json
rustree --llm-ask "Based on this data, what optimization opportunities exist?" \
        --from-tree-file data.json
```

## Workflow Examples

### Code Review Workflow

```bash
# 1. Get overview
rustree --llm-ask "What type of project is this?" --depth 2

# 2. Analyze structure
rustree --llm-ask "How is the code organized?" \
        --filter-include "src/**" --depth 3

# 3. Check for issues
rustree --llm-ask "Are there any potential problems?" \
        --show-size-bytes --calculate-lines \
        --sort-by size --reverse-sort

# 4. Get suggestions
rustree --llm-ask "What improvements would you suggest?" \
        --llm-temperature 0.8 --llm-max-tokens 2000
```

### Documentation Analysis

```bash
# Analyze documentation coverage
rustree --llm-ask "How well documented is this project?" \
        --filter-include "*.md" "*.txt" "README*" \
        --calculate-words

# Compare docs to code
rustree --llm-ask "Does the documentation match the code structure?" \
        --filter-include "*.md" "*.rs" \
        --depth 3
```

### Performance Analysis

```bash
# Find potential performance issues
rustree --llm-ask "What might cause performance problems?" \
        --show-size-bytes --human-friendly \
        --calculate-lines \
        --min-file-size 10K

# Analyze module complexity
rustree --llm-ask "Which modules are most complex?" \
        --apply-function dir-stats \
        --directory-only \
        --sort-by custom --reverse-sort
```

## Error Handling and Troubleshooting

### API Key Issues

```bash
# Check if API key is configured
rustree --llm-ask "test" --dry-run

# Generate template for missing keys
rustree --llm-generate-env
```

### Rate Limiting

If you hit rate limits:
- Use `--dry-run` to test without API calls
- Reduce `--llm-max-tokens` for smaller requests
- Use filtering to reduce tree size
- Switch to different provider/model

### Network Issues

```bash
# Test connectivity with dry run
rustree --llm-ask "test connection" --dry-run

# Use custom endpoint if needed
rustree --llm-ask "test" --llm-endpoint "https://alternative-endpoint.com"
```

## Quick Reference

### Core Options

| Option | Description |
|--------|-------------|
| `--llm-ask <QUESTION>` | Ask question directly to LLM service |
| `--llm-export <QUESTION>` | Export formatted query for external tools |
| `--llm-provider <PROVIDER>` | Choose provider (openai, anthropic, cohere, openrouter) |
| `--llm-model <MODEL>` | Specify model name |
| `--llm-api-key <KEY>` | Provide API key via command line |

### Configuration

| Option | Description |
|--------|-------------|
| `--llm-endpoint <URL>` | Custom endpoint URL |
| `--llm-temperature <FLOAT>` | Response randomness (0.0-2.0, default: 0.7) |
| `--llm-max-tokens <INT>` | Maximum response tokens (default: 1000) |
| `--llm-generate-env` | Generate .env template |

### Debugging

| Option | Description |
|--------|-------------|
| `--dry-run` | Preview request without sending it |
| `--human-friendly` | Format dry-run output in readable markdown |

### Environment Variables

| Variable | Provider |
|----------|----------|
| `OPENAI_API_KEY` | OpenAI |
| `ANTHROPIC_API_KEY` | Anthropic (Claude) |
| `COHERE_API_KEY` | Cohere |
| `OPENROUTER_API_KEY` | OpenRouter |

## Examples

See the [Examples](./examples.md) page for more detailed LLM integration scenarios and real-world use cases.