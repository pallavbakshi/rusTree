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
  - Description: Apply a built-in function to file contents and display the result.
  - Available functions: `CountPluses` (example function, counts '+' characters). More can be added.
  - Example: `rustree --apply-function CountPluses`

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

- `--files-first`
  - Description: When sorting by size, list all files and symlinks before directories. This is the default behavior for size sort.
  - Example: `rustree --sort-by size --files-first` (explicitly stating default)

- `--no-files-first`
  - Description: When sorting by size, intermingle files, symlinks, and directories based purely on their size.
  - Example: `rustree --sort-by size --no-files-first`

## Output Formatting

- `--output-format <FORMAT>`
  - Description: Specifies the output format.
  - Possible values: `text` (default), `markdown`.
  - Example: `rustree --output-format markdown`

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

- `--llm-ask <PROMPT>`
  - Description: Prepend the tree output with a specific prompt string, useful for piping to Large Language Models (LLMs). The prompt is followed by "--- TREE ---" and then the actual tree output.
  - Example: `rustree --llm-ask "Summarize this project structure:" | ollama run mistral`