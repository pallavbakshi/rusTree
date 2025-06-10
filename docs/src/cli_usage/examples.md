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

10. **Apply the `count-pluses` function to files and sort by its custom output:**

    ```bash
    rustree --apply-function count-pluses --sort-by custom ./config_files
    ```

    _(This counts '+' characters in each file and displays the count in metadata)._

11. **Display file contents after the tree structure using the `cat` function:**

    ```bash
    rustree --apply-function cat ./small_project
    ```

    This will first display the directory tree, then show the full contents of each file in the project. Useful for getting a complete view of small projects or configuration directories.

12. **Combine `cat` function with filtering to show contents of specific files:**

    ```bash
    rustree --apply-function cat --filter-include "*.md|*.txt" ./docs
    ```

    Shows the tree structure and then displays the contents of only Markdown and text files.

13. **Pipe `rustree` output to an LLM for summarization:**

    ```bash
    rustree --depth 1 --show-size-bytes ./src --llm-ask "What are the main components in the src directory based on this tree?"
    # or using short flags
    rustree -L 1 -s ./src --llm-ask "What are the main components in the src directory based on this tree?"
    ```

    Then, you would typically pipe this entire output to your LLM command-line tool. For example:

    ```bash
    rustree -L 1 -s ./src --llm-ask "Summarize these components" | ollama run mistral
    ```

14. **List only Rust source files (`*.rs`):**

    ```bash
    rustree --filter-include "*.rs" ./my_project
    # or using short flag
    rustree -P "*.rs" ./my_project
    ```

15. **List only Markdown (`*.md`) or text (`*.txt`) files:**

    ```bash
    rustree --filter-include "*.md|*.txt" ./notes
    # or equivalently
    rustree -P "*.md" -P "*.txt" ./notes
    ```

16. **List only directories named `build` or `target`:**
    (Note: `-P` or `--filter-include` matches files and directories. A trailing `/` makes it specific to directories.)
    ```bash
    rustree --filter-include "build/|target/" ./my_project
    # or using short flag
    rustree -P "build/|target/" ./my_project
    ```

17. **List all Markdown files, including hidden ones (e.g., in `.github/`):**

    ```bash
    rustree --include-hidden --filter-include "*.md"
    # or using short flags
    rustree -a -P "*.md"
    ```

18. **List files starting with `test_` followed by any single character and then `.py`:**

    ```bash
    rustree --filter-include "test_?.py" ./tests
    # or using short flag
    rustree -P "test_?.py" ./tests
    ```

19. **List all files within any subdirectory named `docs`:**

    ```bash
    rustree --filter-include "docs/**" ./project_root
    # or using short flag
    rustree -P "docs/**" ./project_root
    ```

20. **Ignore all `.log` files:**

    ```bash
    rustree --filter-exclude "*.log" ./my_project
    # or using short flag
    rustree -I "*.log" ./my_project
    ```

21. **Ignore the `target/` directory and all `*.tmp` files:**

    ```bash
    rustree --filter-exclude "target/" --filter-exclude "*.tmp" ./my_project
    # or using short flags
    rustree -I "target/" -I "*.tmp" ./my_project
    ```

22. **Use `.gitignore` files to filter the output:**

    ```bash
    rustree --use-gitignore-rules ./my_git_repo
    # or using the deprecated alias: rustree --gitignore ./my_git_repo
    ```

23. **Use a custom ignore file in addition to (or instead of) `.gitignore`:**

    ```bash
    rustree --gitignore-file ./.my_custom_ignores ./my_project
    ```

    If you also want standard `.gitignore` behavior, add `--use-gitignore-rules` (or its alias `--gitignore`):

    ```bash
    rustree --use-gitignore-rules --gitignore-file ./.my_custom_ignores ./my_project
    ```

24. **List only `.TXT` files, case-insensitively (matching `file.txt`, `FILE.TXT`, etc.):**

    ```bash
    rustree --filter-include "*.TXT" --case-insensitive-filter ./my_project
    # or using short flag
    rustree -P "*.TXT" --case-insensitive-filter ./my_project
    ```

25. **Ignore all files ending with `.bak`, case-insensitively, using `-I`:**

    ```bash
    rustree --filter-exclude "*.bak" --case-insensitive-filter ./my_project
    # or using short flag
    rustree -I "*.bak" --case-insensitive-filter ./my_project
    ```

26. **Sort files by version (e.g., `file-1.0.0`, `file-1.2.0`, `file-2.0.0`):**

    ```bash
    rustree -v ./my_scripts
    # or using --sort-by
    rustree --sort-by version ./my_scripts
    ```

27. **Sort files by change time (ctime) and display change times:**

    ```bash
    rustree -c -D ./my_project
    # or using --sort-by
    rustree --sort-by ctime --show-last-modified ./my_project
    ```
    This will sort by ctime (oldest first). The `-D` (or `--show-last-modified`) flag, when combined with `-c` (or `--sort-by ctime`), will display these ctimes.

