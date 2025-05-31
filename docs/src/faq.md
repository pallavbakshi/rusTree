# Frequently Asked Questions (FAQ)

**Q: How do I ignore files and directories?**
A: RusTree offers several ways:

- **`-I <PATTERN>` or `--filter-exclude <PATTERN>`**: Exclude files/directories matching the glob pattern. This can be used multiple times.
- **`--use-gitignore-rules` (alias: `--gitignore`)**: This flag tells RusTree to respect standard gitignore behavior. It will look for `.gitignore` files in the current directory and parent directories, as well as global gitignore configurations (e.g., `~/.config/git/ignore` or `$XDG_CONFIG_HOME/git/ignore`) and repository-specific exclude files (e.g., `.git/info/exclude`). Note: `--gitignore` is a deprecated alias.
- **`--gitignore-file <FILE>`**: This option lets you specify one or more custom files that contain gitignore-style patterns. These patterns are applied as if the file was located at the root of the scan.
- **`--case-insensitive-filter`**: This flag makes all pattern matching (from `-P`/`--filter-include`, `-I`/`--filter-exclude`, `--use-gitignore-rules`, and `--gitignore-file`) case-insensitive.

These options can be combined. For example, you can use `--use-gitignore-rules` and also add specific `-I` patterns.

**Q: How does the `-P` (or `--filter-include`) pattern matching work?**
A: The `-P <PATTERN>` or `--filter-include <PATTERN>` option allows you to specify wildcard patterns to list only matching files and directories.

- Wildcard patterns supported:
  - `*`: any zero or more characters.
  - `?`: any single character.
  - `[...]`: any single character listed (e.g., `[abc]`, `[a-z]`).
  - `[!...]`: any single character not listed.
  - `|`: separates alternate patterns within a single pattern string (e.g., `*.txt|*.log`).
- A `/` at the end of a pattern (e.g., `mydir/`) specifically matches directories.
- Note: To match hidden files (starting with `.`) with patterns like `*`, you must also use the `-a` or `--include-hidden` option. If `-a` is not used, `*` will not match hidden entries. Patterns explicitly starting with `.` (e.g., `.*`) will match hidden files regardless of `-a`.
- The matching can be made case-insensitive using the `--case-insensitive-filter` flag.

**Q: How does `--case-insensitive-filter` work?**
A: The `--case-insensitive-filter` flag makes all pattern matching operations case-insensitive. This applies to:

- Patterns specified with `-P <PATTERN>` or `--filter-include <PATTERN>`.
- Patterns specified with `-I <PATTERN>` or `--filter-exclude <PATTERN>`.
- Patterns found in `.gitignore` files when `--use-gitignore-rules` (or its alias `--gitignore`) is active.
- Patterns found in custom ignore files specified with `--gitignore-file`.

For example, if `--case-insensitive-filter` is used, a pattern like `-P "*.JPG"` would match `image.jpg`, `image.JPG`, and `image.Jpg`. Similarly, an ignore pattern like `-I "README.MD"` would ignore `readme.md`.

**Q: If I use `-d` with `-s` (show size in bytes), will it show directory sizes?**
A: Yes. When `-d` (or `--directory-only`) and `-s` (or `--show-size-bytes`) are used together, RusTree will report the sizes of the directories themselves (as reported by the operating system, which might vary in meaning, e.g., size of metadata vs. total content size on some systems).

Similarly, if `-D` (or `--show-last-modified`) is used with `-d`, it will show the relevant date (modification or change time, depending on whether `-c` is also active) for the directories.

**Q: How does the `-D` (or `--show-last-modified`) flag interact with `-c` (sort by change time)?**
A:

- If you use `-D` (or `--show-last-modified`) alone, it displays the last modification time (mtime).
- If you use `-c` alone, it sorts by change time (ctime), but `-D` (or `--show-last-modified`) is needed to _display_ a time.
- If you use both `-D` (or `--show-last-modified`) and `-c` (or `-D` and `--sort-by ctime`), then `-D` (or `--show-last-modified`) will display the last status change time (ctime) instead of the modification time. This allows you to see the ctime for entries when sorting by ctime.
