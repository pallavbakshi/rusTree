


## For Developers

### Code formatter and linter

```sh
# formatter
cargo fmt

# linter
cargo clippy
```

### Run code

```sh
cargo run
```

### Run tests

```sh
# Run all tests
cargo test

# Run only library tests (unit and integration)
cargo test --lib

# Run tests specifically for the binary (if you had any, typically less common unless the binary itself has complex logic not in the lib)
cargo test --bin rustree

# Run a specific test function
cargo test my_test_function_name

# Run all tests in that module
cargo test core::analyzer::file_stats
```