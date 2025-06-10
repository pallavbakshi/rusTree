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
   rustree --show-size-bytes --show-last-modified -t --reverse-sort ~/Documents
   # or using --sort-by
   rustree --show-size-bytes --show-last-modified --sort-by mtime --reverse-sort ~/Documents
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
   rustree --directory-only --show-size-bytes --depth 1 ./src
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
    rustree --depth 1 --show-size-bytes ./src --llm-ask "What are the main components in the src directory based on this tree?"
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
    rustree --use-gitignore-rules ./my_git_repo
    # or using the deprecated alias: rustree --gitignore ./my_git_repo
    ```

21. **Use a custom ignore file in addition to (or instead of) `.gitignore`:**

    ```bash
    rustree --gitignore-file ./.my_custom_ignores ./my_project
    ```

    If you also want standard `.gitignore` behavior, add `--use-gitignore-rules` (or its alias `--gitignore`):

    ```bash
    rustree --use-gitignore-rules --gitignore-file ./.my_custom_ignores ./my_project
    ```

22. **List only `.TXT` files, case-insensitively (matching `file.txt`, `FILE.TXT`, etc.):**

    ```bash
    rustree --filter-include "*.TXT" --case-insensitive-filter ./my_project
    # or using short flag
    rustree -P "*.TXT" --case-insensitive-filter ./my_project
    ```

23. **Ignore all files ending with `.bak`, case-insensitively, using `-I`:**

    ```bash
    rustree --filter-exclude "*.bak" --case-insensitive-filter ./my_project
    # or using short flag
    rustree -I "*.bak" --case-insensitive-filter ./my_project
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
    rustree --sort-by ctime --show-last-modified ./my_project
    ```
    This will sort by ctime (oldest first). The `-D` (or `--show-last-modified`) flag, when combined with `-c` (or `--sort-by ctime`), will display these ctimes.

26. **Sort files by creation time (crtime/btime), newest first:**
    (Note: Creation time might not be available on all filesystems or OS versions.)
    ```bash
    rustree --sort-by crtime -r ./my_photos
    ```

27. **Prune empty directories from the output:**
    Imagine a project with many empty `build/` or `log/` subdirectories.

    ```bash
    rustree --prune-empty-directories ./my_project
    # or using the alias
    rustree --prune ./my_project
    ```
    This will list `my_project`, but any directories within it (or nested deeper) that become empty after other filters (like `-P`, `-I`, or gitignore) are applied will not be shown.

28. **Prune empty directories while listing only `.rs` files:**

    ```bash
    rustree -P "*.rs" --prune ./my_rust_project
    ```
    In this case, if a directory `src/utils/` contains only `helper.txt` and `mod.rs`, after `-P "*.rs"` is applied, `helper.txt` is filtered out. If `src/utils/` now only effectively contains `mod.rs`, it's not empty. However, if `src/empty_module/` contained only `old_code.txt`, it would first be filtered by `-P`, then `src/empty_module/` would become empty and subsequently pruned by `--prune`.

29. **List directories before files for better readability:**

    ```bash
    rustree --dirs-first ./my_project
    ```
    This will show all directories before any files at each level, making the structure more readable by grouping similar types together.

30. **List files before directories:**

    ```bash
    rustree --files-first ./my_project
    ```
    This will show all files before any directories at each level.

31. **Combine directory ordering with different sort modes:**

    ```bash
    # Directories first, sorted by modification time
    rustree --dirs-first --sort-by mtime ./my_project

    # Files first, sorted by size (largest first)
    rustree --files-first --sort-by size -r ./my_project

    # Directories first with version sorting
    rustree --dirs-first -v ./releases
    ```

32. **Directory ordering with metadata and filtering:**

    ```bash
    # Show directories first with sizes and modification times, only for .rs files and directories
    rustree --dirs-first -s -D -P "*.rs|*/" ./src

    # Files first, showing line counts for text files
    rustree --files-first --calculate-lines -P "*.txt|*.md|*/" ./docs
    ```

Note: These examples cover common use cases. Combine options as needed to achieve your desired output! Remember to use `rustree --help` for a full list of options.