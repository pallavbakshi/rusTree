# Contributing

Contributions to RusTree are welcome! Whether it's bug reports, feature requests, documentation improvements, or code contributions, your help is appreciated.

## Ways to Contribute

- **Reporting Bugs:** If you find a bug, please open an issue on the GitHub repository. Include steps to reproduce, expected behavior, and actual behavior.
- **Suggesting Enhancements:** Have an idea for a new feature or an improvement to an existing one? Open an issue to discuss it.
- **Improving Documentation:** If you find parts of the documentation unclear or missing, feel free to suggest changes or submit a pull request.
- **Writing Code:** If you'd like to contribute code, please follow the guidelines below.

## Code Contributions

1. **Fork the Repository:** Start by forking the official RusTree repository on GitHub.
1. **Clone Your Fork:** Clone your forked repository to your local machine.
   ```bash
   git clone https://github.com/your-username/rustree.git
   cd rustree
   ```
1. **Create a Branch:** Create a new branch for your feature or bug fix.
   ```bash
   git checkout -b my-new-feature
   ```
1. **Make Changes:** Implement your changes.
   - Follow the existing code style. Consider using `rustfmt` to format your code (`cargo fmt`).
   - Add tests for any new functionality or bug fixes.
   - Ensure all tests pass (`cargo test`).
   - Update documentation (both `rustdoc` comments and `mdBook` if applicable).
1. **Commit Your Changes:** Commit your changes with a clear and descriptive commit message.
   ```bash
   git commit -am "Add some feature"
   ```
1. **Push to Your Fork:** Push your changes to your forked repository.
   ```bash
   git push origin my-new-feature
   ```
1. **Submit a Pull Request:** Open a pull request from your branch on your fork to the `main` branch of the official RusTree repository.
   - Provide a clear title and description for your pull request.
   - Reference any related issues.

## Development Setup

- Ensure you have Rust installed (see [rustup.rs](https://rustup.rs/)).
- To build the project: `cargo build`
- To run tests: `cargo test`
- To format code: `cargo fmt`
- To run linters (clippy): `cargo clippy`
- To build and view `rustdoc` API documentation: `cargo doc --open`
- To build and view `mdBook` documentation (from the project root):
  ```bash
  # Install mdbook if you haven't already: cargo install mdbook
  mdbook serve docs
  ```

Thank you for considering contributing to RusTree!
