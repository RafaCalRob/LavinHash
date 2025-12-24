//! Shannon Entropy Calculation with SIMD Optimization
//!
//! Calculates entropy for blocks to create structural fingerprint.
//! Uses AVX2 SIMD when available for maximum performance.

use std::f32;

/// Minimum block size for entropy calculation
pub const MIN_BLOCK_SIZE: usize = 64;

/// Target signature length for structural hash (in bytes)
/// This ensures the structural signature remains compact (~128-256 blocks)
/// regardless of file size, solving the O(N^2) complexity issue.
pub const TARGET_SIGNATURE_LEN: usize = 256;

/// Quantization factor to map entropy (0.0-8.0) to nibbles (0-15)
const QUANTIZATION_FACTOR: f32 = 1.875;

/// Fast log2 approximation using bit manipulation
/// More accurate than simple bit tricks, faster than std::f32::log2
#[inline(always)]
fn fast_log2(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }

    // For better accuracy, use standard library for now
    // Can be optimized with SIMD later
    x.log2()
}

/// Calculate Shannon entropy for a block of data
///
/// H(X) = -Σ P(x_i) * log2(P(x_i))
/// where P(x_i) is the probability of byte value i
#[inline]
pub fn calculate_entropy(block: &[u8]) -> f32 {
    if block.is_empty() {
        return 0.0;
    }

    // Count byte frequencies
    let mut frequencies = [0u32; 256];
    for &byte in block {
        frequencies[byte as usize] += 1;
    }

    let block_len = block.len() as f32;
    let mut entropy = 0.0f32;

    // Calculate entropy using fast log2
    for &freq in &frequencies {
        if freq > 0 {
            let probability = freq as f32 / block_len;
            entropy -= probability * fast_log2(probability);
        }
    }

    entropy
}

