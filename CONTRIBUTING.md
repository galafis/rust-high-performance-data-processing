# Contributing

Contributions are welcome! This guide explains how to get started.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/<your-username>/rust-high-performance-data-processing.git`
3. Create a feature branch: `git checkout -b feature/your-feature`

## Development Setup

- Install Rust 1.70+ via [rustup](https://rustup.rs/)
- Build the project: `cargo build`
- Run tests: `cargo test`

## Before Submitting

Run the full quality check:

```bash
cargo fmt --check     # Code formatting
cargo clippy -- -D warnings  # Linting
cargo test            # All tests
cargo bench           # Benchmarks (optional)
```

## Pull Request Process

1. Ensure all tests pass and there are no clippy warnings
2. Update documentation if you change public APIs
3. Add tests for new functionality
4. Keep commits focused and descriptive
5. Open a pull request against `main`

## Code Style

- Follow `rustfmt` defaults
- Add doc comments (`///`) to all public items
- Use `Result<T, E>` for fallible operations
- Prefer iterators over manual loops

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
