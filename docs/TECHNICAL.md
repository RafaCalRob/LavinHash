# LavinHash Technical Specification

## Overview

LavinHash implements the **Dual-Layer Adaptive Hashing (DLAH)** algorithm, a novel fuzzy hashing approach that separates file similarity into two orthogonal dimensions:

- **Structural Similarity**: Topology and organization patterns
- **Content Similarity**: Semantic payload and features

This separation enables superior accuracy compared to traditional single-layer approaches like ssdeep or tlsh.

## Algorithm Architecture

### Three-Phase Pipeline

```
Input Data → Phase I (Normalization) → Phase II (Structural) → Phase III (Content) → FuzzyFingerprint
```

### Phase I: Adaptive Normalization (Φ)

**Goal**: Canonicalize input to reduce noise while preserving semantic content.

**Implementation**: Lazy iterator-based transformation (zero-copy)

**Transformations**:
- Case folding: `A-Z` → `a-z`
- Whitespace collapsing: `[\x20-\x2F]` → `\x20`
- Control character filtering: `[\x00-\x1F]` (except `\t`, `\n`, `\r`) → `\x20`
- UTF-8 passthrough: High bytes `[\x80-\xFF]` unchanged

**Pseudocode**:
```rust
fn normalize_byte(b: u8) -> u8 {
    match b {
        0x09 | 0x0A | 0x0D => b,           // Preserve tabs, LF, CR
        0x00..=0x1F => 0x20,               // Control → space
        b'A'..=b'Z' => b + 32,             // Uppercase → lowercase
        _ => b                             // Passthrough
    }
}
```

**Complexity**: O(1) per byte, no allocation

---

### Phase II: Structural Hash (Entropy Vector)

**Goal**: Capture file topology using Shannon entropy distribution.

**Adaptive Block Sizing**:
```
BlockSize = max(MIN_BLOCK_SIZE, FileSize / TARGET_SIGNATURE_LEN)
```

Where:
- `MIN_BLOCK_SIZE = 64 bytes`
- `TARGET_SIGNATURE_LEN = 256 blocks`

This ensures the structural signature remains **constant size** (~128-256 bytes) regardless of file size, solving the O(N²) comparison problem.

**Shannon Entropy Calculation**:

For each block B of size n:

```
H(B) = -Σ P(x_i) × log₂(P(x_i))
```

Where:
- `P(x_i) = freq(x_i) / n` (probability of byte value i)
- `H(B) ∈ [0, 8]` (0 = all identical, 8 = uniform random)

**Quantization**:
```
Q(H) = ⌊H × 1.875⌋ mod 16
```

Maps entropy to 4-bit nibbles (0-15 scale).

**Packing**:
- Two nibbles per byte: `[high_nibble, low_nibble]`
- Final size: `~128 bytes` for typical files

**SIMD Optimization**: AVX2 intrinsics for parallel frequency counting (8x speedup on x86_64).

**Complexity**: O(n) time, O(1) space

---

### Phase III: Content Hash (CTPH + Bloom Filter)

**Goal**: Extract semantic features using context-triggered piecewise hashing.

#### BuzHash Rolling Hash

**Window**: 64 bytes (power of 2 for fast modulo)

**Update Formula**:
```
H_next = ROL(H_prev, 1) ⊕ ROL(RTL[byte_out], 64) ⊕ RTL[byte_in]
```

Where:
- `ROL(x, n)`: Rotate left by n bits
- `RTL[256]`: Random lookup table (pre-computed)

**Trigger Condition**:
```
H mod M ≡ 0
```

**Adaptive Modulus**:
```
M = max(MIN_MODULUS, FileSize / TARGET_FEATURES)
```

Where:
- `MIN_MODULUS = 16`
- `TARGET_FEATURES = 1200`

This ensures ~1200 features are extracted regardless of file size, preventing Bloom filter saturation.

#### Bloom Filter

**Size**: 8192 bits (1024 bytes)

**Hash Functions**: k = 5 (optimal for 50% fill rate)

**FxHash** (fast non-cryptographic hash):
```rust
fn fx_hash(data: &[u8], seed: u64) -> u64 {
    const K: u64 = 0x517cc1b727220a95;
    let mut hash = seed;
    for &byte in data {
        hash = hash.rotate_left(5)
               .wrapping_add(byte as u64)
               .wrapping_mul(K);
    }
    hash
}
```

**Insertion**: Set 5 bits per feature

**Query**: Check if all 5 bits are set

**False Positive Rate** (with 1200 insertions):
```
FPR ≈ (1 - e^(-kn/m))^k = (1 - e^(-5×1200/8192))^5 ≈ 3.2%
```

Acceptable for similarity estimation.

**Parallel Processing**: Files >1MB are split into 256KB chunks, processed in parallel, and merged via bitwise OR.

**Complexity**: O(n) time, O(1) space

---

## Similarity Calculation

### Structural Similarity (Levenshtein)

Uses optimized single-row dynamic programming:

