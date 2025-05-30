# CLI Usage Options

## Frequently Asked Questions (FAQ)

**Q: How is RusTree different from the standard `tree` command?**

A: While inspired by `tree`, RusTree aims to offer more:

- **Extensibility:** Built in Rust, allowing for easier addition of new features.
- **Multiple Output Formats:** Starting with text and Markdown, with potential for more (e.g., JSON).
- **Sorting Options:** More granular control over sorting.
- **Library Usage:** Can be used as a Rust library in other projects.

**Q: What are the system requirements?**

A: RusTree is built with Rust. To build it from source or install via `cargo install`, you'll need a Rust compiler and Cargo installed (see [rustup.rs](https://rustup.rs/)). Once compiled, the binary should be relatively portable across systems supported by Rust.

**Q: How do I report a bug or suggest a feature?**

A: Please open an issue on the [GitHub repository](https://github.com/yourusername/rustree) (replace with the actual link).

**Q: Is there a way to ignore certain files or directories (like `.gitignore`)?**

A: Yes, RusTree now offers several ways to ignore files and directories:

- **`--gitignore`**: This flag tells RusTree to respect standard gitignore behavior. It will look for `.gitignore` files in the current directory and parent directories, as well as global gitignore configurations (e.g., `~/.config/git/ignore` or `$XDG_CONFIG_HOME/git/ignore`) and repository-specific exclude files (e.g., `.git/info/exclude`).
- **`-I <PATTERN>` or `--filter-exclude <PATTERN>`**: This option allows you to specify glob patterns for files and directories that should be excluded from the output. You can use this option multiple times. It uses the same wildcard syntax as the `-P` option. For example, `rustree -I "*.log" -I "tmp/"` or `rustree --filter-exclude "*.log" --filter-exclude "tmp/"` will ignore all `.log` files and any directory named `tmp`.
- **`--git-ignore-files <FILE>`**: This option lets you specify one or more custom files that contain gitignore-style patterns. These patterns are applied as if the file was located at the root of the scan.
- **`--ignore-case`**: This flag makes all pattern matching (from `-P`/`--filter-include`, `-I`/`--filter-exclude`, `--gitignore`, and `--git-ignore-files`) case-insensitive.

These options can be combined. For example, you can use `--gitignore` and also add specific `-I` patterns.

**Q: How does the `-P pattern` / `--filter-include pattern` feature work?**

A: This feature allows you to list only files and directories whose names match one or more specified wildcard patterns.

- You can use options like `-P "*.txt"` or `--filter-include "*.txt"` to show only text files, or `-P "docs/"` to show only a directory named `docs`.
- Multiple patterns can be provided (e.g., `-P "*.rs" -P "*.toml"`) or combined with `|` (e.g., `-P "*.rs|*.toml"`).
- Supported wildcards include `*`, `**`, `?`, `[...]`, and `[^...]`.
- A `/` at the end of a pattern (e.g., `mydir/`) specifically matches directories.
- To match hidden files (starting with `.`) with general patterns like `*`, you must also use the `-a` (or `--include-hidden`) option. If `-a` is not used, `*` will not match hidden entries. Patterns explicitly starting with `.` (e.g., `.*`) will match hidden files regardless of `-a`.
- The matching can be made case-insensitive using the `--ignore-case` flag.
- The summary line (number of directories and files) will reflect only the listed items.

**Q: How does `--ignore-case` work?**

A: The `--ignore-case` flag makes all pattern matching operations case-insensitive. This applies to:

- Patterns specified with `-P` or `--filter-include`.
- Patterns specified with `-I` or `--filter-exclude`.
- Patterns found in `.gitignore` files when `--gitignore` is active.
- Patterns found in custom ignore files specified with `--git-ignore-files`.

For example, if `--ignore-case` is used, a pattern like `-P "*.JPG"` would match `image.jpg`, `image.JPG`, and `image.Jpg`. Similarly, an ignore pattern like `-I "README.MD"` would ignore `readme.md`.

**Q: Can I customize the output format further?**

A: Currently, RusTree supports "text" and "markdown" formats. The text format has some implicit styling. More advanced customization (e.g., custom colors, icons, or entirely new formats like JSON) could be considered for future development. If you have specific needs, please open a feature request.

**Q: How does the `--llm-ask` feature work?**

A: The `--llm-ask` option formats the `rustree` output along with your question in a way that is convenient to pipe directly into a command-line Large Language Model (LLM) tool (like `ollama`, or scripts using OpenAI/Anthropic APIs). RusTree itself does not make any API calls to LLMs. It simply prepares the text.

Example:

```bash
rustree --llm-ask "Summarize this project structure" | ollama run mistral
```

This pipes the tree output and your question to the `ollama` tool running the `mistral` model.

**Q: How does the `-d` (or `--directory-only`) flag work?**

A: When `-d` or `--directory-only` is used, RusTree will only list directories. All files will be excluded from the output.

- Symlinks pointing to directories are treated as directories and will be listed.
- Symlinks pointing to files (or broken symlinks) will be excluded.
- The summary line will reflect "X directories, 0 files".
- File-specific analysis options (like `--calculate-lines`, `--calculate-words`, `--apply-function`) will effectively be ignored as there are no files to analyze in the output.
- Metadata like size (`-s`) and modification time (`-D`) will apply to the listed directories.

**Q: If I use `-d` with `-s` (report sizes), will it show directory sizes?**

A: Yes. When `-d` (or `--directory-only`) and `-s` (or `--report-sizes`) are used together, RusTree will report the sizes of the directories themselves (as reported by the operating system, which might vary in meaning, e.g., size of metadata vs. total content size on some systems).

**Q: What happens if I use `-d` with file-specific sorting keys like `lines` or `words`?**

A: Since `-d` (or `--directory-only`) excludes files, sorting by file-specific attributes like line count or word count will not be meaningful. The sorting behavior in such cases might default to sorting by name or be unpredictable for those specific keys. It's recommended to use sort keys applicable to directories (e.g., `name`, `m-time`, `size` if `-s` is also used) when `-d` is active.

**Q: Where can I find the API documentation for the library?**

A: You can generate it locally by running `cargo doc --open` in the project's root directory. If the crate is published to `crates.io`, the API documentation will also be available on `docs.rs`.

## CLI Options Reference

### Filtering and Ignoring

- `-P, --filter-include <PATTERN>`

  - Description: List only those files and directories that match the specified wildcard pattern. This option can be used multiple times to provide several patterns. If any pattern matches, the entry is listed. (Original `tree` flag: `-P`)
  - Wildcard operators:
    - `*`: any zero or more characters.
    - `**`: any zero or more characters, including path separators (`/`).
    - `?`: any single character.
    - `[...]`: any single character listed (e.g., `[abc]`, `[a-z]`).
    - `[^...]`: any single character _not_ listed.
    - `|`: separates alternate patterns within a single pattern string (e.g., `*.txt|*.log`).
  - A `/` at the end of a pattern (e.g., `mydir/`) specifically matches directories.
  - Note: To match hidden files (starting with `.`) with patterns like `*`, you must also use the `-a` or `--include-hidden` option. If `-a` is not used, `*` will not match hidden entries. Patterns explicitly starting with `.` (e.g., `.*`) will match hidden files regardless of `-a`.
  - This option is affected by `--ignore-case`.
  - Example: `rustree -P "*.rs"`, `rustree --filter-include "*.txt|*.md" --filter-include "docs/"`

- `-I, --filter-exclude <PATTERN>`

  - Description: Do not list those files or directories that match the specified wildcard pattern. This option can be used multiple times to provide several patterns. If any pattern matches, the entry is excluded. (Original `tree` flag: `-I`)
  - Uses the same wildcard pattern syntax as `-P, --filter-include`.
  - This option is affected by `--ignore-case`.
  - Example: `rustree -I "*.log"`, `rustree --filter-exclude "target/" --filter-exclude "*.tmp"`

- `--gitignore`

  - Description: Respects gitignore rules for filtering files and directories. This includes checking `.gitignore` files in the scanned directories and their parents, the global gitignore file (e.g., `~/.config/git/ignore`), and repository-specific exclude files (e.g., `$GIT_DIR/info/exclude`).
  - This option is affected by `--ignore-case`.
  - Example: `rustree --gitignore`

- `--git-ignore-files <FILE>`

  - Description: Use the specified file(s) as additional sources of gitignore patterns. Patterns in these files are matched as if the specified file was located at the root of the scan. This option can be specified multiple times.
  - This option is affected by `--ignore-case`.
  - Example: `rustree --git-ignore-files ./.customignore --git-ignore-files ./project.ignores`

- `--ignore-case`
  - Description: Perform case-insensitive matching for all patterns provided via `-P` (`--filter-include`), `-I` (`--filter-exclude`), `--gitignore`, and `--git-ignore-files`.
  - Example: `rustree -P "*.TXT" --ignore-case` (would match `file.txt`)

### Metadata Reporting

- `-s, --report-sizes`

  - Description: Report sizes of files in the output. (Original `tree` flag: `-s`)

- `-D, --report-mtime`
  - Description: Report last modification times for files and directories. (Original `tree` flag: `-D`)

### Content Analysis

- `--calculate-lines`

  - Description: Calculate and display line counts for files.

- `-w, --calculate-words`

  - Description: Calculate and display word counts for files.

- `--apply-function <FUNCTION_NAME>`
  - Description: Apply a built-in function to file contents and display the result.
  - Possible values: `CountPluses` (more can be added)
  - Example: `rustree --apply-function CountPluses`

### Sorting

- `-t, --sort-by-mtime`

  - Description: Sort the output by last modification time instead of alphabetically. (Original `tree` flag: `-t`)
  - This option is mutually exclusive with `-U` (`--unsorted`) and `--sort-key`.

- `-U, --unsorted`

  - Description: Do not sort. Lists files in directory order. (Original `tree` flag: `-U`)
  - This option is mutually exclusive with `-t` (`--sort-by-mtime`), `--sort-key`, and `-r` (`--reverse-sort`).

- `--sort-key <KEY>`

  - Description: Specifies the key for sorting directory entries. If no sorting option (`-t`, `-U`, or `--sort-key`) is provided, `rustree` defaults to sorting by `name`.
  - Possible values: `name`, `size`, `m-time` (equivalent to `-t`), `words`, `lines`, `custom`
  - This option is mutually exclusive with `-t` (`--sort-by-mtime`) and `-U` (`--unsorted`).
  - Example: `rustree --sort-key size`

- `-r, --reverse-sort`
  - Description: Reverse the order of the active sort. (Original `tree` flag: `-r`)
  - This option is ignored if `-U` (`--unsorted`) is used.

### LLM Integration

- `--llm-ask <QUESTION>`
  - Description: Ask a question to an LLM, providing the `rustree` output as context. The output will be specially formatted for easy piping to an LLM tool.
  - Example: `rustree --llm-ask "Summarize the Go files in this project." | your-llm-cli-tool`

### Help and Version

- `-h, --help`

  - Description: Print help information.

- `-V, --version`
  - Description: Print version information.
