# CLI Options Reference

This page provides a comprehensive reference for all command-line options available in `rustree`.

## General Usage

```bash
rustree [OPTIONS] [PATH]
```

- `[OPTIONS]`: Various flags to control behavior (e.g., depth, sorting, output format).
- `[PATH]`: Optional path to the directory to scan. Defaults to the current directory (`.`).

## Display Options

- `-a, --include-hidden`

  - Description: Include hidden files and directories (those starting with a `.`) in the output. (Original `tree` flag: `-a`)
  - Example: `rustree -a`

- `-d, --directory-only`

  - Description: List directories only. Files will be excluded. Symlinks pointing to directories are treated as directories. (Original `tree` flag: `-d`)
  - Example: `rustree -d`

- `-L, --depth <LEVEL>`

  - Description: Descend only `<LEVEL>` directory levels deep. `0` means display only the root. (Original `tree` flag: `-L`)
  - Example: `rustree -L 2` (shows root and its direct children)

- `-f, --full-path`

  - Description: Print the full path prefix for each file. (Original `tree` flag: `-f`)
  - Example: `rustree -f`

- `-o, --output-format <FORMAT>`
  - Description: Specifies the output format.
  - Possible values: `text` (default), `markdown`.
  - Example: `rustree --output-format markdown`

## Filtering and Ignoring

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

## Metadata Reporting

- `-s, --report-sizes`

  - Description: Report sizes of files and directories in the output. (Original `tree` flag: `-s`)
  - Example: `rustree -s`

- `-D, --date`
  - Description: Report dates for files and directories. By default, this shows the last modification time (mtime). If sorting by change time (`-c` or `--sort-by ctime`), this flag will instead display the last status change time (ctime). (Original `tree` flag: `-D`)
  - Example: `rustree -D`

## Content Analysis

- `--calculate-lines`

  - Description: Calculate and display line counts for files.
  - Example: `rustree --calculate-lines`

- `-w, --calculate-words`

  - Description: Calculate and display word counts for files.
  - Example: `rustree --calculate-words`

- `--apply-function <FUNCTION_NAME>`
  - Description: Apply a built-in function to file contents and display the result.
  - Possible values: `CountPluses` (more can be added).
  - Example: `rustree --apply-function CountPluses`

## Sorting

- `-v`

  - Description: Sort the output by version name (e.g., `file-1.2.0` before `file-1.10.0`). Uses an improved natural version sorting algorithm. (Original `tree` flag: `-v`)
  - This option is mutually exclusive with `-t`, `-c`, `-U`, and `--sort-by`.
  - Example: `rustree -v`

- `-t`

  - Description: Sort the output by last modification time (mtime) instead of alphabetically. Oldest first. (Original `tree` flag: `-t`)
  - This option is mutually exclusive with `-v`, `-c`, `-U`, and `--sort-by`.
  - Example: `rustree -t`

- `-c`

  - Description: Sort the output by last status change time (ctime) instead of alphabetically. Oldest first. If `-D` (or `--date`) is also used, `-D` will display change times instead of modification times. (Original `tree` flag: `-c`)
  - This option is mutually exclusive with `-v`, `-t`, `-U`, and `--sort-by`.
  - Example: `rustree -c`

- `-U, --unsorted`

  - Description: Do not sort. Lists files in directory order. (Original `tree` flag: `-U`)
  - This option is mutually exclusive with `-v`, `-t`, `-c`, `--sort-by`, and `-r` (`--reverse-sort`).
  - Example: `rustree -U`

- `-S, --sort-by <FIELD>`

  - Description: Specifies the field for sorting directory entries. If no sorting option (`-v`, `-t`, `-c`, `-U`, or `--sort-by`) is provided, `rustree` defaults to sorting by file size (descending). Default sort order for time-based fields is oldest first. For size, it's largest first.
  - Possible values for `<FIELD>`:
    - `name`: Sort by entry name (ascending).
    - `version` (alias `ver`): Sort by version string (e.g., `file_v1.2.txt` before `file_v1.10.txt`). Uses an improved natural version sorting. Equivalent to `-v`.
    - `size`: Sort by size (default, descending). By default, files/symlinks are grouped before directories. Within these groups, entries are sorted by size (largest first), then by name (ascending). Use with `-r` for smallest first and to reverse the group order (directories first, sorted by size ascending).
    - `mtime` (aliases `m`, `mod_time`): Sort by last modification time (oldest first). Equivalent to `-t`.
    - `ctime` (aliases `c`, `change_time`): Sort by last status change time (oldest first). Equivalent to `-c`.
    - `crtime` (aliases `cr`, `create_time`): Sort by creation time (oldest first).
    - `words`: Sort by word count (for files, ascending).
    - `lines`: Sort by line count (for files, ascending).
    - `custom`: Sort by the output of a custom applied function (ascending).
    - `none` (alias `n`): No sorting; preserve directory order. Equivalent to `-U`.
  - This option is mutually exclusive with `-v`, `-t`, `-c`, and `-U`.
  - Example: `rustree --sort-by size`, `rustree -S mtime`

- `-r, --reverse-sort`
  - Description: Reverse the order of the active sort. For example, if sorting by `mtime` (oldest first), `-r` makes it newest first. If sorting by `size` (largest first, files before dirs), `-r` makes it smallest first and directories before files (within dirs, smallest first). (Original `tree` flag: `-r`)
  - This option is ignored if `-U` (`--unsorted`) or `--sort-by none` is used.
  - Example: `rustree -t -r` (sorts by mtime, newest first)

## LLM Integration

- `--llm-ask <QUESTION>`
  - Description: Ask a question to an LLM, providing the `rustree` output as context. The output will be specially formatted for easy piping to an LLM tool.
  - Example: `rustree --llm-ask "Summarize the Go files in this project." | your-llm-cli-tool`

## Help and Version

- `-h, --help`

  - Description: Print help information.

- `-V, --version`
  - Description: Print version information.
