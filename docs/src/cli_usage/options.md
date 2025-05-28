## Options and Flags

This page lists the command-line options available for `rustree`. You can also view this information by running `rustree --help`.

### General Options

*   `[PATH]`
    *   Description: The root path to start scanning from.
    *   Default: `.` (current directory)

*   `-L, --max-depth <DEPTH>`
    *   Description: Maximum depth to scan into the directory tree.
    *   Example: `rustree -L 2`

*   `-a, --all`
    *   Description: Show hidden files and directories (those starting with a `.`).

*   `--output-format <FORMAT>`
    *   Description: Specifies the output format for the tree.
    *   Possible values: `text`, `markdown`
    *   Default: `text`
    *   Example: `rustree --output-format markdown`

### Metadata Reporting

*   `-s, --report-sizes`
    *   Description: Report sizes of files in the output.

*   `-D, --report-mtime`
    *   Description: Report last modification times for files and directories.

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

*   `--sort-key <KEY>`
    *   Description: Specifies the key for sorting directory entries.
    *   Possible values: `name`, `size`, `m-time`, `words`, `lines`, `custom`
    *   Default (if not specified): `name`
    *   Example: `rustree --sort-key size`

*   `-r, --reverse-sort`
    *   Description: Reverse the order of the sort specified by `sort_key`.

### LLM Integration

*   `--llm-ask <QUESTION>`
    *   Description: Ask a question to an LLM, providing the `rustree` output as context. The output will be specially formatted for easy piping to an LLM tool.
    *   Example: `rustree --llm-ask "Summarize the Go files in this project." | your-llm-cli-tool`

### Help and Version

*   `-h, --help`
    *   Description: Print help information.

*   `-V, --version`
    *   Description: Print version information.