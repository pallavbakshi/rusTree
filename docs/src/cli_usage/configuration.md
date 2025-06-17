# Configuration

RusTree supports configuration files to save your preferred settings and avoid repetitive command-line arguments. This page covers configuration file usage, utilities, and advanced options.

## Configuration Files

### Automatic Discovery

RusTree automatically looks for configuration files in several locations:

1. **Project-specific**: `.rustree/config.toml` in current directory and parent directories
2. **Global user config**: `~/.config/rustree/config.toml` (Linux/macOS) or `%APPDATA%\rustree\config.toml` (Windows)
3. **Custom config files**: Specified via `--config-file`

### Configuration File Format

Configuration files use TOML format with sections corresponding to CLI option groups:

```toml
# Example .rustree/config.toml

[input_source]
root_display_name = "MyProject"

[listing]
max_depth = 3
show_hidden = true

[filtering]
ignore_patterns = ["target/", "node_modules/", "*.log"]
match_patterns = ["*.rs", "*.toml", "*.md"]
use_gitignore_rules = true
case_insensitive_filter = false

[metadata]
show_size_bytes = true
show_last_modified = true
calculate_lines = true

[sorting]
sort_by = "size"
reverse_sort = true
dirs_first = true

[misc]
output_format = "text"
no_summary_report = false
```

## Generate Configuration Template

### Create Template File

Generate a fully commented configuration template:

```bash
# Generate template and save to file
rustree --generate-config > .rustree/config.toml

# View template without saving
rustree --generate-config

# Generate in project directory
mkdir -p .rustree
rustree --generate-config > .rustree/config.toml
```

The generated template includes all available options with comments explaining their purpose.

## Custom Configuration Files

### Specify Configuration Files

Load specific configuration files:

```bash
# Load single custom config
rustree --config-file ./my-config.toml

# Load multiple configs (later ones override earlier ones)
rustree --config-file ./base-config.toml --config-file ./project-config.toml

# Combine with CLI options (CLI options override config)
rustree --config-file ./config.toml --depth 2 --show-size-bytes
```

### Disable Configuration Discovery

Skip automatic configuration file discovery:

```bash
# Ignore all config files
rustree --no-config

# Use only CLI options and built-in defaults
rustree --no-config --show-size-bytes --depth 3
```

## Configuration Precedence

Settings are applied in this order (later overrides earlier):

1. **Built-in defaults**
2. **Global user config** (`~/.config/rustree/config.toml`)
3. **Project configs** (`.rustree/config.toml`, searching up from current directory)
4. **Custom config files** (via `--config-file`, in order specified)
5. **Command-line options**

### Example Precedence

```bash
# This command:
rustree --config-file custom.toml --depth 5

# Results in:
# 1. Built-in defaults
# 2. ~/.config/rustree/config.toml (if exists)
# 3. .rustree/config.toml (if found in current/parent dirs)
# 4. custom.toml
# 5. --depth 5 (overrides any depth setting from configs)
```

## Configuration Sections

### Input Source Options

```toml
[input_source]
root_display_name = "MyProject"  # Custom name for root directory
root_is_directory = true         # Whether root should be treated as directory
```

### Listing Options

```toml
[listing]
max_depth = 3                    # Maximum traversal depth
show_hidden = false              # Include hidden files/directories
```

### Filtering Options

```toml
[filtering]
ignore_patterns = [              # Files/dirs to exclude
    "target/",
    "node_modules/", 
    "*.log",
    "*.tmp"
]
match_patterns = [               # Files/dirs to include (if specified)
    "*.rs",
    "*.toml", 
    "*.md"
]
use_gitignore_rules = true       # Respect .gitignore files
case_insensitive_filter = false  # Case-insensitive pattern matching
min_file_size = "1K"             # Minimum file size (with units)
max_file_size = "10M"            # Maximum file size (with units)
prune_empty_directories = true   # Remove empty directories
```

### Metadata Options

```toml
[metadata]
show_size_bytes = true           # Display file sizes
show_last_modified = true        # Display modification times
calculate_lines = false          # Count lines in text files
calculate_words = false          # Count words in text files

# Apply function configuration
apply_function = "cat"           # Built-in function to apply
# apply_function_cmd = "wc -l"   # External command to apply
apply_include_patterns = ["*.rs"] # Apply function only to these patterns
apply_exclude_patterns = ["*test*"] # Don't apply function to these patterns
```

### Sorting Options

```toml
[sorting]
sort_by = "name"                 # Sort key (name, size, mtime, etc.)
reverse_sort = false             # Reverse sort order
dirs_first = false               # Show directories before files
files_first = false              # Show files before directories
```

### Miscellaneous Options

```toml
[misc]
output_format = "text"           # Output format (text, markdown, json, html)
no_summary_report = false        # Omit summary line
human_friendly = false           # Human-readable sizes
```

## Verbose Configuration Display

### View Merged Configuration

See exactly what configuration RusTree will use:

```bash
# Show merged configuration before execution
rustree --verbose

# Combine with other options to see their effect
rustree --verbose --config-file custom.toml --depth 2
```

This displays:
- All configuration sources found
- Final merged configuration values
- Command-line overrides applied

