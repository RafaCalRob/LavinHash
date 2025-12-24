# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-12-24

### Added
- Initial release of LavinHash
- Dual-Layer Adaptive Hashing (DLAH) algorithm implementation
- Phase I: Adaptive normalization (lazy, zero-copy)
- Phase II: Structural hash using Shannon entropy vectors
- Phase III: Content hash using BuzHash + Bloom filter
- Adaptive block sizing for constant signature size
- Adaptive modulus for preventing Bloom filter saturation
- Parallel processing support for files >1MB using Rayon
- WebAssembly (WASM) bindings for JavaScript/Node.js
- C-compatible FFI layer
- Comprehensive test suite with >80% coverage
- Benchmarking suite using Criterion
- Complete documentation and technical specification
- MIT License

### Features
- Fuzzy hash generation from arbitrary byte data
- Similarity comparison (0-100 score)
- Configurable alpha coefficient (structure vs content weight)
- Configurable parallel processing
- Configurable feature density via min_modulus
- Serialization/deserialization of fingerprints
- Cross-platform determinism (Linux, macOS, Windows, WASM)

### Performance
- O(n) time complexity for hash generation
- O(1) time complexity for comparison
- ~500 MB/s single-threaded throughput
- ~2 GB/s multi-threaded throughput (on modern CPUs)
- ~1.2 KB fingerprint size

## [Unreleased]

### Planned
- NEON SIMD support for ARM platforms
- AVX-512 support for latest Intel CPUs
- Python bindings (PyO3)
- Go bindings (cgo)
- Incremental hashing for streaming data
- GPU acceleration for batch processing

---

[1.0.0]: https://github.com/RafaCalRob/LavinHash/releases/tag/v1.0.0
