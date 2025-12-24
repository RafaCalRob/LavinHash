# Contributing to LavinHash

Thank you for your interest in contributing to LavinHash! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Respect different viewpoints and experiences

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/RafaCalRob/LavinHash/issues)
2. Create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)
   - Code samples if applicable

### Suggesting Enhancements

1. Check existing issues and discussions
2. Open a new issue describing:
   - The enhancement goal
   - Use cases
   - Proposed implementation (optional)

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/YourFeature`
3. Make your changes following the code style
4. Add tests for new functionality
5. Ensure all tests pass: `cargo test`
6. Run benchmarks if performance-critical: `cargo bench`
7. Update documentation as needed
8. Commit with clear messages
9. Push to your fork
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+ (`rustup install stable`)
- wasm-pack (for WASM builds): `cargo install wasm-pack`
- Node.js 14+ (for npm testing)

### Building

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/LavinHash.git
cd LavinHash

# Build Rust library
cargo build --release

# Build WASM
wasm-pack build --target web --out-dir wasm --out-name lavinhash

# Run tests
cargo test
cargo test --release

# Run benchmarks
cargo bench
```

### Code Style

- Follow Rust style guide (use `rustfmt`)
- Run `cargo clippy` and fix warnings
- Write documentation for public APIs
- Add unit tests for new functions
- Keep functions focused and small

### Testing

- Write unit tests in the same file as the code
- Use integration tests in `tests/` for end-to-end scenarios
- Aim for >80% code coverage
- Test edge cases (empty input, large files, etc.)

### Commit Messages

Follow conventional commits:

```
feat: add new feature
fix: fix bug
docs: update documentation
test: add tests
refactor: refactor code
perf: improve performance
chore: maintenance tasks
```

Example:
```
feat: add parallel processing for large files

- Implement Rayon-based parallel chunking
- Add configuration option to enable/disable
- Benchmark shows 4x speedup on 10MB+ files
```

## Project Structure

```
LavinHash/
├── src/
│   ├── lib.rs           # Main library entry
│   ├── algo/            # Core algorithms
│   │   ├── bloom.rs     # Bloom filter
│   │   ├── buzhash.rs   # Rolling hash
│   │   └── entropy.rs   # Shannon entropy
│   ├── model/           # Data structures
│   │   └── fingerprint.rs
│   └── utils/           # Utilities
│       └── mem.rs       # FFI helpers
├── docs/                # Documentation
│   └── TECHNICAL.md     # Technical spec
├── examples/            # Usage examples
├── benches/             # Benchmarks
└── tests/               # Integration tests
```

## Performance Guidelines

- Profile before optimizing (`cargo flamegraph`)
- Use SIMD when beneficial (AVX2, NEON)
- Prefer stack allocation for small data
- Use `#[inline]` for hot paths
- Benchmark changes: `cargo bench --bench fuzzy_hash`

## Documentation

- Document all public APIs with `///` comments
- Include examples in documentation
- Update README.md for significant changes
- Add entries to CHANGELOG.md

## Release Process

1. Update version in `Cargo.toml` and `package.json`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v1.x.x`
4. Push tag: `git push origin v1.x.x`
5. Publish to crates.io: `cargo publish`
6. Publish to npm: `npm publish`

## Questions?

- Open a [Discussion](https://github.com/RafaCalRob/LavinHash/discussions)
- Join our community chat (if available)
- Email maintainers (see README.md)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
