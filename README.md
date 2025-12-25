# LavinHash

High-performance fuzzy hashing library implementing the Dual-Layer Adaptive Hashing (DLAH) algorithm for detecting file and text similarity.

## Overview

LavinHash is a Rust-based fuzzy hashing library that analyzes both structural patterns and content features to compute similarity scores between data. The library uses a dual-layer approach that separates structural similarity (topology) from content similarity (semantic features), providing accurate similarity detection even for modified or partially similar data.

**Key Features:**

- Dual-layer similarity analysis (structure + content)
- Adaptive scaling for constant-time comparison regardless of file size
- Cross-platform support (Linux, macOS, Windows, WebAssembly)
- High performance with SIMD optimizations and parallel processing
- Multiple language bindings (JavaScript/TypeScript, with more planned)
- Deterministic hashing across all platforms

## Installation

### JavaScript/TypeScript (npm)

```bash
npm install lavinhash
```

### Rust (crates.io)

```toml
[dependencies]
lavinhash = "1.0"
```

### Building from Source

```bash
git clone https://github.com/RafaCalRob/LavinHash.git
cd LavinHash
cargo build --release
```

## Quick Start

### JavaScript/Node.js (CommonJS)

```javascript
const { wasm_compare_data, wasm_generate_hash } = require('lavinhash');

// Compare two texts directly
const encoder = new TextEncoder();
const text1 = encoder.encode("The quick brown fox jumps over the lazy dog");
const text2 = encoder.encode("The quick brown fox leaps over the lazy dog");

const similarity = wasm_compare_data(text1, text2);
console.log(`Similarity: ${similarity}%`); // Output: Similarity: 95%
```

### JavaScript with Bundlers (Webpack, Vite, Rollup)

If you're using a modern bundler, you can use ES modules:

```javascript
import { wasm_compare_data, wasm_generate_hash } from 'lavinhash';

const encoder = new TextEncoder();
const text1 = encoder.encode("Sample text");
const text2 = encoder.encode("Sample text modified");

const similarity = wasm_compare_data(text1, text2);
console.log(`Similarity: ${similarity}%`);
```

Note: For bundler support, rebuild with `wasm-pack build --target bundler`

### Rust

```rust
use lavinhash::{generate_hash, compare_hashes, HashConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data1 = b"Document content version 1";
    let data2 = b"Document content version 2";

    let config = HashConfig::default();

    let hash1 = generate_hash(data1, &config)?;
    let hash2 = generate_hash(data2, &config)?;

    let similarity = compare_hashes(&hash1, &hash2, 0.3);
    println!("Similarity: {}%", similarity);

    Ok(())
}
```

## API Reference

### JavaScript/WASM API

#### `wasm_generate_hash(data: Uint8Array): Uint8Array`

Generates a fuzzy hash fingerprint from input data.

**Parameters:**
- `data`: Input data as Uint8Array

**Returns:**
- Serialized fingerprint (approximately 1KB)

**Example:**
```javascript
const data = encoder.encode("Text to hash");
const hash = wasm_generate_hash(data);
console.log(`Hash size: ${hash.length} bytes`);
```

#### `wasm_compare_hashes(hash_a: Uint8Array, hash_b: Uint8Array): number`

Compares two previously generated hashes.

**Parameters:**
- `hash_a`: First fingerprint
- `hash_b`: Second fingerprint

**Returns:**
- Similarity score (0-100)

**Example:**
```javascript
const hash1 = wasm_generate_hash(data1);
const hash2 = wasm_generate_hash(data2);
const similarity = wasm_compare_hashes(hash1, hash2);
```

#### `wasm_compare_data(data_a: Uint8Array, data_b: Uint8Array): number`

Generates hashes and compares in a single operation.

**Parameters:**
- `data_a`: First data array
- `data_b`: Second data array

**Returns:**
- Similarity score (0-100)

**Example:**
```javascript
const similarity = wasm_compare_data(text1, text2);
```

### Rust API

#### `generate_hash(data: &[u8], config: &HashConfig) -> Result<FuzzyFingerprint, FingerprintError>`

Generates a fuzzy hash from input data.

**Parameters:**
- `data`: Input data slice
- `config`: Configuration options

**Returns:**
- `Ok(FuzzyFingerprint)`: Generated fingerprint
- `Err(FingerprintError)`: Error if data is invalid

#### `compare_hashes(hash_a: &FuzzyFingerprint, hash_b: &FuzzyFingerprint, alpha: f32) -> u8`

Compares two fingerprints.

**Parameters:**
- `hash_a`: First fingerprint
- `hash_b`: Second fingerprint
- `alpha`: Weight coefficient (0.0-1.0, default 0.3)

**Returns:**
- Similarity score (0-100)

#### `HashConfig`

Configuration structure for hash generation.

**Fields:**
- `enable_parallel: bool` - Enable parallel processing for large files (default: true)
- `alpha: f32` - Weight for structure vs content (default: 0.3)
- `min_modulus: u64` - Feature density control (default: 16)

**Example:**
```rust
let mut config = HashConfig::default();
config.alpha = 0.5;  // 50% structure, 50% content
config.enable_parallel = false;  // Disable parallel processing
```

## Algorithm Details

### DLAH (Dual-Layer Adaptive Hashing)

LavinHash implements a three-phase pipeline:

**Phase I: Adaptive Normalization**
- Case folding (A-Z to a-z)
- Whitespace normalization
- Control character filtering
- Zero-copy iterator-based processing

