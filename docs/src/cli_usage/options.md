# Command-Line Options

This page details all available command-line options for `rustree`.

The basic syntax is:
`rustree [OPTIONS] [PATH]`

If `PATH` is omitted, it defaults to the current directory (`.`).

## Path Argument

- `[PATH]`
  - Description: The directory or file path to process. If omitted, defaults to the current directory (`.`).
  - Example: `rustree ./my_project`, `rustree /var/log/syslog`

## Listing Control

- `-a, --include-hidden`
  - Description: Include hidden files and directories (those starting with a `.`) in the listing. (Original `tree` flag: `-a`)
  - Example: `rustree -a`

- `-d, --directory-only`
  - Description: List directories only. Files will not be included in the output. (Original `tree` flag: `-d`)
  - Example: `rustree -d ./src`

- `-L, --depth <LEVEL>`
  - Description: Descend only `<LEVEL>` directory levels deep. `1` means the root and its direct children. (Original `tree` flag: `-L`)
  - Example: `rustree -L 2` (shows root and its direct children)

- `-f, --full-path`
  - Description: Print the full path prefix for each file. (Original `tree` flag: `-f`)
  - Example: `rustree -f`

## Filtering and Ignoring

- `-P, --filter-include <PATTERN>`
  - Description: List only those files and directories that match the specified wildcard pattern. This option can be used multiple times to provide several patterns. If any pattern matches, the entry is listed. (Original `tree` flag: `-P`)
  - Wildcard patterns supported:
    - `*`: any zero or more characters.
    - `?`: any single character.
    - `[...]`: any single character listed (e.g., `[abc]`, `[a-z]`).
    - `[!...]`: any single character not listed.
    - `|`: separates alternate patterns within a single pattern string (e.g., `*.txt|*.log`).
  - A `/` at the end of a pattern (e.g., `mydir/`) specifically matches directories.
  - Note: To match hidden files (starting with `.`) with patterns like `*`, you must also use the `-a` or `--include-hidden` option. If `-a` is not used, `*` will not match hidden entries. Patterns explicitly starting with `.` (e.g., `.*`) will match hidden files regardless of `-a`.
  - This option is affected by `--case-insensitive-filter`.
  - Example: `rustree -P "*.rs"`, `rustree --filter-include "*.txt|*.md" --filter-include "docs/"`

- `-I, --filter-exclude <PATTERN>`
  - Description: Do not list those files or directories that match the specified wildcard pattern. This option can be used multiple times to provide several patterns. If any pattern matches, the entry is excluded. (Original `tree` flag: `-I`)
  - Uses the same wildcard pattern syntax as `-P, --filter-include`.
  - This option is affected by `--case-insensitive-filter`.
  - Example: `rustree -I "*.log"`, `rustree --filter-exclude "target/" --filter-exclude "*.tmp"`

- `--use-gitignore-rules`, `--gitignore` (deprecated alias)
  - Description: Respects gitignore rules for filtering files and directories. This includes checking `.gitignore` files in the scanned directories and their parents, the global gitignore file (e.g., `~/.config/git/ignore`), and repository-specific exclude files (e.g., `$GIT_DIR/info/exclude`). The `--gitignore` flag is deprecated; use `--use-gitignore-rules` instead.
  - This option is affected by `--case-insensitive-filter`.
  - Example: `rustree --use-gitignore-rules`

- `--gitignore-file <FILE>`
  - Description: Use the specified file(s) as additional sources of gitignore patterns. Patterns in these files are matched as if the specified file was located at the root of the scan. This option can be specified multiple times.
  - This option is affected by `--case-insensitive-filter`.
  - Example: `rustree --gitignore-file ./.customignore --gitignore-file ./project.ignores`

- `--case-insensitive-filter`
  - Description: Perform case-insensitive matching for all patterns provided via `-P` (`--filter-include`), `-I` (`--filter-exclude`), `--use-gitignore-rules` (and its alias `--gitignore`), and `--gitignore-file`.
  - Example: `rustree -P "*.TXT" --case-insensitive-filter` (would match `file.txt`)

- `--prune-empty-directories`, `--prune` (alias)
  - Description: Remove empty directories from the output. An empty directory is one that contains no files and no non-empty subdirectories after all other filtering (e.g., `-P`, `-I`, gitignore rules) has been applied. This option is applied before sorting.
  - Example: `rustree --prune-empty-directories`, `rustree --prune ./my_project`

## Metadata Reporting

- `-s, --show-size-bytes`
  - Description: Report sizes of files and directories in bytes in the output. (Original `tree` flag: `-s`)
  - Example: `rustree -s` or `rustree --show-size-bytes`

- `-D, --show-last-modified`
  - Description: Report dates for files and directories. By default, this shows the last modification time (mtime). If sorting by change time (`-c` or `--sort-by ctime`), this flag will instead display the last status change time (ctime). (Original `tree` flag: `-D`)
  - Example: `rustree -D` or `rustree --show-last-modified`

## Content Analysis