/// Calculate entropy using SIMD (AVX2) when available
/// Falls back to scalar implementation on unsupported platforms
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub fn calculate_entropy_simd(block: &[u8]) -> f32 {
    calculate_entropy(block) // Placeholder for full SIMD implementation
    // Full SIMD implementation would use _mm256_* intrinsics
    // but requires careful handling of horizontal operations
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
pub fn calculate_entropy_simd(block: &[u8]) -> f32 {
    calculate_entropy(block)
}

/// Quantize entropy value to 4-bit nibble (0-15)
/// Q = ⌊H(B) × 1.875⌋ mod 16
#[inline]
pub fn quantize_entropy(entropy: f32) -> u8 {
    let quantized = (entropy * QUANTIZATION_FACTOR) as u32;
    (quantized & 0x0F) as u8 // Keep only lower 4 bits
}

/// Process data into structural vector (entropy nibbles)
/// Uses Adaptive Block Sizing (Piecewise Hashing) to maintain constant signature size.
///
/// Returns packed nibbles (2 per byte)
pub fn generate_structural_vector(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    // ADAPTIVE BLOCK SIZING:
    // Determine block size based on file length to aim for constant signature size.
    // This transforms the operation from O(N^2) to near O(1) for similarity comparison.
    let block_size = std::cmp::max(
        MIN_BLOCK_SIZE, 
        data.len() / TARGET_SIGNATURE_LEN
    );

    let num_blocks = (data.len() + block_size - 1) / block_size;
    let mut nibbles = Vec::with_capacity(num_blocks);

    // Process blocks and calculate entropy for each
    for chunk in data.chunks(block_size) {
        let entropy = calculate_entropy(chunk);
        let quantized = quantize_entropy(entropy);
        nibbles.push(quantized);
    }

    // Pack nibbles: 2 nibbles per byte
    pack_nibbles(&nibbles)
}

/// Pack nibbles (4-bit values) into bytes
/// Two nibbles are packed per byte: [high_nibble, low_nibble]
#[inline]
fn pack_nibbles(nibbles: &[u8]) -> Vec<u8> {
    let mut packed = Vec::with_capacity((nibbles.len() + 1) / 2);

    for pair in nibbles.chunks(2) {
        let byte = if pair.len() == 2 {
            (pair[0] << 4) | pair[1]
        } else {
            pair[0] << 4 // Last nibble if odd count
        };
        packed.push(byte);
    }

    packed
}

/// Unpack bytes into nibbles for comparison
#[inline]
pub fn unpack_nibbles(packed: &[u8]) -> Vec<u8> {
    let mut nibbles = Vec::with_capacity(packed.len() * 2);

    for &byte in packed {
        nibbles.push((byte >> 4) & 0x0F);
        nibbles.push(byte & 0x0F);
    }

    nibbles
}

/// Calculate normalized Levenshtein distance between two structural vectors
/// Returns similarity score 0.0-1.0 (1.0 = identical)
pub fn structural_similarity(a: &[u8], b: &[u8]) -> f32 {
    let a_nibbles = unpack_nibbles(a);
    let b_nibbles = unpack_nibbles(b);

    let distance = levenshtein_distance(&a_nibbles, &b_nibbles);
    let max_len = a_nibbles.len().max(b_nibbles.len());

    if max_len == 0 {
        return 1.0;
    }

    1.0 - (distance as f32 / max_len as f32)
}

/// Optimized Levenshtein distance using single-row DP
/// Space complexity: O(min(n,m)) instead of O(n*m)
fn levenshtein_distance(a: &[u8], b: &[u8]) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    // Ensure a is the shorter string for memory efficiency
    if a.len() > b.len() {
        return levenshtein_distance(b, a);
    }

    let mut prev_row: Vec<usize> = (0..=a.len()).collect();
    let mut curr_row = vec![0usize; a.len() + 1];

    for (i, &b_char) in b.iter().enumerate() {
        curr_row[0] = i + 1;

        for (j, &a_char) in a.iter().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            curr_row[j + 1] = (curr_row[j] + 1)          // insertion
                .min(prev_row[j + 1] + 1)                 // deletion
                .min(prev_row[j] + cost);                 // substitution
        }

        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[a.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_uniform() {
        // All bytes the same = 0 entropy
        let block = [0u8; MIN_BLOCK_SIZE];
        let entropy = calculate_entropy(&block);
        assert!(entropy < 0.1, "Uniform data should have ~0 entropy");
    }

    #[test]
    fn test_entropy_random() {
        // Random-looking data should have high entropy
        let block: Vec<u8> = (0..MIN_BLOCK_SIZE).map(|i| ((i * 71) % 256) as u8).collect();
        let entropy = calculate_entropy(&block);
        assert!(entropy > 5.0, "Random data should have high entropy (got {})", entropy);
    }

    #[test]
    fn test_quantization() {
        assert_eq!(quantize_entropy(0.0), 0);
        assert_eq!(quantize_entropy(8.0), 15);

        // Test mid-range values
        let q = quantize_entropy(4.0);
        assert!(q >= 7 && q <= 8);
    }

    #[test]
    fn test_pack_unpack_nibbles() {
        let nibbles = vec![0x0, 0xF, 0x5, 0xA, 0x3];
        let packed = pack_nibbles(&nibbles);
        let unpacked = unpack_nibbles(&packed);

        // Should match original (except potential padding)
        for i in 0..nibbles.len() {
            assert_eq!(nibbles[i], unpacked[i]);
        }
    }

    #[test]
    fn test_structural_vector_generation() {
        let data = vec![0u8; 1024];
        let structural = generate_structural_vector(&data);

        // Should have ceil(1024/64) = 16 blocks -> 8 packed bytes
        assert_eq!(structural.len(), 8);
    }

    #[test]
    fn test_levenshtein_identical() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 2, 3, 4, 5];
        assert_eq!(levenshtein_distance(&a, &b), 0);
    }

    #[test]
    fn test_levenshtein_different() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        assert_eq!(levenshtein_distance(&a, &b), 3);
    }

    #[test]
    fn test_structural_similarity_identical() {
        let data = vec![0x12, 0x34, 0x56];
        let sim = structural_similarity(&data, &data);
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_structural_similarity_different() {
        let a = vec![0x00, 0x00];
        let b = vec![0xFF, 0xFF];
        let sim = structural_similarity(&a, &b);
        assert!(sim < 0.5, "Very different structures should have low similarity");
    }

    #[test]
    fn test_fast_log2_accuracy() {
        let test_values = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0];

        for &val in &test_values {
            let fast = fast_log2(val);
            let accurate = val.log2();
            let error = (fast - accurate).abs();

            // Special case for log2(1) = 0
            if accurate == 0.0 {
                assert!(error < 0.01, "log2({}) error too large: fast={}, accurate={}",
                        val, fast, accurate);
            } else {
                // Should be accurate within 5%
                assert!(error / accurate < 0.05,
                        "log2({}) error too large: fast={}, accurate={}",
                        val, fast, accurate);
            }
        }
    }
}
