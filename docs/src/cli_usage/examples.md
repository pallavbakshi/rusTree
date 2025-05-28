## Examples

Here are some practical examples of how to use `rustree` from the command line.

1. **Basic tree of the current directory:**

   ```bash
   rustree
   ```

2. **Tree of a specific directory, showing hidden files and up to depth 2:**

   ```bash
   rustree -a -L 2 /var/log
   ```

3. **List files in `~/Documents`, showing sizes and modification times, sorted by modification time (newest first using `-t` and `-r`):**

   ```bash
   rustree -s -D -t -r ~/Documents
   ```

4. **Analyze a source code project, showing line counts and word counts, sorted by line count (largest first):**

   ```bash
   rustree --calculate-lines --calculate-words --sort-key lines -r ./my_project_src
   ```

5. **List directories only in the current path:**

   ```bash
   rustree -d
   ```

6. **List directories only in `./src`, showing sizes, up to depth 1:**

   ```bash
   rustree -d -s -L 1 ./src
   ```

7. **Output the tree structure as Markdown:**

   ```bash
   rustree --output-format markdown > project_structure.md
   ```

8. **Sort by modification time (oldest first using `-t`):**

   ```bash
   rustree -t ./my_project
   ```

9. **List files in directory order (unsorted using `-U`):**

   ```bash
   rustree -U ./my_project
   ```

10. **Apply the `CountPluses` function to files and sort by its custom output:**

    ```bash
    rustree --apply-function CountPluses --sort-key custom ./config_files
    ```

    _(This assumes `CountPluses` is a meaningful function for your files, e.g., counting '+' characters)._

11. **Pipe `rustree` output to an LLM for summarization:**

    ```bash
    rustree -L 1 --report-sizes ./src --llm-ask "What are the main components in the src directory based on this tree?"
    ```

    Then, you would typically pipe this entire output to your LLM command-line tool. For example:

    ```bash
    rustree -L 1 --report-sizes ./src --llm-ask "Summarize these components" | ollama run mistral
    ```

12. **List only Rust source files (`*.rs`):**

    ```bash
    rustree -P "*.rs" ./my_project
    ```

13. **List only Markdown (`*.md`) or text (`*.txt`) files:**

    ```bash
    rustree -P "*.md|*.txt" ./notes
    # or equivalently
    rustree -P "*.md" -P "*.txt" ./notes
    ```

14. **List only directories named `build` or `target`:**

    ```bash
    rustree -P "build/|target/" ./my_project
    ```

15. **List all Markdown files, including hidden ones (e.g., in `.github/`):**

    ```bash
    rustree -a -P "*.md"
    ```

16. **List files starting with `test_` followed by any single character and then `.py`:**

    ```bash
    rustree -P "test_?.py" ./tests
    ```

17. **List all files within any subdirectory named `docs`:**

    ```bash
    rustree -P "docs/**" ./project_root
    ```

18. **Ignore all `.log` files:**

    ```bash
    rustree -I "*.log" ./my_project
    ```

19. **Ignore the `target/` directory and all `*.tmp` files:**

    ```bash
    rustree -I "target/" -I "*.tmp" ./my_project
    ```

20. **Use `.gitignore` files to filter the output:**

    ```bash
    rustree --use-gitignore ./my_git_repo
    ```

21. **Use a custom ignore file in addition to (or instead of) `.gitignore`:**

    ```bash
    rustree --git-ignore-files ./.my_custom_ignores ./my_project
    ```

    If you also want standard `.gitignore` behavior, add `--use-gitignore`:

    ```bash
    rustree --use-gitignore --git-ignore-files ./.my_custom_ignores ./my_project
    ```

22. **List only `.TXT` files, case-insensitively (matching `file.txt`, `FILE.TXT`, etc.):**

    ```bash
    rustree -P "*.TXT" --ignore-case ./my_project
    ```

23. **Ignore all files ending with `.bak`, case-insensitively, using `-I`:**

    ```bash
    rustree -I "*.bak" --ignore-case ./my_project
    ```

Note: These examples cover common use cases. Combine options as needed to achieve your desired output! Remember to use `rustree --help` for a full list of options.
