# LavinHash

[![Crates.io](https://img.shields.io/crates/v/lavinhash)](https://crates.io/crates/lavinhash)
[![npm](https://img.shields.io/npm/v/lavinhash)](https://www.npmjs.com/package/lavinhash)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

High-performance fuzzy hashing library implementing the **DLAH (Dual-Layer Adaptive Hashing)** algorithm for detecting file similarity with extreme efficiency.

**[Demo](https://bdovenbird.com/lavinhash/)** | **[Documentation](https://docs.rs/lavinhash)** | **[Crate](https://crates.io/crates/lavinhash)**

## Features

- **Dual-Layer Architecture**: Separates structural (topology) and content (semantic) similarity
- **Adaptive Scaling**: Maintains constant-time comparison regardless of file size
- **High Performance**: Rust core with SIMD optimizations and parallel processing
- **WebAssembly Support**: Run in browsers and Node.js
- **Cross-Platform**: Works on Linux, macOS, Windows, and WASM targets
- **FFI Ready**: C-compatible API for integration with other languages

## How It Works

LavinHash implements a three-phase pipeline:

1. **Phase I - Normalization**: Case folding, whitespace collapsing (lazy, zero-copy)
2. **Phase II - Structural Hash**: Shannon entropy vectors with adaptive block sizing
3. **Phase III - Content Hash**: BuzHash rolling hash + Bloom filter (8192 bits)

The similarity score combines both layers:

```
Similarity = α × Levenshtein(Structure) + (1-α) × Jaccard(Content)
```

Where α = 0.3 by default (30% structure, 70% content).

## Installation

### Rust

```toml
[dependencies]
lavinhash = "1.0"
```

### JavaScript/TypeScript (npm)

```bash
npm install lavinhash
```

### From Source

```bash
git clone https://github.com/RafaCalRob/LavinHash.git
cd LavinHash
cargo build --release
```

## Quick Start

### Rust

```rust
use lavinhash::{generate_hash, compare_hashes, HashConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data1 = b"The quick brown fox jumps over the lazy dog";
    let data2 = b"The quick brown fox leaps over the lazy dog";

    let config = HashConfig::default();

    let hash1 = generate_hash(data1, &config)?;
    let hash2 = generate_hash(data2, &config)?;

    let similarity = compare_hashes(&hash1, &hash2, 0.3);
    println!("Similarity: {}%", similarity);

    Ok(())
}
```

### JavaScript/Node.js

```javascript
import init, { generate_hash, compare_hashes } from 'lavinhash';

await init();

const encoder = new TextEncoder();
const data1 = encoder.encode("The quick brown fox jumps over the lazy dog");
const data2 = encoder.encode("The quick brown fox leaps over the lazy dog");

const hash1 = generate_hash(data1);
const hash2 = generate_hash(data2);

const similarity = compare_hashes(hash1, hash2);
console.log(`Similarity: ${similarity}%`);
```

## Configuration

```rust
let mut config = HashConfig::default();
config.alpha = 0.5;              // 50% structure, 50% content
config.enable_parallel = true;   // Parallel processing for files >1MB
config.min_modulus = 16;         // Feature density control
```

## Performance

- **Time Complexity**: O(n) for hashing, O(1) for comparison (constant signature size)
- **Space Complexity**: ~1KB per fingerprint + O(log n) structural data
- **Throughput**: ~500 MB/s single-threaded, ~2 GB/s multi-threaded (on modern CPUs)

## Use Cases

- **Duplicate Detection**: Find similar files in large datasets
- **Plagiarism Detection**: Compare documents for content similarity
- **Version Control**: Identify related versions of files
- **Malware Analysis**: Group similar malware samples
- **Data Deduplication**: Reduce storage by identifying redundant files

## Algorithm Details

### Structural Hash (Phase II)

- Adaptive block sizing: `BlockSize = max(64, FileSize / 256)`
- Shannon entropy quantized to 4-bit nibbles (0-15 scale)
- Ensures constant signature size (~128-256 blocks)

### Content Hash (Phase III)

- BuzHash rolling window: 64 bytes
- Adaptive modulus: `M = FileSize / 1200` (targets ~1200 features)
- Bloom filter: 8192 bits (1KB) with 5 hash functions
- Prevents saturation even for GB-scale files

## Building WASM

```bash
wasm-pack build --target web --out-dir wasm --out-name lavinhash
```

## Testing

```bash
cargo test
cargo bench
```

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) first.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by ssdeep and tlsh fuzzy hashing algorithms
- Built with Rust for maximum performance and safety
- WASM bindings powered by wasm-bindgen

## Citation

If you use LavinHash in academic work, please cite:

```bibtex
@software{lavinhash2024,
  title={LavinHash: High-Performance Fuzzy Hashing with Dual-Layer Adaptive Hashing},
  author={LavinHash Contributors},
  year={2024},
  url={https://github.com/RafaCalRob/LavinHash}
}
```

## Links

- **Homepage**: https://bdovenbird.com/lavinhash/
- **Documentation**: https://docs.rs/lavinhash
- **Repository**: https://github.com/RafaCalRob/LavinHash
- **Issues**: https://github.com/RafaCalRob/LavinHash/issues
