# git-stack
git-stack is a developer productivity tool that makes stacked pull requests simple, intuitive, and GitHub‑native. It streamlines the workflow of creating, managing, and merging dependent branches, so you can focus on writing code instead of wrestling with rebases and PR chains.

## Building from Source

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build Instructions

Build the project:
```bash
cargo build
```

Run the application:
```bash
cargo run
```

Build optimized release version:
```bash
cargo build --release
```

Run tests:
```bash
cargo test
```

### CLI Usage

After building, you can run the binary directly:
```bash
# Debug build
./target/debug/git-stack

# Release build
./target/release/git-stack
```

Both will output "Hello World" to stdout.