**Example output:**
```
Configuration loaded from:
  - Built-in defaults
  - ~/.config/rustree/config.toml
  - ./.rustree/config.toml
  - Command line: --depth 2

Final configuration:
[listing]
max_depth = 2                    # from command line
show_hidden = true               # from ./.rustree/config.toml

[filtering] 
ignore_patterns = ["target/"]    # from ~/.config/rustree/config.toml
...

[Running rustree with above configuration]
```

## Shell Completions

### Generate Completion Scripts

RusTree can generate shell completion scripts:

```bash
# Bash completions
rustree --generate-completions bash > ~/.local/share/bash-completion/completions/rustree

# Zsh completions  
rustree --generate-completions zsh > ~/.zfunc/_rustree

# Fish completions
rustree --generate-completions fish > ~/.config/fish/completions/rustree.fish

# PowerShell completions
rustree --generate-completions powershell > rustree_completions.ps1

# Elvish completions
rustree --generate-completions elvish > rustree_completions.elv
```

### Install Completions

After generating completions, you may need to reload your shell or source the completion files.

## Project-Specific Configuration

### Per-Project Settings

Create project-specific configurations:

```bash
# In your project root
mkdir -p .rustree

# Generate base config
rustree --generate-config > .rustree/config.toml

# Edit for project needs
# For example, a Rust project might use:
```

```toml
[filtering]
ignore_patterns = ["target/", "Cargo.lock"]
match_patterns = ["*.rs", "*.toml"]
use_gitignore_rules = true

[metadata]
show_size_bytes = true
calculate_lines = true

[sorting]
sort_by = "lines"
reverse_sort = true
```

### Team Configuration

Share configuration across team members:

```bash
# Commit project config to version control
git add .rustree/config.toml
git commit -m "Add RusTree project configuration"

# Team members automatically get the config
git pull
rustree  # Uses project config automatically
```

## Environment-Specific Configurations

### Development vs Production

Use different configs for different environments:

```bash
# Development config with detailed output
rustree --config-file .rustree/dev-config.toml

# Production config with minimal output
rustree --config-file .rustree/prod-config.toml

# CI/CD config for automated reporting
rustree --config-file .rustree/ci-config.toml --output-format json
```

### User-Specific Overrides

Combine global and project configs with user preferences:

```bash
# User's global config provides defaults
# ~/.config/rustree/config.toml

# Project config provides project-specific settings  
# .rustree/config.toml

# User overrides with personal config for this project
rustree --config-file ~/.rustree-overrides.toml
```

## Configuration Examples

### Source Code Analysis

```toml
# .rustree/source-analysis.toml
[filtering]
match_patterns = ["*.rs", "*.js", "*.py", "*.java", "*.cpp"]
ignore_patterns = ["**/target/**", "**/node_modules/**", "**/__pycache__/**"]
use_gitignore_rules = true

[metadata]
show_size_bytes = true
calculate_lines = true
calculate_words = true

[sorting]
sort_by = "lines" 
reverse_sort = true

[misc]
output_format = "markdown"
```

### Documentation Review

```toml
# .rustree/docs-config.toml
[filtering]
match_patterns = ["*.md", "*.txt", "README*", "*.rst"]
ignore_patterns = ["**/target/**", "**/build/**"]

[metadata]
show_size_bytes = true
calculate_words = true

[sorting]
sort_by = "words"
reverse_sort = true

[misc]
output_format = "markdown"
no_summary_report = true
```

### Quick Overview

```toml
# .rustree/quick.toml
[listing]
max_depth = 2

[filtering]
ignore_patterns = [".*", "**/target/**", "**/node_modules/**"]

[sorting]
dirs_first = true

[misc]
no_summary_report = true
```

## Troubleshooting Configuration

### Common Issues

**Config not loading:**
```bash
# Check what configs are found
rustree --verbose

# Verify config file syntax
rustree --config-file problematic-config.toml --verbose
```

**Option conflicts:**
```bash
# See final merged configuration
rustree --verbose

# Test with no config to isolate CLI options
rustree --no-config --your-options-here
```

**Path issues:**
```bash
# Check current directory for .rustree/config.toml
ls -la .rustree/

# Check global config location
ls -la ~/.config/rustree/
```

## Quick Reference

### Configuration Management

| Option | Description |
|--------|-------------|
| `--config-file <FILE>` | Load specific configuration file |
| `--no-config` | Ignore all configuration files |
| `--generate-config` | Generate configuration template |
| `--verbose` | Show merged configuration before execution |

### Shell Completions

| Option | Description |
|--------|-------------|
| `--generate-completions <SHELL>` | Generate completion script for shell |

### Supported Shells
- `bash` - Bash shell completions
- `zsh` - Zsh shell completions  
- `fish` - Fish shell completions
- `powershell` - PowerShell completions
- `elvish` - Elvish shell completions

### Configuration File Locations

1. **Project**: `.rustree/config.toml` (current and parent directories)
2. **Global**: `~/.config/rustree/config.toml` (Linux/macOS)
3. **Global**: `%APPDATA%\rustree\config.toml` (Windows)
4. **Custom**: Via `--config-file`

## Examples

See the [Examples](./examples.md) page for more configuration scenarios and real-world setups.