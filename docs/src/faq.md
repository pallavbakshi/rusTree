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

A: This feature is planned but not yet implemented in the initial versions. For now, you can use shell globbing or tools like `grep` to filter the output if needed, or rely on the `--show-hidden` (`-a`) flag.

**Q: Can I customize the output format further?**

A: Currently, RusTree supports "text" and "markdown" formats. The text format has some implicit styling. More advanced customization (e.g., custom colors, icons, or entirely new formats like JSON) could be considered for future development. If you have specific needs, please open a feature request.

**Q: How does the `--llm-ask` feature work?**

A: The `--llm-ask` option formats the `rustree` output along with your question in a way that is convenient to pipe directly into a command-line Large Language Model (LLM) tool (like `ollama`, or scripts using OpenAI/Anthropic APIs). RusTree itself does not make any API calls to LLMs. It simply prepares the text.

Example:
```bash
rustree --llm-ask "Summarize this project structure" | ollama run mistral
```
This pipes the tree output and your question to the `ollama` tool running the `mistral` model.

**Q: Where can I find the API documentation for the library?**

A: You can generate it locally by running `cargo doc --open` in the project's root directory. If the crate is published to `crates.io`, the API documentation will also be available on `docs.rs`.