## Basic Usage

Once RusTree is installed, you can use it from your command line.

### Displaying the Current Directory

The simplest way to use RusTree is to navigate to the directory you want to inspect and run:

```bash
rustree
```

This will display the tree structure of the current directory (`.`).

### Specifying a Path

You can also specify a path to a directory:

```bash
rustree /path/to/your/directory
```

Or a relative path:

```bash
rustree ../some/other/folder
```

### Common Options

Here are a few common options to get you started:

*   **Show all files (including hidden):**
    ```bash
    rustree -a
    # or
    rustree --all
    ```

*   **Limit depth:**
    ```bash
    rustree -L 2  # Show current directory and its direct children (depth 1 and 2)
    ```

*   **Show file sizes:**
    ```bash
    rustree -s
    # or
    rustree --report-sizes
    ```

*   **Sort by size (ascending):**
    ```bash
    rustree --sort-key Size
    ```

*   **Sort by size (descending):**
    ```bash
    rustree --sort-key Size -r
    ```

### Getting Help

For a full list of options and commands, use the help flag:

```bash
rustree --help
```

This will display all available arguments and their descriptions.

Explore the [Command-Line Interface (CLI)](../cli_usage.md) section for more detailed information on all options.