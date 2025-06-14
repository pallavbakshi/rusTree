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
   
   **Enhanced Summary Report**: The output now includes aggregated totals in the summary:
   ```
   src/
   ├── [L:  364] [W:1498] lib.rs
   ├── [L:   63] [W: 243] main.rs
   └── core/
       ├── [L:  119] [W: 264] mod.rs
       └── [L:  200] [W: 450] util.rs
   
   2 directories, 4 files, 746 total lines, 2,455 total words
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

8. **Output tree without the summary report at the end:**

   ```bash
   rustree --no-summary-report ./my_project
   ```
   
   This will display the tree structure without the summary line that normally shows directory/file counts and metadata totals like "4 directories, 6 files, 1,234 total lines".

9. **Generate clean markdown output without summary for documentation:**

   ```bash
   rustree --output-format markdown --no-summary-report ./src > code_structure.md
   ```
   
   Perfect for including in documentation where you want just the tree structure without file counts.

10. **Sort by modification time (oldest first using `-t`):**

   ```bash
   rustree -t ./my_project
   # or using --sort-by
   rustree --sort-by mtime ./my_project
   ```

11. **List files in directory order (unsorted using `-U`):**

   ```bash
   rustree -U ./my_project
   # or using long flag
   rustree --unsorted ./my_project
   # or using --sort-by
   rustree --sort-by none ./my_project
   ```

11b. **Show full paths for all files (similar to tree -f):**

   ```bash
   rustree --full-path ./my_project
   # or using short flag
   rustree -f ./my_project
   ```
   
   **Example output:**
   ```
   my_project/
   ├── README.md
   ├── src/
   │   ├── src/main.rs
   │   ├── src/lib.rs
   │   └── src/utils/
   │       └── src/utils/helper.rs
   └── tests/
       └── tests/integration_test.rs
   
   3 directories, 5 files
   ```
   
   Notice how nested files show their full relative path from the scan root (e.g., `src/main.rs` instead of just `main.rs`).

11c. **Combine full-path with other metadata for detailed analysis:**

   ```bash
   rustree -f -s --calculate-lines ./my_project
   ```
   
   **Example output:**
   ```
   my_project/
   ├── [   1024B] [L:  50] README.md
   ├── src/
   │   ├── [   2048B] [L: 150] src/main.rs
   │   ├── [   1536B] [L: 100] src/lib.rs
   │   └── src/utils/
   │       └── [    512B] [L:  25] src/utils/helper.rs
   └── tests/
       └── [   1280B] [L:  75] tests/integration_test.rs
   
   3 directories, 4 files, 400 total lines, 6.25 KB total
   ```
   
   This combines full paths with size and line count information, making it easy to identify specific files and their characteristics.

11d. **Full-path output in Markdown format:**

   ```bash
   rustree -f --output-format markdown ./docs > file_structure.md
   ```
   
   **Generated Markdown:**
   ```markdown
   # docs
   
   * README.md
   * guides/
     * guides/installation.md
     * guides/usage.md
     * guides/advanced/
       * guides/advanced/configuration.md
   * api/
     * api/reference.md
   
   __3 directories, 5 files total__
   ```
   
   Perfect for documentation where you want to show the complete file paths in a structured format.

12. **Apply the `count-pluses` function to files and sort by its custom output:**

    ```bash
    rustree --apply-function count-pluses --sort-by custom ./config_files
    ```

    _(This counts '+' characters in each file and displays the count in metadata)._

13. **Display file contents after the tree structure using the `cat` function:**

    ```bash
    rustree --apply-function cat ./small_project
    ```

    This will first display the directory tree, then show the full contents of each file in the project. Useful for getting a complete view of small projects or configuration directories.

14. **Combine `cat` function with filtering to show contents of specific files:**

    ```bash
    rustree --apply-function cat --filter-include "*.md|*.txt" ./docs
    ```

    Shows the tree structure and then displays the contents of only Markdown and text files.

15. **Export formatted query for external LLM tools:**

    ```bash
    rustree --llm-export "What are the main components in this project?" ./src
    ```

    This outputs a specially formatted query that can be piped to external LLM tools:

    ```bash
    rustree --llm-export "Analyze the architecture" | claude-cli
    rustree --llm-export "Security review" | ollama run mistral
    rustree --llm-export "Code quality assessment" > analysis-prompt.txt
    ```

16. **Direct LLM integration (requires API keys):**

    ```bash
    # Basic LLM query with OpenAI (default provider)
    rustree --llm-ask "What's the architecture of this project?"
    
    # With specific provider and model
    rustree --llm-ask "Analyze this codebase structure" \
      --llm-provider anthropic \
      --llm-model claude-3-sonnet
    
    # With custom settings for focused analysis
    rustree --llm-ask "Brief security review" \
      --llm-temperature 0.3 \
      --llm-max-tokens 500
    ```

17. **LLM analysis with tree filtering:**

    ```bash
    # Analyze only source code structure
    rustree --llm-ask "What patterns do you see?" \
      --include "*.rs" --exclude "**/target/**" \
      --depth 3
    
    # Focus on documentation structure
    rustree --llm-ask "How is documentation organized?" \
      --include "*.md" --dirs-only
    
    # Analyze project architecture at high level
    rustree --llm-ask "Describe the overall architecture" \
      --depth 2 --dirs-only --size
    ```

18. **LLM setup and configuration:**

    ```bash
    # Generate .env template for API keys
    rustree --llm-generate-env > .env
    # Then edit .env file to add your API keys
    
    # Use with environment variables
    export OPENAI_API_KEY="your-key-here"
    rustree --llm-ask "Analyze this project"
    
    # Override with CLI argument
    rustree --llm-ask "Quick analysis" --llm-api-key "your-key"
    
    # Use different providers
    rustree --llm-ask "Code review" --llm-provider anthropic
    rustree --llm-ask "Performance analysis" --llm-provider cohere
    rustree --llm-ask "Multi-model analysis" --llm-provider openrouter
    ```

19. **Advanced LLM analysis examples:**

    ```bash
    # Architecture analysis with context
    rustree --llm-ask "What architectural patterns do you see? Suggest improvements." \
      --depth 4 --dirs-only --size --llm-temperature 0.2
    
    # Security-focused analysis
    rustree --llm-ask "Are there any potential security concerns in this structure?" \
      --include "*.rs" --include "*.toml" --include "*.yml" \
      --llm-provider anthropic --llm-model claude-3-sonnet
    
    # Performance optimization suggestions
    rustree --llm-ask "What could be optimized for better performance?" \
      --size --file-stats --calculate-lines \
      --llm-max-tokens 1000
    
    # Code organization review
    rustree --llm-ask "How would you reorganize this codebase?" \
      --include "*.rs" --depth 3 \
      --llm-temperature 0.7 --llm-max-tokens 1500
    ```

20. **Combining LLM with metadata analysis:**

    ```bash
    # Analyze large files and complexity
    rustree --llm-ask "Which files seem overly complex based on size and line count?" \
      --size --calculate-lines --sort-by size --reverse-sort
    
    # Module organization analysis
    rustree --llm-ask "How well are the modules organized?" \
      --apply-function dir-stats --dirs-only --depth 2
    
    # Documentation coverage analysis
    rustree --llm-ask "What documentation is missing?" \
      --include "*.md" --include "*.rs" --calculate-words
    ```

21. **LLM request preview and debugging:**

    ```bash
    # Preview what would be sent to the LLM (no API call, no cost)
    rustree --llm-ask "What's the architecture of this project?" --dry-run
    
    # Human-readable markdown format for better readability
    rustree --llm-ask "Analyze the code organization" \
      --dry-run --human-friendly
    
    # Cost estimation for complex queries
    rustree --llm-ask "Detailed security analysis with recommendations" \
      --llm-max-tokens 3000 --dry-run
    
    # Debug prompt structure before sending
    rustree --llm-ask "Custom analysis question" \
      --include "*.rs" --depth 3 --size --dry-run --human-friendly
    
    # Verify API configuration without making requests
    rustree --llm-ask "Test question" --llm-provider anthropic \
      --llm-model claude-3-haiku --dry-run
    ```

22. **LLM workflow examples:**

    ```bash
    # Step 1: Preview and refine your question
    rustree --llm-ask "Initial analysis" --dry-run --human-friendly
    
    # Step 2: Export for review
    rustree --llm-export "Refined analysis question" > project-analysis.txt
    
    # Step 3: Direct analysis with verified prompt
    rustree --llm-ask "Focus on error handling patterns" \
      --include "*.rs" --llm-temperature 0.3
    
    # Step 4: Compare with external tool
    rustree --llm-export "Compare with previous analysis" | your-llm-tool
    ```

23. **List only Rust source files (`*.rs`):**

    ```bash
    rustree --filter-include "*.rs" ./my_project
    # or using short flag
    rustree -P "*.rs" ./my_project
    ```

24. **List only Markdown (`*.md`) or text (`*.txt`) files:**

    ```bash
    rustree --filter-include "*.md|*.txt" ./notes
    # or equivalently
    rustree -P "*.md" -P "*.txt" ./notes
    ```

25. **List only directories named `build` or `target`:**
    (Note: `-P` or `--filter-include` matches files and directories. A trailing `/` makes it specific to directories.)
    ```bash
    rustree --filter-include "build/|target/" ./my_project
    # or using short flag
    rustree -P "build/|target/" ./my_project
    ```

26. **List all Markdown files, including hidden ones (e.g., in `.github/`):**

    ```bash
    rustree --include-hidden --filter-include "*.md"
    # or using short flags
    rustree -a -P "*.md"
    ```

27. **List files starting with `test_` followed by any single character and then `.py`:**

    ```bash
    rustree --filter-include "test_?.py" ./tests
    # or using short flag
    rustree -P "test_?.py" ./tests
    ```

28. **List all files within any subdirectory named `docs`:**

    ```bash
    rustree --filter-include "docs/**" ./project_root
    # or using short flag
    rustree -P "docs/**" ./project_root
    ```

29. **Ignore all `.log` files:**

    ```bash
    rustree --filter-exclude "*.log" ./my_project
    # or using short flag
    rustree -I "*.log" ./my_project
    ```

29. **Ignore the `target/` directory and all `*.tmp` files:**

    ```bash
    rustree --filter-exclude "target/" --filter-exclude "*.tmp" ./my_project
    # or using short flags
    rustree -I "target/" -I "*.tmp" ./my_project
    ```

30. **Use `.gitignore` files to filter the output:**

    ```bash
    rustree --use-gitignore-rules ./my_git_repo
    # or using the deprecated alias: rustree --gitignore ./my_git_repo
    ```

31. **Use a custom ignore file in addition to (or instead of) `.gitignore`:**

    ```bash
    rustree --gitignore-file ./.my_custom_ignores ./my_project
    ```

    If you also want standard `.gitignore` behavior, add `--use-gitignore-rules` (or its alias `--gitignore`):

    ```bash
    rustree --use-gitignore-rules --gitignore-file ./.my_custom_ignores ./my_project
    ```

32. **List only `.TXT` files, case-insensitively (matching `file.txt`, `FILE.TXT`, etc.):**

    ```bash
    rustree --filter-include "*.TXT" --case-insensitive-filter ./my_project
    # or using short flag
    rustree -P "*.TXT" --case-insensitive-filter ./my_project
    ```

33. **Ignore all files ending with `.bak`, case-insensitively, using `-I`:**

    ```bash
    rustree --filter-exclude "*.bak" --case-insensitive-filter ./my_project
    # or using short flag
    rustree -I "*.bak" --case-insensitive-filter ./my_project
    ```

34. **Sort files by version (e.g., `file-1.0.0`, `file-1.2.0`, `file-2.0.0`):**

    ```bash
    rustree -v ./my_scripts
    # or using --sort-by
    rustree --sort-by version ./my_scripts
    ```

35. **Sort files by change time (ctime) and display change times:**

    ```bash
    rustree -c -D ./my_project
    # or using --sort-by
    rustree --sort-by ctime --show-last-modified ./my_project
    ```
    This will sort by ctime (oldest first). The `-D` (or `--show-last-modified`) flag, when combined with `-c` (or `--sort-by ctime`), will display these ctimes.

36. **Sort files by creation time (crtime/btime), newest first:**
    (Note: Creation time might not be available on all filesystems or OS versions.)
    ```bash
    rustree --sort-by crtime -r ./my_photos
    ```

37. **Prune empty directories from the output:**
    Imagine a project with many empty `build/` or `log/` subdirectories.

    ```bash
    rustree --prune-empty-directories ./my_project
    # or using the alias
    rustree --prune ./my_project
    ```
    This will list `my_project`, but any directories within it (or nested deeper) that become empty after other filters (like `-P`, `-I`, or gitignore) are applied will not be shown.

38. **Prune empty directories while listing only `.rs` files:**

    ```bash
    rustree -P "*.rs" --prune ./my_rust_project
    ```
    In this case, if a directory `src/utils/` contains only `helper.txt` and `mod.rs`, after `-P "*.rs"` is applied, `helper.txt` is filtered out. If `src/utils/` now only effectively contains `mod.rs`, it's not empty. However, if `src/empty_module/` contained only `old_code.txt`, it would first be filtered by `-P`, then `src/empty_module/` would become empty and subsequently pruned by `--prune`.

39. **List directories before files for better readability:**

    ```bash
    rustree --dirs-first ./my_project
    ```
    This will show all directories before any files at each level, making the structure more readable by grouping similar types together.

40. **List files before directories:**

    ```bash
    rustree --files-first ./my_project
    ```
    This will show all files before any directories at each level.

41. **Combine directory ordering with different sort modes:**

    ```bash
    # Directories first, sorted by modification time
    rustree --dirs-first --sort-by mtime ./my_project

    # Files first, sorted by size (largest first)
    rustree --files-first --sort-by size -r ./my_project

    # Directories first with version sorting
    rustree --dirs-first -v ./releases
    ```

42. **Directory ordering with metadata and filtering:**

    ```bash
    # Show directories first with sizes and modification times, only for .rs files and directories
    rustree --dirs-first -s -D -P "*.rs|*/" ./src

    # Files first, showing line counts for text files
    rustree --files-first --calculate-lines -P "*.txt|*.md|*/" ./docs
    ```

## Apply Function Examples

43. **Get directory statistics showing file count, directory count, and total size:**

    ```bash
    rustree --apply-function dir-stats --show-size-bytes ./my_project
    ```
    
    This shows statistics like `[F: "5f,2d,1024B"]` for each directory, indicating 5 files, 2 subdirectories, and 1024 bytes total.

44. **Count files in each directory:**

    ```bash
    rustree --apply-function count-files ./project
    ```
    
    Shows `[F: "3"]` for directories containing 3 files.

45. **Calculate total size of files in each directory:**

    ```bash
    rustree --apply-function size-total --show-size-bytes ./downloads
    ```
    
    Note: `--show-size-bytes` must be enabled for size-total to work properly.

46. **Apply function only to specific directories using patterns:**

    ```bash
    # Only apply dir-stats to src directories
    rustree --apply-function dir-stats --apply-include "src*" ./workspace

    # Apply count-files to all directories except target and build
    rustree --apply-function count-files --apply-exclude "target/*" --apply-exclude "build/*" ./project
    ```

47. **Use pattern files for complex filtering:**

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

48. **Combine cat function with selective application:**

    ```bash
    # Show contents of only configuration files
    rustree --apply-function cat --apply-include "*.toml" --apply-include "*.yml" ./config
    
    # Show contents excluding sensitive files
    rustree --apply-function cat --apply-exclude "*secret*" --apply-exclude "*key*" ./scripts
    ```

## File-based Pattern Filtering Examples

The `--filter-include-from` and `--filter-exclude-from` options allow you to maintain pattern lists in separate files, which is especially useful for complex projects or when sharing filter configurations across teams.

56. **Use file-based include patterns:**

    Create a file `include-patterns.txt`:
    ```
    # Include only source code files
    *.rs
    *.go
    *.js
    *.ts
    
    # Include configuration files
    *.toml
    *.yml
    *.yaml
    
    # Include documentation
    *.md
    README*
    ```

    Then use:
    ```bash
    rustree --filter-include-from ./include-patterns.txt ./project
    ```

57. **Use file-based exclude patterns:**

    Create a file `exclude-patterns.txt`:
    ```
    # Build artifacts
    target/
    build/
    dist/
    
    # Dependencies
    node_modules/
    vendor/
    
    # Temporary files
    *.tmp
    *.log
    *.swp
    .DS_Store
    ```

    Then use:
    ```bash
    rustree --filter-exclude-from ./exclude-patterns.txt ./project
    ```

58. **Combine file-based and command-line patterns:**

    ```bash
    # Use patterns from file and add additional CLI patterns
    rustree --filter-include-from ./src-patterns.txt \
            --filter-include "*.md" \
            --filter-exclude "*.test.js" ./project
    ```

59. **Multiple pattern files:**

    ```bash
    # Load patterns from multiple files
    rustree --filter-include-from ./core-patterns.txt \
            --filter-include-from ./docs-patterns.txt \
            --filter-exclude-from ./ignore-patterns.txt ./workspace
    ```

60. **Project-specific filtering setup:**

    Create a `.rustree` directory in your project with pattern files:
    ```bash
    mkdir .rustree
    echo "*.rs" > .rustree/rust-files.txt
    echo "*.md" > .rustree/docs.txt
    echo "target/" > .rustree/ignore.txt
    ```

    Then use consistently across the team:
    ```bash
    # Show only Rust files
    rustree --filter-include-from .rustree/rust-files.txt ./
    
    # Show documentation structure
    rustree --filter-include-from .rustree/docs.txt --dirs-first ./
    
    # Full tree excluding build artifacts
    rustree --filter-exclude-from .rustree/ignore.txt ./
    ```

61. **Combine with gitignore patterns:**

    ```bash
    # Use gitignore AND custom exclude patterns
    rustree --use-gitignore-rules \
            --filter-exclude-from ./additional-ignores.txt ./project
    
    # Override gitignore with specific includes
    rustree --use-gitignore-rules \
            --filter-include-from ./force-include.txt ./project
    ```

62. **Pattern file with comments for documentation:**

    Create a well-documented `analysis-patterns.txt`:
    ```
    # Code Analysis Patterns
    # =====================
    
    # Core source files
    src/**/*.rs
    lib/**/*.rs
    
    # Test files (for separate analysis)
    tests/**/*.rs
    benches/**/*.rs
    
    # Configuration that affects behavior
    Cargo.toml
    .cargo/config.toml
    rust-toolchain.toml
    
    # Exclude examples and docs from analysis
    # (uncomment if needed)
    # !examples/
    # !docs/
    ```

    Use for consistent code analysis:
    ```bash
    rustree --filter-include-from ./analysis-patterns.txt \
            --calculate-lines --sort-by size -r ./
    ```

56. **Directory analysis for large projects:**

    ```bash
    # Get overview of all subdirectories with statistics
    rustree -d --apply-function dir-stats --show-size-bytes --sort-by custom -r ./large_project
    ```
    
    This shows only directories (`-d`), applies statistics function, enables size collection, and sorts by the statistics output in reverse order (largest/most complex directories first).

50. **Analyze code organization:**

    ```bash
    # Count Rust files in each module directory
    rustree --apply-function count-files --apply-include "*.rs" -d ./src
    
    # Get comprehensive statistics for source directories only
    rustree --apply-function dir-stats --apply-include "src*" --apply-include "lib*" ./workspace
    ```

## Metadata Aggregation Examples

The summary report now automatically aggregates metadata values, providing totals for lines, words, sizes, and apply function outputs.

51. **Get comprehensive project statistics:**

    ```bash
    rustree --calculate-lines --calculate-words --show-size-bytes ./my_project
    ```
    
    Output includes totals in the summary:
    ```
    my_project/
    ├── [   1024B] [L:  50] [W: 250] README.md
    ├── [   2048B] [L: 100] [W: 500] main.rs
    └── src/
        ├── [   3072B] [L: 150] [W: 750] lib.rs
        └── [   1536B] [L:  75] [W: 375] util.rs
    
    2 directories, 4 files, 375 total lines, 1,875 total words, 7.7 KB total
    ```

52. **Analyze large codebases with human-readable totals:**

    ```bash
    rustree --calculate-lines --depth 2 ./large_project
    ```
    
    Automatically formats large numbers with thousand separators:
    ```
    large_project/
    ├── [L:12345] frontend/
    ├── [L: 8765] backend/
    └── [L: 4321] docs/
    
    4 directories, 156 files, 125,431 total lines
    ```

53. **Combine size analysis with directory statistics:**

    ```bash
    rustree --show-size-bytes --apply-function dir-stats ./project
    ```
    
    Shows both individual file sizes and aggregated directory statistics:
    ```
    project/
    ├── [   512B] config.toml
    ├── [F: "3f,0d,1536B"] src/
    │   ├── [  1024B] main.rs
    │   └── [   512B] lib.rs
    └── [F: "2f,0d,256B"] tests/
        ├── [  128B] test1.rs
        └── [  128B] test2.rs
    
    3 directories, 5 files, 2.3 KB total, 1.8 KB total (from function)
    ```

54. **Quick project overview with multiple metadata types:**

    ```bash
    rustree --depth 1 --calculate-lines --calculate-words --show-size-bytes ./workspace
    ```
    
    Perfect for getting a high-level overview of project complexity:
    ```
    workspace/
    ├── [  45.2 KB] [L:1200] [W:6000] frontend/
    ├── [  32.1 KB] [L: 900] [W:4500] backend/
    ├── [  12.8 KB] [L: 400] [W:2000] shared/
    └── [   5.5 KB] [L: 150] [W: 750] docs/
    
    5 directories, 87 files, 2,650 total lines, 13,250 total words, 95.6 KB total
    ```

55. **Compare module sizes in a Rust project:**

    ```bash
    rustree --depth 2 --show-size-bytes --filter-include "*.rs|*/" --sort-by size -r ./src
    ```
    
    Shows Rust modules sorted by size with total calculations:
    ```
    src/
    ├── [  15.2 KB] core/
    ├── [  12.8 KB] utils/
    ├── [   8.4 KB] cli/
    ├── [   3.2 KB] main.rs
    └── [   1.1 KB] lib.rs
    
    4 directories, 45 files, 40.7 KB total
    ```

56. **Markdown output with metadata aggregation for documentation:**

    ```bash
    rustree --output-format markdown --calculate-lines --depth 2 ./api > api_overview.md
    ```
    
    Generates markdown with totals:
    ```markdown
    # ./api
    
    * handlers/ 1250L
    * models/ 800L
    * routes/ 600L
    * main.rs 150L
    
    __2 directories, 25 files, 2,800 total lines total__
    ```

Note: The enhanced summary report automatically detects which metadata types are being displayed and includes appropriate totals. No additional flags are needed - the aggregation happens automatically when metadata options like `--calculate-lines`, `--calculate-words`, or `--show-size-bytes` are used.

These examples cover common use cases. Combine options as needed to achieve your desired output! Remember to use `rustree --help` for a full list of options.