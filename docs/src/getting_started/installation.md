## Installation

RusTree is a Rust application and can be installed using Cargo, Rust's package manager.

### Prerequisites

Ensure you have Rust and Cargo installed. If not, please visit [rustup.rs](https://rustup.rs/) to install them.

### Installing from Crates.io (Recommended)

Once RusTree is published to [crates.io](https://crates.io/), you can install it directly using:

```bash
cargo install rustree
```

This will download the source code, compile it, and place the `rustree` executable in your Cargo binary directory (usually `~/.cargo/bin/`). Make sure this directory is in your system's `PATH`.

### Building from Source

If you want to build from the latest source code (e.g., from a Git repository):

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/rustree.git # Replace with actual URL
   cd rustree
   ```

1. Build and install the binary:

   ```bash
   cargo install --path .
   ```

   Alternatively, to just build for development:

   ```bash
   cargo build --release
   ```

   The executable will be located at `target/release/rustree`.

### Verifying Installation

After installation, you should be able to run:

```bash
rustree --version
```

This command should print the installed version of RusTree. If you see an error, ensure `~/.cargo/bin` is in your `PATH` or try opening a new terminal session.