**Phase II: Structural Hash**
- Shannon entropy calculation with adaptive block sizing
- Quantization to 4-bit nibbles
- Compact vector representation
- Levenshtein distance for comparison

**Phase III: Content Hash**
- BuzHash rolling hash algorithm
- Adaptive modulus scaling
- 8192-bit Bloom filter (1KB)
- Jaccard similarity for comparison

### Similarity Formula

```
Similarity = α × Levenshtein(Structure) + (1-α) × Jaccard(Content)
```

Where:
- `α = 0.3` (default) - 30% weight to structure, 70% to content
- Levenshtein: Normalized edit distance on entropy vectors
- Jaccard: Set similarity on Bloom filter features

### Performance Characteristics

**Time Complexity:**
- Hash generation: O(n) where n is data size
- Hash comparison: O(1) - constant time regardless of file size

**Space Complexity:**
- Fingerprint size: ~1KB + O(log n) structural data
- Memory usage: O(1) for comparison, O(n) for generation

**Throughput:**
- Single-threaded: ~500 MB/s
- Multi-threaded: ~2 GB/s (files larger than 1MB)

## Configuration

### Basic Configuration

```rust
use lavinhash::HashConfig;

let config = HashConfig {
    enable_parallel: true,
    alpha: 0.3,
    min_modulus: 16,
};
```

### Advanced Configuration

**Adjusting Structure vs Content Weight:**

```rust
// More weight to structure (topology)
config.alpha = 0.5;  // 50% structure, 50% content

// More weight to content (features)
config.alpha = 0.1;  // 10% structure, 90% content
```

**Controlling Feature Density:**

```rust
// Higher sensitivity (more features)
config.min_modulus = 8;

// Lower sensitivity (fewer features)
config.min_modulus = 32;
```

**Parallel Processing:**

```rust
// Force sequential processing
config.enable_parallel = false;

// Enable automatic parallel processing for files > 1MB
config.enable_parallel = true;
```

## Use Cases

### Document Similarity Detection

Compare different versions of documents to detect modifications and measure similarity.

```javascript
const { readFileSync } = require('fs');
const { wasm_compare_data } = require('lavinhash');

const doc1 = readFileSync('document_v1.txt');
const doc2 = readFileSync('document_v2.txt');
const similarity = wasm_compare_data(doc1, doc2);
console.log(`Similarity: ${similarity}%`);
```

### Duplicate Detection

Identify duplicate or near-duplicate files in large datasets.

```rust
let files = vec![file1, file2, file3];
let hashes: Vec<_> = files.iter()
    .map(|f| generate_hash(f, &config).unwrap())
    .collect();

for i in 0..hashes.len() {
    for j in i+1..hashes.len() {
        let sim = compare_hashes(&hashes[i], &hashes[j], 0.3);
        if sim > 90 {
            println!("Files {} and {} are similar: {}%", i, j, sim);
        }
    }
}
```

### Version Tracking

Track changes between different versions of files or content.

```javascript
const { readFileSync } = require('fs');
const { wasm_generate_hash, wasm_compare_hashes } = require('lavinhash');

const versions = ['v1.txt', 'v2.txt', 'v3.txt'];
const hashes = versions.map(v => {
    const data = readFileSync(v);
    return wasm_generate_hash(data);
});

for (let i = 0; i < hashes.length - 1; i++) {
    const sim = wasm_compare_hashes(hashes[i], hashes[i + 1]);
    console.log(`${versions[i]} -> ${versions[i+1]}: ${sim}% similar`);
}
```

## Building WASM

To build the WebAssembly bindings:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for Node.js (CommonJS, recommended for npm)
wasm-pack build --target nodejs --out-dir pkg --out-name lavinhash

# Or build for bundlers (Webpack, Vite, Rollup)
wasm-pack build --target bundler --out-dir pkg --out-name lavinhash

# The compiled files will be in the pkg/ directory
```

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_generate_hash_basic
```

### Benchmarks

```bash
# Run benchmarks
cargo bench
```

## Technical Specifications

**Fingerprint Format:**

```
Offset | Field            | Type     | Size
-------|------------------|----------|-------------
0x00   | Magic            | u8       | 1 byte (0x48)
0x01   | Version          | u8       | 1 byte (0x01)
0x02   | Struct Length    | u16 LE   | 2 bytes
0x04   | Content Bloom    | u64[128] | 1024 bytes
0x404  | Structural Data  | u8[]     | Variable
```

**Cross-Platform Determinism:**
- Identical input produces identical hash on all platforms
- Little-endian byte ordering
- IEEE 754 floating-point compliance

**Thread Safety:**
- Hash generation is thread-safe
- Parallel processing uses Rayon for data parallelism
- No global state or locks

## Examples

See the `examples/` directory for complete working examples:

- `basic_usage.rs` - Rust usage examples
- `javascript_example.js` - Node.js integration
- `browser_example.html` - Browser-based demo

## Documentation

- **API Documentation**: Available at [docs.rs/lavinhash](https://docs.rs/lavinhash)
- **Technical Specification**: See `docs/TECHNICAL.md` in the repository
- **Contributing Guide**: See `CONTRIBUTING.md`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

Source code: [https://github.com/RafaCalRob/LavinHash](https://github.com/RafaCalRob/LavinHash)

## Support

For bug reports and feature requests, please open an issue on GitHub:
[https://github.com/RafaCalRob/LavinHash/issues](https://github.com/RafaCalRob/LavinHash/issues)
