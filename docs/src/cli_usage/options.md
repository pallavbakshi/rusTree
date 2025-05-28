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