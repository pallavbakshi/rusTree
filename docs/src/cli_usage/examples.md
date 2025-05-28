## Examples

Here are some practical examples of how to use `rustree` from the command line.

1.  **Basic tree of the current directory:**
    ```bash
    rustree
    ```

2.  **Tree of a specific directory, showing hidden files and up to depth 2:**
    ```bash
    rustree -a -L 2 /var/log
    ```

3.  **List files in `~/Documents`, showing sizes and modification times, sorted by modification time (newest first using `-t` and `-r`):**
    ```bash
    rustree -s -D -t -r ~/Documents
    ```

4.  **Analyze a source code project, showing line counts and word counts, sorted by line count (largest first):**
    ```bash
    rustree --calculate-lines --calculate-words --sort-key lines -r ./my_project_src
    ```

5.  **List directories only in the current path:**
    ```bash
    rustree -d
    ```

6.  **List directories only in `./src`, showing sizes, up to depth 1:**
    ```bash
    rustree -d -s -L 1 ./src
    ```

7.  **Output the tree structure as Markdown:**
    ```bash
    rustree --output-format markdown > project_structure.md
    ```

8.  **Sort by modification time (oldest first using `-t`):**
    ```bash
    rustree -t ./my_project
    ```

9.  **List files in directory order (unsorted using `-U`):**
    ```bash
    rustree -U ./my_project
    ```

10. **Apply the `CountPluses` function to files and sort by its custom output:**
    ```bash
    rustree --apply-function CountPluses --sort-key custom ./config_files
    ```
    *(This assumes `CountPluses` is a meaningful function for your files, e.g., counting '+' characters).*

11. **Pipe `rustree` output to an LLM for summarization:**
    ```bash
    rustree -L 1 --report-sizes ./src --llm-ask "What are the main components in the src directory based on this tree?"
    ```
    Then, you would typically pipe this entire output to your LLM command-line tool. For example:
    ```bash
    rustree -L 1 --report-sizes ./src --llm-ask "Summarize these components" | ollama run mistral
    ```
    *(Replace `ollama run mistral` with your actual LLM tool and model).*

Note: These examples cover common use cases. Combine options as needed to achieve your desired output! Remember to use `rustree --help` for a full list of options.