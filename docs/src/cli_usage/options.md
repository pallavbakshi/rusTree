## Options and Flags

This page lists the command-line options available for `rustree`. You can also view this information by running `rustree --help`.

### General Options

*   `[PATH]`
    *   Description: The root path to start scanning from.
    *   Default: `.` (current directory)

*   `-L, --max-depth <DEPTH>`
    *   Description: Maximum depth to scan into the directory tree. (Original `tree` flag: `-L`)
    *   Example: `rustree -L 2`

*   `-a, --all`
    *   Description: Show hidden files and directories (those starting with a `.`). (Original `tree` flag: `-a`)

*   `--output-format <FORMAT>`
    *   Description: Specifies the output format for the tree.
    *   Possible values: `text`, `markdown`
    *   Default: `text`
    *   Example: `rustree --output-format markdown`

*   `-d, --dirs-only`
    *   Description: List directories only. Files will be excluded from the output. (Original `tree` flag: `-d`)
    *   Example: `rustree -d`

### Filtering and Ignoring

*   `-P, --match-pattern <PATTERN>`
    *   Description: List only those files and directories that match the specified wildcard pattern. This option can be used multiple times to provide several patterns. If any pattern matches, the entry is listed. (Original `tree` flag: `-P`)
    *   Wildcard operators:
        *   `*`: any zero or more characters.
        *   `**`: any zero or more characters, including path separators (`/`).
        *   `?`: any single character.
        *   `[...]`: any single character listed (e.g., `[abc]`, `[a-z]`).
        *   `[^...]`: any single character *not* listed.
        *   `|`: separates alternate patterns within a single pattern string (e.g., `*.txt|*.log`).
    *   A `/` at the end of a pattern (e.g., `mydir/`) specifically matches directories.
    *   Note: To match hidden files (starting with `.`) with patterns like `*`, you must also use the `-a` or `--all` option. If `-a` is not used, `*` will not match hidden entries. Patterns explicitly starting with `.` (e.g., `.*`) will match hidden files regardless of `-a`.
    *   This option is affected by `--ignore-case`.
    *   Example: `rustree -P "*.rs"`, `rustree -P "*.txt|*.md" -P "docs/"`

*   `-I, --ignore-path <PATTERN>`
    *   Description: Do not list those files or directories that match the specified wildcard pattern. This option can be used multiple times to provide several patterns. If any pattern matches, the entry is excluded. (Original `tree` flag: `-I`)
    *   Uses the same wildcard pattern syntax as `-P, --match-pattern`.
    *   This option is affected by `--ignore-case`.
    *   Example: `rustree -I "*.log"`, `rustree -I "target/" -I "*.tmp"`

*   `--use-gitignore`
    *   Description: Respects gitignore rules for filtering files and directories. This includes checking `.gitignore` files in the scanned directories and their parents, the global gitignore file (e.g., `~/.config/git/ignore`), and repository-specific exclude files (e.g., `$GIT_DIR/info/exclude`).
    *   This option is affected by `--ignore-case`.
    *   Example: `rustree --use-gitignore`

*   `--git-ignore-files <FILE>`
    *   Description: Use the specified file(s) as additional sources of gitignore patterns. Patterns in these files are matched as if the specified file was located at the root of the scan. This option can be specified multiple times.
    *   This option is affected by `--ignore-case`.
    *   Example: `rustree --git-ignore-files ./.customignore --git-ignore-files ./project.ignores`

*   `--ignore-case`
    *   Description: Perform case-insensitive matching for all patterns provided via `-P` (`--match-pattern`), `-I` (`--ignore-path`), `--use-gitignore`, and `--git-ignore-files`.
    *   Example: `rustree -P "*.TXT" --ignore-case` (would match `file.txt`)

*   `-I, --ignore-path <PATTERN>`
    *   Description: Do not list those files or directories that match the specified wildcard pattern. This option can be used multiple times to provide several patterns. If any pattern matches, the entry is excluded. (Original `tree` flag: `-I`)
    *   Uses the same wildcard pattern syntax as `-P, --match-pattern`.
    *   Example: `rustree -I "*.log"`, `rustree -I "target/" -I "*.tmp"`

*   `--use-gitignore`
    *   Description: Respects gitignore rules for filtering files and directories. This includes checking `.gitignore` files in the scanned directories and their parents, the global gitignore file (e.g., `~/.config/git/ignore`), and repository-specific exclude files (e.g., `$GIT_DIR/info/exclude`).
    *   Example: `rustree --use-gitignore`

*   `--git-ignore-files <FILE>`
    *   Description: Use the specified file(s) as additional sources of gitignore patterns. Patterns in these files are matched as if the specified file was located at the root of the scan. This option can be specified multiple times.
    *   Example: `rustree --git-ignore-files ./.customignore --git-ignore-files ./project.ignores`

*   `--ignore-case`
    *   Description: Perform case-insensitive matching for all patterns provided via `-P` (`--match-pattern`), `-I` (`--ignore-path`), `--use-gitignore`, and `--git-ignore-files`.
    *   Example: `rustree -P "*.TXT" --ignore-case` (would match `file.txt`)

### Metadata Reporting

*   `-s, --report-sizes`
    *   Description: Report sizes of files in the output. (Original `tree` flag: `-s`)

*   `-D, --report-mtime`
    *   Description: Report last modification times for files and directories. (Original `tree` flag: `-D`)

### Content Analysis

*   `--calculate-lines`
    *   Description: Calculate and display line counts for files.

*   `-w, --calculate-words`
    *   Description: Calculate and display word counts for files.

*   `--apply-function <FUNCTION_NAME>`
    *   Description: Apply a built-in function to file contents and display the result.
    *   Possible values: `CountPluses` (more can be added)
    *   Example: `rustree --apply-function CountPluses`

### Sorting

*   `-t, --sort-by-mtime`
    *   Description: Sort the output by last modification time instead of alphabetically. (Original `tree` flag: `-t`)
    *   This option is mutually exclusive with `-U` and `--sort-key`.

*   `-U, --unsorted`
    *   Description: Do not sort. Lists files in directory order. (Original `tree` flag: `-U`)
    *   This option is mutually exclusive with `-t`, `--sort-key`, and `-r`.

*   `--sort-key <KEY>`
    *   Description: Specifies the key for sorting directory entries. If no sorting option (`-t`, `-U`, or `--sort-key`) is provided, `rustree` defaults to sorting by `name`.
    *   Possible values: `name`, `size`, `m-time` (equivalent to `-t`), `words`, `lines`, `custom`
    *   This option is mutually exclusive with `-t` and `-U`.
    *   Example: `rustree --sort-key size`

*   `-r, --reverse-sort`
    *   Description: Reverse the order of the active sort. (Original `tree` flag: `-r`)
    *   This option is ignored if `-U` (unsorted) is used.

### LLM Integration

*   `--llm-ask <QUESTION>`
    *   Description: Ask a question to an LLM, providing the `rustree` output as context. The output will be specially formatted for easy piping to an LLM tool.
    *   Example: `rustree --llm-ask "Summarize the Go files in this project." | your-llm-cli-tool`

### Help and Version

*   `-h, --help`
    *   Description: Print help information.

*   `-V, --version`
    *   Description: Print version information.