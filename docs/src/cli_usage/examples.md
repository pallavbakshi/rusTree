## Examples

Here are some practical examples of how to use `rustree` from the command line.

1. **Basic tree of the current directory:**

   ```bash
   rustree
   ```

2. **Tree of a specific directory, showing hidden files and up to depth 2:**

   ```bash
   rustree --include-hidden --depth 2 /var/log
   # or using short flags
   rustree -a -L 2 /var/log
   ```

3. **List files in `~/Documents`, showing sizes and modification times, sorted by modification time (newest first using `-t` and `-r`):**

   ```bash
   rustree -s -D -t -r ~/Documents
   # or using long flags
   rustree --report-sizes --date -t --reverse-sort ~/Documents
   # or using --sort-by
   rustree --report-sizes --date --sort-by mtime --reverse-sort ~/Documents
   ```

4. **Analyze a source code project, showing line counts and word counts, sorted by line count (largest first):**

   ```bash
   rustree --calculate-lines --calculate-words --sort-by lines -r ./my_project_src
   ```

5. **List directories only in the current path:**

   ```bash
   rustree --directory-only
   # or using short flag
   rustree -d
   ```

6. **List directories only in `./src`, showing sizes, up to depth 1:**

   ```bash
   rustree --directory-only --report-sizes --depth 1 ./src
   # or using short flags
   rustree -d -s -L 1 ./src
   ```

7. **Output the tree structure as Markdown:**

   ```bash
   rustree --output-format markdown > project_structure.md
   ```

8. **Sort by modification time (oldest first using `-t`):**

   ```bash
   rustree -t ./my_project
   # or using --sort-by
   rustree --sort-by mtime ./my_project
   ```

9. **List files in directory order (unsorted using `-U`):**

   ```bash
   rustree -U ./my_project
   # or using long flag
   rustree --unsorted ./my_project
   # or using --sort-by
   rustree --sort-by none ./my_project
   ```

10. **Apply the `CountPluses` function to files and sort by its custom output:**

    ```bash
    rustree --apply-function CountPluses --sort-by custom ./config_files
    ```

    _(This assumes `CountPluses` is a meaningful function for your files, e.g., counting '+' characters)._

11. **Pipe `rustree` output to an LLM for summarization:**

    ```bash
    rustree --depth 1 --report-sizes ./src --llm-ask "What are the main components in the src directory based on this tree?"
    # or using short flags
    rustree -L 1 -s ./src --llm-ask "What are the main components in the src directory based on this tree?"
    ```

    Then, you would typically pipe this entire output to your LLM command-line tool. For example:

    ```bash
    rustree -L 1 -s ./src --llm-ask "Summarize these components" | ollama run mistral
    ```

12. **List only Rust source files (`*.rs`):**

    ```bash
    rustree --filter-include "*.rs" ./my_project
    # or using short flag
    rustree -P "*.rs" ./my_project
    ```

13. **List only Markdown (`*.md`) or text (`*.txt`) files:**

    ```bash
    rustree --filter-include "*.md|*.txt" ./notes
    # or equivalently
    rustree -P "*.md" -P "*.txt" ./notes
    ```

14. **List only directories named `build` or `target`:**
    (Note: `-P` or `--filter-include` matches files and directories. A trailing `/` makes it specific to directories.)
    ```bash
    rustree --filter-include "build/|target/" ./my_project
    # or using short flag
    rustree -P "build/|target/" ./my_project
    ```

15. **List all Markdown files, including hidden ones (e.g., in `.github/`):**

    ```bash
    rustree --include-hidden --filter-include "*.md"
    # or using short flags
    rustree -a -P "*.md"
    ```

16. **List files starting with `test_` followed by any single character and then `.py`:**

    ```bash
    rustree --filter-include "test_?.py" ./tests
    # or using short flag
    rustree -P "test_?.py" ./tests
    ```

17. **List all files within any subdirectory named `docs`:**

    ```bash
    rustree --filter-include "docs/**" ./project_root
    # or using short flag
    rustree -P "docs/**" ./project_root
    ```

18. **Ignore all `.log` files:**

    ```bash
    rustree --filter-exclude "*.log" ./my_project
    # or using short flag
    rustree -I "*.log" ./my_project
    ```

19. **Ignore the `target/` directory and all `*.tmp` files:**

    ```bash
    rustree --filter-exclude "target/" --filter-exclude "*.tmp" ./my_project
    # or using short flags
    rustree -I "target/" -I "*.tmp" ./my_project
    ```

20. **Use `.gitignore` files to filter the output:**

    ```bash
    rustree --gitignore ./my_git_repo
    ```

21. **Use a custom ignore file in addition to (or instead of) `.gitignore`:**

    ```bash
    rustree --git-ignore-files ./.my_custom_ignores ./my_project
    ```

    If you also want standard `.gitignore` behavior, add `--gitignore`:

    ```bash
    rustree --gitignore --git-ignore-files ./.my_custom_ignores ./my_project
    ```

22. **List only `.TXT` files, case-insensitively (matching `file.txt`, `FILE.TXT`, etc.):**

    ```bash
    rustree --filter-include "*.TXT" --ignore-case ./my_project
    # or using short flag
    rustree -P "*.TXT" --ignore-case ./my_project
    ```

23. **Ignore all files ending with `.bak`, case-insensitively, using `-I`:**

    ```bash
    rustree --filter-exclude "*.bak" --ignore-case ./my_project
    # or using short flag
    rustree -I "*.bak" --ignore-case ./my_project
    ```

24. **Sort files by version (e.g., `file-1.0.0`, `file-1.2.0`, `file-2.0.0`):**

    ```bash
    rustree -v ./my_scripts
    # or using --sort-by
    rustree --sort-by version ./my_scripts
    ```

25. **Sort files by change time (ctime) and display change times:**

    ```bash
    rustree -c -D ./my_project
    # or using --sort-by
    rustree --sort-by ctime --date ./my_project
    ```
    This will sort by ctime (oldest first). The `-D` (or `--date`) flag, when combined with `-c` (or `--sort-by ctime`), will display these ctimes.

26. **Sort files by creation time (crtime/btime), newest first:**
    (Note: Creation time might not be available on all filesystems or OS versions.)
    ```bash
    rustree --sort-by crtime -r ./my_photos
    ```

Note: These examples cover common use cases. Combine options as needed to achieve your desired output! Remember to use `rustree --help` for a full list of options.