- `--calculate-lines`
  - Description: Calculate and display line counts for files.
  - Example: `rustree --calculate-lines`

- `--calculate-words`
  - Description: Calculate and display word counts for files.
  - Example: `rustree --calculate-words`

- `--apply-function <FUNCTION_NAME>`
  - Description: Apply a built-in function to file or directory contents and display the result. When using the `cat` function, the tree structure is displayed first, followed by the contents of each file.
  - Available functions: 
    - **File functions** (work with file content):
      - `count-pluses`: Counts '+' characters in each file and displays the count in metadata
      - `cat`: Displays the full content of each file after the tree structure
    - **Directory functions** (work with directory children):
      - `count-files`: Counts the number of files in each directory
      - `count-dirs`: Counts the number of subdirectories in each directory
      - `size-total`: Calculates the total size of all files in each directory
      - `dir-stats`: Shows combined statistics (files, directories, total size) for each directory
  - Example: `rustree --apply-function cat`, `rustree --apply-function count-pluses`, `rustree --apply-function dir-stats`

- `--apply-include <PATTERN>`
  - Description: Apply the function only to files/directories matching the specified pattern. Can be used multiple times. Uses the same wildcard syntax as `--filter-include`.
  - Example: `rustree --apply-function count-pluses --apply-include "*.rs"`

- `--apply-exclude <PATTERN>`
  - Description: Do not apply the function to files/directories matching the specified pattern. Can be used multiple times. Uses the same wildcard syntax as `--filter-exclude`.
  - Example: `rustree --apply-function dir-stats --apply-exclude "target/*"`

- `--apply-include-from <FILE_PATH>`
  - Description: Read include patterns for apply-function from the specified file. One pattern per line. Lines starting with `#` and empty lines are ignored.
  - Example: `rustree --apply-function cat --apply-include-from ./include-patterns.txt`

- `--apply-exclude-from <FILE_PATH>`
  - Description: Read exclude patterns for apply-function from the specified file. One pattern per line. Lines starting with `#` and empty lines are ignored.
  - Example: `rustree --apply-function dir-stats --apply-exclude-from ./exclude-patterns.txt`

### Size-based Filtering (new)

- `--min-file-size <SIZE>`
  - Description: Include only files **at least** `<SIZE>` bytes. Directories are never filtered by size; only their child files are tested. Accepts optional suffixes `K`, `M`, `G` (base-1024) for kibibytes, mebibytes, and gibibytes. Examples: `10K`, `2M`, `1G`.
  - Example: `rustree --min-file-size 100K` (shows files ≥ 100 KiB)

- `--max-file-size <SIZE>`
  - Description: Include only files **no larger than** `<SIZE>` bytes. Same suffix rules as above.
  - Example: `rustree --max-file-size 2M` (shows files ≤ 2 MiB)

These flags can be combined to specify a size range, e.g. `--min-file-size 10K --max-file-size 1M`.

## Sorting

- `-U, --unsorted`
  - Description: Do not sort the output. List entries in directory order. Overrides other sort options. Equivalent to `--sort-by none`. (Original `tree` flag: `-U`)
  - Example: `rustree -U`

- `-t`
  - Description: Sort the output by last modification time (mtime) instead of alphabetically. Oldest first. Equivalent to `--sort-by mtime`. (Original `tree` flag: `-t`)
  - Example: `rustree -t`

- `-c`
  - Description: Sort the output by last status change time (ctime) instead of alphabetically. Oldest first. Equivalent to `--sort-by ctime`. (Original `tree` flag: `-c`)
  - Example: `rustree -c`

- `-v`
  - Description: Sort the output by version strings embedded in file names. (Original `tree` flag: `-v`)
  - Example: `rustree -v`

- `--sort-by <KEY>`
  - Description: Sort the output by the given key.
  - Possible keys:
    - `name` (default): Alphabetical sort.
    - `size`: Sort by file/directory size. Default is largest first.
    - `mtime`: Sort by modification time (oldest first).
    - `ctime`: Sort by status change time (oldest first).
    - `crtime`: Sort by creation time (oldest first). (May not be available on all systems/files.)
    - `version`: Sort by version strings in names.
    - `lines`: Sort by line count (requires `--calculate-lines`). Default is most lines first.
    - `words`: Sort by word count (requires `--calculate-words`). Default is most words first.
    - `custom`: Sort by the output of `--apply-function`.
    - `none`: No sorting (directory order).
  - Example: `rustree --sort-by size`, `rustree --sort-by mtime`

- `-r, --reverse-sort`
  - Description: Reverse the order of the sort.
  - Example: `rustree -t -r` (newest mtime first), `rustree --sort-by size -r` (smallest size first)

- `--dirs-first`
  - Description: List directories before files. More readable. This applies to all sorting modes and overrides the default mixing behavior. Conflicts with `--files-first`.
  - Example: `rustree --dirs-first`, `rustree --sort-by size --dirs-first`

