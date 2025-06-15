# Persistent Configuration Files

RusTree can read default options from **TOML** files, so you don’t have to
repeat long command-lines every time.

## Search order

1. Paths supplied via `--config-file <FILE>` (can be given multiple times; _last
   one wins_).
2. Per-project file `./.rustree/config.toml` (looked-up from the working dir).
3. Global file `$XDG_CONFIG_HOME/rustree/config.toml` (or `~/.config/rustree/…`
   on macOS / Linux).
4. Built-in defaults.

Disable project/global discovery with `--no-config`.

## File format

TOML sections map 1-to-1 to option groups:

```toml
[listing]
show_hidden = true
max_depth   = 2

[filtering]
match_patterns  = ["*.rs", "Cargo.*"]
ignore_patterns = ["target/*"]

[sorting]
sort_by = "size"
reverse = true      # largest files first

[metadata]
show_size_bytes    = true
calculate_line_count = true

[output]
format = "markdown"

[llm]
provider    = "openai"        # Supported: openai, anthropic, cohere, openrouter
model       = "gpt-4o"         # Optional - uses provider's default if not specified
api_key_env = "OPENAI_API_KEY" # Recommended – keeps secrets out of the file
# api_key     = "sk-..."       # Direct API key (not recommended in shared configs)
# endpoint    = "https://..."  # Custom endpoint for self-hosted or proxy services
# temperature = 0.7            # Model temperature (0.0-2.0)
# max_tokens  = 1000           # Maximum response tokens
```

Unknown keys are ignored but a warning is printed, so your config keeps working
after version upgrades.

## Creating a template

Run:

```bash
rustree --generate-config > .rustree/config.toml
```

Then open the file and uncomment / edit the options you want.

## Security notes

If you use `api_key_file`, make sure the file is **not** world-readable. RusTree
prints a warning when it detects permissive modes (`chmod 600 <file>` is a good
baseline).

## Precedence vs CLI

Values from configuration files are applied _before_ parsing the CLI, therefore
**command-line flags always override** TOML settings.

---

See the full option list in the next chapter or by running `rustree --help`.