28. **Sort files by creation time (crtime/btime), newest first:**
    (Note: Creation time might not be available on all filesystems or OS versions.)
    ```bash
    rustree --sort-by crtime -r ./my_photos
    ```

29. **Prune empty directories from the output:**
    Imagine a project with many empty `build/` or `log/` subdirectories.

    ```bash
    rustree --prune-empty-directories ./my_project
    # or using the alias
    rustree --prune ./my_project
    ```
    This will list `my_project`, but any directories within it (or nested deeper) that become empty after other filters (like `-P`, `-I`, or gitignore) are applied will not be shown.

30. **Prune empty directories while listing only `.rs` files:**

    ```bash
    rustree -P "*.rs" --prune ./my_rust_project
    ```
    In this case, if a directory `src/utils/` contains only `helper.txt` and `mod.rs`, after `-P "*.rs"` is applied, `helper.txt` is filtered out. If `src/utils/` now only effectively contains `mod.rs`, it's not empty. However, if `src/empty_module/` contained only `old_code.txt`, it would first be filtered by `-P`, then `src/empty_module/` would become empty and subsequently pruned by `--prune`.

31. **List directories before files for better readability:**

    ```bash
    rustree --dirs-first ./my_project
    ```
    This will show all directories before any files at each level, making the structure more readable by grouping similar types together.

32. **List files before directories:**

    ```bash
    rustree --files-first ./my_project
    ```
    This will show all files before any directories at each level.

33. **Combine directory ordering with different sort modes:**

    ```bash
    # Directories first, sorted by modification time
    rustree --dirs-first --sort-by mtime ./my_project

    # Files first, sorted by size (largest first)
    rustree --files-first --sort-by size -r ./my_project

    # Directories first with version sorting
    rustree --dirs-first -v ./releases
    ```

34. **Directory ordering with metadata and filtering:**

    ```bash
    # Show directories first with sizes and modification times, only for .rs files and directories
    rustree --dirs-first -s -D -P "*.rs|*/" ./src

    # Files first, showing line counts for text files
    rustree --files-first --calculate-lines -P "*.txt|*.md|*/" ./docs
    ```

## Apply Function Examples

35. **Get directory statistics showing file count, directory count, and total size:**

    ```bash
    rustree --apply-function dir-stats --show-size-bytes ./my_project
    ```
    
    This shows statistics like `[F: "5f,2d,1024B"]` for each directory, indicating 5 files, 2 subdirectories, and 1024 bytes total.

36. **Count files in each directory:**

    ```bash
    rustree --apply-function count-files ./project
    ```
    
    Shows `[F: "3"]` for directories containing 3 files.

37. **Calculate total size of files in each directory:**

    ```bash
    rustree --apply-function size-total --show-size-bytes ./downloads
    ```
    
    Note: `--show-size-bytes` must be enabled for size-total to work properly.

38. **Apply function only to specific directories using patterns:**

    ```bash
    # Only apply dir-stats to src directories
    rustree --apply-function dir-stats --apply-include "src*" ./workspace

    # Apply count-files to all directories except target and build
    rustree --apply-function count-files --apply-exclude "target/*" --apply-exclude "build/*" ./project
    ```

39. **Use pattern files for complex filtering:**

    Create a file `include-patterns.txt`:
    ```
    # Include source directories
    src/*
    lib/*
    # Include documentation
    docs/*
    ```

    Create a file `exclude-patterns.txt`:
    ```
    # Exclude build artifacts
    *target*
    *build*
    # Exclude temporary files
    *.tmp
    ```

    Then use:
    ```bash
    rustree --apply-function dir-stats \
            --apply-include-from ./include-patterns.txt \
            --apply-exclude-from ./exclude-patterns.txt ./project
    ```

40. **Combine cat function with selective application:**

    ```bash
    # Show contents of only configuration files
    rustree --apply-function cat --apply-include "*.toml" --apply-include "*.yml" ./config
    
    # Show contents excluding sensitive files
    rustree --apply-function cat --apply-exclude "*secret*" --apply-exclude "*key*" ./scripts
    ```

41. **Directory analysis for large projects:**

    ```bash
    # Get overview of all subdirectories with statistics
    rustree -d --apply-function dir-stats --show-size-bytes --sort-by custom -r ./large_project
    ```
    
    This shows only directories (`-d`), applies statistics function, enables size collection, and sorts by the statistics output in reverse order (largest/most complex directories first).

42. **Analyze code organization:**

    ```bash
    # Count Rust files in each module directory
    rustree --apply-function count-files --apply-include "*.rs" -d ./src
    
    # Get comprehensive statistics for source directories only
    rustree --apply-function dir-stats --apply-include "src*" --apply-include "lib*" ./workspace
    ```

Note: These examples cover common use cases. Combine options as needed to achieve your desired output! Remember to use `rustree --help` for a full list of options.