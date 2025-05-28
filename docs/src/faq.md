# Frequently Asked Questions (FAQ)

**Q: How is RusTree different from the standard `tree` command?**

A: While inspired by `tree`, RusTree aims to offer more:
*   **Extensibility:** Built in Rust, allowing for easier addition of new features.
*   **Analysis Features:** Built-in capabilities like line/word counts and potential for custom function application on file contents.
*   **Multiple Output Formats:** Starting with text and Markdown, with potential for more (e.g., JSON).
*   **Sorting Options:** More granular control over sorting.
*   **Library Usage:** Can be used as a Rust library in other projects.

**Q: What are the system requirements?**

A: RusTree is built with Rust. To build it from source or install via `cargo install`, you'll need a Rust compiler and Cargo installed (see [rustup.rs](https://rustup.rs/)). Once compiled, the binary should be relatively portable across systems supported by Rust.

**Q: How do I report a bug or suggest a feature?**

A: Please open an issue on the [GitHub repository](https://github.com/yourusername/rustree) (replace with the actual link).

**Q: Is there a way to ignore certain files or directories (like `.gitignore`)?**

A: This feature is planned but not yet implemented in the initial versions. For now, you can use shell globbing or tools like `grep` to filter the output if needed, or rely on the `--show-hidden` (`-a`) flag. The new `-P`/`--match-pattern` options provide powerful built-in filtering.

**Q: How does the `-P pattern` / `--match-pattern pattern` feature work?**

A: This feature allows you to list only files and directories whose names match one or more specified wildcard patterns.
    *   You can use options like `-P "*.txt"` to show only text files, or `-P "docs/"` to show only a directory named `docs`.
    *   Multiple patterns can be provided (e.g., `-P "*.rs" -P "*.toml"`) or combined with `|` (e.g., `-P "*.rs|*.toml"`).
    *   Supported wildcards include `*`, `**`, `?`, `[...]`, and `[^...]`.
    *   A `/` at the end of a pattern (e.g., `mydir/`) specifically matches directories.
    *   To include hidden files (starting with `.`) in the pattern matching process, you must also use the `-a` (or `--all`) option.
    *   The summary line (number of directories and files) will reflect only the listed items.

**Q: Can I customize the output format further?**

A: Currently, RusTree supports "text" and "markdown" formats. The text format has some implicit styling. More advanced customization (e.g., custom colors, icons, or entirely new formats like JSON) could be considered for future development. If you have specific needs, please open a feature request.

**Q: How does the `--llm-ask` feature work?**

A: The `--llm-ask` option formats the `rustree` output along with your question in a way that is convenient to pipe directly into a command-line Large Language Model (LLM) tool (like `ollama`, or scripts using OpenAI/Anthropic APIs). RusTree itself does not make any API calls to LLMs. It simply prepares the text.

Example:
```bash
rustree --llm-ask "Summarize this project structure" | ollama run mistral
```
This pipes the tree output and your question to the `ollama` tool running the `mistral` model.

**Q: How does the `-d` (or `--dirs-only`) flag work?**

A: When `-d` is used, RusTree will only list directories. All files will be excluded from the output.
    *   Symlinks pointing to directories are treated as directories and will be listed.
    *   Symlinks pointing to files (or broken symlinks) will be excluded.
    *   The summary line will reflect "X directories, 0 files".
    *   File-specific analysis options (like `--calculate-lines`, `--calculate-words`, `--apply-function`) will effectively be ignored as there are no files to analyze in the output.
    *   Metadata like size (`-s`) and modification time (`-D`) will apply to the listed directories.

**Q: If I use `-d` with `-s` (report sizes), will it show directory sizes?**

A: Yes. When `-d` and `-s` are used together, RusTree will report the sizes of the directories themselves (as reported by the operating system, which might vary in meaning, e.g., size of metadata vs. total content size on some systems).

**Q: What happens if I use `-d` with file-specific sorting keys like `lines` or `words`?**

A: Since `-d` excludes files, sorting by file-specific attributes like line count or word count will not be meaningful. The sorting behavior in such cases might default to sorting by name or be unpredictable for those specific keys. It's recommended to use sort keys applicable to directories (e.g., `name`, `m-time`, `size` if `-s` is also used) when `-d` is active.

**Q: Where can I find the API documentation for the library?**

A: You can generate it locally by running `cargo doc --open` in the project's root directory. If the crate is published to `crates.io`, the API documentation will also be available on `docs.rs`.