- `--files-first`
  - Description: List files before directories. More readable. This applies to all sorting modes and overrides the default mixing behavior. Conflicts with `--dirs-first`.
  - Example: `rustree --files-first`, `rustree --sort-by mtime --files-first`

## Output Formatting

- `--output-format <FORMAT>`
  - Description: Specifies the output format.
  - Possible values: `text` (default), `markdown`, `json`.
  - Example: `rustree --output-format json | jq '.'`
  - When combined with LLM flags (`--llm-ask`, `--llm-export`, or `--dry-run`) the program emits a single JSON object that bundles both the tree and the LLM section.  This makes it trivial to post-process or archive the entire interaction.  Example:

    ```bash
    rustree -L 1 \
            --output-format json \
            --llm-ask "Summarise this repo" --dry-run | jq .
    ```

    yields

    ```json
    {
      "tree": [ { "type": "directory", "name": ".", "contents": [ … ] } ],
      "llm": {
        "dry_run": true,
        "request": { "provider": "openai", "model": "gpt-4", ... },
        "question": "Summarise this repo"
      }
    }
    ```

- `--no-summary-report`
  - Description: Omits printing of the file and directory report at the end of the tree listing. By default, `rustree` displays a summary line like "4 directories, 6 files" at the end of the output. This flag removes that summary line entirely.
  - Example: `rustree --no-summary-report`, `rustree --output-format markdown --no-summary-report`

- `--no-indent`
  - Description: Turn off file/directory indentation. (Original `tree` flag: `-i`)
  - Example: `rustree --no-indent`

## Miscellaneous

- `-V, --version`
  - Description: Print version information and exit.
  - Example: `rustree -V`

- `-h, --help`
  - Description: Print help information and exit.
  - Example: `rustree --help`

## LLM Integration

- `--llm-export <QUESTION>`
  - Description: Export a formatted query for external LLM tools. This preserves the original behavior of outputting specially formatted text for piping to external LLM command-line tools.
  - Example: `rustree --llm-export "Analyze this project structure" | claude-cli`

- `--llm-ask <QUESTION>`
  - Description: Ask a question directly to an LLM service. Requires API key configuration.
  - Example: `rustree --llm-ask "What's the architecture of this project?"`

- `--llm-provider <PROVIDER>`
  - Description: Choose the LLM provider for direct queries. Supported providers: `openai`, `anthropic`, `cohere`, `openrouter`.
  - Default: `openai`
  - Example: `rustree --llm-ask "Analyze this" --llm-provider anthropic`

- `--llm-model <MODEL>`
  - Description: Specify the model to use with the chosen provider.
  - Default models: OpenAI (`gpt-4`), Anthropic (`claude-3-sonnet-20240229`), Cohere (`command-r`), OpenRouter (`openai/gpt-4`)
  - Example: `rustree --llm-ask "Review this" --llm-model gpt-3.5-turbo`

- `--llm-api-key <KEY>`
  - Description: Provide API key via command line. Can also be set via environment variables or `.env` file.
  - Environment variables: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `COHERE_API_KEY`, `OPENROUTER_API_KEY`
  - Example: `rustree --llm-ask "Question" --llm-api-key "your-api-key"`

- `--llm-endpoint <URL>`
  - Description: Custom endpoint URL for self-hosted or proxy services.
  - Example: `rustree --llm-ask "Question" --llm-endpoint "https://api.custom.com/v1"`

- `--llm-temperature <FLOAT>`
  - Description: Control response randomness. Range: 0.0 (deterministic) to 2.0 (very random).
  - Default: `0.7`
  - Example: `rustree --llm-ask "Precise analysis" --llm-temperature 0.1`

- `--llm-max-tokens <INT>`
  - Description: Maximum number of tokens in the LLM response. Range: 1 to 32000.
  - Default: `1000`
  - Example: `rustree --llm-ask "Brief summary" --llm-max-tokens 200`

- `--llm-generate-env`
  - Description: Generate a sample `.env` file template with all supported API key variables.
  - Example: `rustree --llm-generate-env > .env`

- `--dry-run`
  - Description: Preview the LLM request without actually sending it. When used with `--llm-ask`, RusTree builds the full HTTP request that would be sent to the provider, displays it, and exits without making the API call. This is useful for debugging, cost estimation, and verifying the request structure before sending.
  - **Token Estimation**: Shows approximate token counts using a 4:1 character-to-token ratio for prompts and max_tokens setting for completion. These are rough estimates for planning purposes only - actual token usage may vary significantly based on content type and provider tokenization.
  - Example: `rustree --llm-ask "What is this project?" --dry-run`

- `--human-friendly`
  - Description: Format dry-run output in a human-friendly markdown format instead of the default plain text. This option requires `--dry-run` to be enabled. The markdown format provides better structure with organized sections for configuration, headers, messages, and JSON body.
  - Example: `rustree --llm-ask "Analyze this project" --dry-run --human-friendly`

  **Additional effect:** When combined with `--size` / `-s`, tree listings will display file sizes in a readable form (e.g. `1.2 MB` instead of `1234567B`).