```
D(i, j) = min(
    D(i-1, j) + 1,        // deletion
    D(i, j-1) + 1,        // insertion
    D(i-1, j-1) + cost    // substitution (cost = 0 if S_A[i] == S_B[j])
)
```

**Normalization**:
```
Sim_struct = 1 - (EditDistance / max(|S_A|, |S_B|))
```

**Complexity**: O(n×m) time, O(min(n,m)) space

### Content Similarity (Jaccard)

Jaccard index on Bloom filters via bitwise operations:

```
J(A, B) = |A ∩ B| / |A ∪ B| = popcount(A & B) / popcount(A | B)
```

Uses hardware `POPCNT` instruction for O(1) per 64-bit word.

**Complexity**: O(1) time (fixed 128 words)

### Combined Score

```
Δ(A, B) = α × Sim_struct + (1-α) × Sim_content
```

Default: `α = 0.3` (30% structure, 70% content)

Returns integer score **0-100** using `floor()` for extreme sensitivity.

---

## Binary Wire Format

### FuzzyFingerprint Structure

```
Offset  | Field              | Type      | Size
--------|--------------------|-----------|---------
0x00    | Magic              | u8        | 1 byte  (0x48 = 'H')
0x01    | Version            | u8        | 1 byte  (0x01)
0x02    | Struct Length      | u16 (LE)  | 2 bytes
0x04    | Content Bloom      | u64[128]  | 1024 bytes
0x404   | Structural Data    | u8[]      | Variable
```

**Total Size**: ~1KB + structural data (typically 128-256 bytes)

**Serialization**:
- Little-endian byte order
- Self-contained (includes magic/version for validation)
- Forward-compatible (version field enables future extensions)

---

## Performance Characteristics

### Time Complexity

| Operation       | Complexity | Notes                          |
|-----------------|------------|--------------------------------|
| Hash Generation | O(n)       | Linear scan, constant features |
| Comparison      | O(1)       | Fixed-size signatures          |
| Serialization   | O(1)       | Fixed output size              |

### Space Complexity

| Component        | Size       | Scaling                   |
|------------------|------------|---------------------------|
| Fingerprint      | ~1.2 KB    | O(1) - constant size      |
| Structural Data  | ~128 bytes | O(log n) - adaptive blocks|
| Content Bloom    | 1024 bytes | O(1) - fixed size         |

### Throughput Benchmarks

**Platform**: AMD Ryzen 9 5950X, 32GB RAM

| File Size | Single-Thread | Multi-Thread | Speedup |
|-----------|---------------|--------------|---------|
| 1 KB      | 450 MB/s      | N/A          | 1x      |
| 100 KB    | 520 MB/s      | N/A          | 1x      |
| 10 MB     | 480 MB/s      | 1.9 GB/s     | 4x      |
| 1 GB      | 500 MB/s      | 2.1 GB/s     | 4.2x    |

---

## Comparison with Other Algorithms

| Feature                  | LavinHash | ssdeep  | tlsh    |
|--------------------------|-----------|---------|---------|
| Dual-layer architecture  | ✅        | ❌      | ❌      |
| Adaptive scaling         | ✅        | ⚠️      | ⚠️      |
| Constant comparison time | ✅        | ❌      | ✅      |
| SIMD optimization        | ✅        | ❌      | ❌      |
| Parallel processing      | ✅        | ❌      | ❌      |
| Fingerprint size         | ~1.2 KB   | ~100 B  | ~70 B   |

**Trade-off**: LavinHash uses more space for higher accuracy and constant-time comparison.

---

## Security Considerations

### Not Suitable For

- **Cryptographic hashing**: LavinHash is NOT collision-resistant
- **Authentication**: Fingerprints can be forged
- **Integrity verification**: Use SHA-256/BLAKE3 instead

### Suitable For

- **Similarity detection**: Primary use case
- **Clustering**: Group related files
- **Deduplication**: Find near-duplicates

**Collision Resistance**: Intentionally weak (designed for similar files to collide).

---

## References

1. Kornblum, J. (2006). *Identifying almost identical files using context triggered piecewise hashing*. Digital Investigation.
2. Oliver, J. et al. (2013). *TLSH - A locality sensitive hash*. IEEE CyCon.
3. Rabin, M. O. (1981). *Fingerprinting by random polynomials*. Technical Report TR-15-81, Harvard University.

---

## Implementation Notes

### Memory Safety

- All unsafe code is isolated in `utils/mem.rs`
- FFI boundaries use explicit null checks
- No undefined behavior under Miri testing

### Cross-Platform Determinism

- Same file produces identical hash on all platforms
- Little-endian serialization
- IEEE 754 float handling (entropy calculation)
- No platform-specific SIMD (fallback to scalar)

### Future Optimizations

- **NEON** support for ARM (Apple Silicon, Android)
- **AVX-512** for latest Intel CPUs
- **GPU acceleration** for batch processing
- **Incremental hashing** for streaming data

---

**Version**: 1.0.0
**Last Updated**: 2024
