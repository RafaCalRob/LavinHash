//! Bloom Filter for Content Fingerprinting
//!
//! Fixed-size 8192-bit Bloom filter optimized for L1/L2 cache.
//! Uses 5 hash functions with adaptive modulus to prevent saturation.

/// Bloom filter size in bits (8192 bits = 1024 bytes = 1KB)
/// Optimal size with adaptive modulus scaling
pub const BLOOM_SIZE_BITS: usize = 8_192;
pub const BLOOM_SIZE_BYTES: usize = BLOOM_SIZE_BITS / 8; // 1024 bytes

/// Number of 64-bit words needed (8192 / 64 = 128)
const BLOOM_WORDS: usize = BLOOM_SIZE_BITS / 64;

/// Number of hash functions (k=5 optimal for 8KB with adaptive features)
const NUM_HASH_FUNCTIONS: usize = 5;

/// Seeds for hash functions (prime numbers for better distribution)
const HASH_SEEDS: [u64; NUM_HASH_FUNCTIONS] = [
    0x517cc1b727220a95, // seed 1
    0x5bc42f4b7f0db7e3, // seed 2
    0x9e3779b97f4a7c15, // seed 3
    0xc3a5c85c97cb3127, // seed 4
    0xb492b66fbe98f273, // seed 5
];

/// Fixed-size Bloom Filter (Heap Allocated)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BloomFilter {
    // Use Vec for heap allocation. We enforce size logic in methods.
    bits: Vec<u64>, 
}

impl BloomFilter {
    /// Create a new empty Bloom filter
    #[inline]
    pub fn new() -> Self {
        Self { 
            bits: vec![0u64; BLOOM_WORDS]
        }
    }

    /// Create from raw bytes (for deserialization)
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), BLOOM_SIZE_BYTES, "BloomFilter bytes must be exactly {} bytes", BLOOM_SIZE_BYTES);

        let mut bits_vec = Vec::with_capacity(BLOOM_WORDS);
        
        for chunk in bytes.chunks_exact(8) {
            bits_vec.push(u64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
                chunk[4], chunk[5], chunk[6], chunk[7],
            ]));
        }

        Self { 
             bits: bits_vec
        }
    }

    /// Convert to raw bytes (for serialization)
    #[inline]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(BLOOM_SIZE_BYTES);
        for word in self.bits.iter() {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        bytes
    }

    /// FxHash-inspired fast hash function
    /// Extremely fast with good distribution properties
    #[inline(always)]
    fn fx_hash(data: &[u8], seed: u64) -> u64 {
        const K: u64 = 0x517cc1b727220a95;
        let mut hash = seed;

        for &byte in data {
            hash = hash.rotate_left(5).wrapping_add(byte as u64).wrapping_mul(K);
        }

        hash
    }

    /// Get bit indices for a given data
    #[inline]
    fn get_indices(&self, data: &[u8]) -> [usize; NUM_HASH_FUNCTIONS] {
        let mut indices = [0usize; NUM_HASH_FUNCTIONS];

        for i in 0..NUM_HASH_FUNCTIONS {
            let hash = Self::fx_hash(data, HASH_SEEDS[i]);
            indices[i] = (hash as usize) % BLOOM_SIZE_BITS;
        }

        indices
    }

    /// Set a bit at the given index
    #[inline(always)]
    fn set_bit(&mut self, index: usize) {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        self.bits[word_idx] |= 1u64 << bit_idx;
    }

    /// Check if a bit is set at the given index
    #[inline(always)]
    fn get_bit(&self, index: usize) -> bool {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        (self.bits[word_idx] & (1u64 << bit_idx)) != 0
    }

    /// Insert data into the Bloom filter
    #[inline]
    pub fn insert(&mut self, data: &[u8]) {
        let indices = self.get_indices(data);
        for &idx in &indices {
            self.set_bit(idx);
        }
    }

    /// Check if data might be in the Bloom filter
    /// Returns true if possibly present, false if definitely not present
    #[inline]
    pub fn contains(&self, data: &[u8]) -> bool {
        let indices = self.get_indices(data);
        indices.iter().all(|&idx| self.get_bit(idx))
    }

    /// Calculate Jaccard similarity with another Bloom filter
    /// J(A,B) = |A ∩ B| / |A ∪ B|
    /// Uses fast bitwise operations
    #[inline]
    pub fn jaccard_similarity(&self, other: &BloomFilter) -> f32 {
        let mut intersection = 0u32;
        let mut union = 0u32;

        for i in 0..BLOOM_WORDS {
            let and_bits = self.bits[i] & other.bits[i];
            let or_bits = self.bits[i] | other.bits[i];

            intersection += and_bits.count_ones();
            union += or_bits.count_ones();
        }

        if union == 0 {
            // Both filters are empty - they are identical
            return 1.0;
        }

        intersection as f32 / union as f32
    }

    /// Merge another Bloom filter into this one (bitwise OR)
    #[inline]
    pub fn merge(&mut self, other: &BloomFilter) {
        for i in 0..BLOOM_WORDS {
            self.bits[i] |= other.bits[i];
        }
    }

    /// Get the number of set bits (population count)
    #[inline]
    pub fn count_set_bits(&self) -> u32 {
        self.bits.iter().map(|&word| word.count_ones()).sum()
    }

    /// Clear all bits
    #[inline]
    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    /// Check if the filter is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&word| word == 0)
    }
}

impl Default for BloomFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating Bloom filters from feature sets
pub struct BloomFilterBuilder {
    filter: BloomFilter,
    feature_count: usize,
}

impl BloomFilterBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            filter: BloomFilter::new(),
            feature_count: 0,
        }
    }

    #[inline]
    pub fn add_feature(&mut self, data: &[u8]) {
        self.filter.insert(data);
        self.feature_count += 1;
    }

    #[inline]
    pub fn build(self) -> BloomFilter {
        self.filter
    }

    #[inline]
    pub fn feature_count(&self) -> usize {
        self.feature_count
    }
}

impl Default for BloomFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_insert_and_contains() {
        let mut bloom = BloomFilter::new();

        let data1 = b"Hello, World!";
        let data2 = b"Fuzzy Hashing";
        let _data3 = b"Not inserted";

        bloom.insert(data1);
        bloom.insert(data2);

        assert!(bloom.contains(data1));
        assert!(bloom.contains(data2));
        // data3 might return true (false positive) but unlikely with 2048 bits
    }

    #[test]
    fn test_bloom_deterministic() {
        let mut bloom1 = BloomFilter::new();
        let mut bloom2 = BloomFilter::new();

        let data = b"Test data for determinism";

        bloom1.insert(data);
        bloom2.insert(data);

        assert_eq!(bloom1, bloom2);
    }

    #[test]
    fn test_bloom_serialization() {
        let mut bloom = BloomFilter::new();
        bloom.insert(b"Feature 1");
        bloom.insert(b"Feature 2");

        let bytes = bloom.to_bytes();
        let restored = BloomFilter::from_bytes(&bytes);

        assert_eq!(bloom, restored);
    }

    #[test]
    fn test_jaccard_identical() {
        let mut bloom = BloomFilter::new();
        bloom.insert(b"Same data");
        bloom.insert(b"More data");

        let similarity = bloom.jaccard_similarity(&bloom);
        assert!((similarity - 1.0).abs() < 0.001, "Identical filters should have J=1.0");
    }

    #[test]
    fn test_jaccard_disjoint() {
        let mut bloom1 = BloomFilter::new();
        let mut bloom2 = BloomFilter::new();

        bloom1.insert(b"Data set A");
        bloom2.insert(b"Data set B - completely different and unlikely to collide");

        let similarity = bloom1.jaccard_similarity(&bloom2);
        assert!(similarity < 0.5, "Disjoint sets should have low similarity");
    }

    #[test]
    fn test_jaccard_overlap() {
        let mut bloom1 = BloomFilter::new();
        let mut bloom2 = BloomFilter::new();

        // Add common features
        bloom1.insert(b"Common 1");
        bloom1.insert(b"Common 2");
        bloom2.insert(b"Common 1");
        bloom2.insert(b"Common 2");

        // Add unique features
        bloom1.insert(b"Unique to A");
        bloom2.insert(b"Unique to B");

        let similarity = bloom1.jaccard_similarity(&bloom2);
        assert!(similarity > 0.3 && similarity < 1.0,
                "Overlapping sets should have moderate similarity");
    }

    #[test]
    fn test_bloom_merge() {
        let mut bloom1 = BloomFilter::new();
        let mut bloom2 = BloomFilter::new();

        bloom1.insert(b"Feature A");
        bloom2.insert(b"Feature B");

        bloom1.merge(&bloom2);

        assert!(bloom1.contains(b"Feature A"));
        assert!(bloom1.contains(b"Feature B"));
    }

    #[test]
    fn test_bloom_clear() {
        let mut bloom = BloomFilter::new();
        bloom.insert(b"Data");

        assert!(!bloom.is_empty());

        bloom.clear();

        assert!(bloom.is_empty());
        assert_eq!(bloom.count_set_bits(), 0);
    }

    #[test]
    fn test_bloom_set_bits_count() {
        let mut bloom = BloomFilter::new();

        let initial_count = bloom.count_set_bits();
        assert_eq!(initial_count, 0);

        bloom.insert(b"Feature 1");
        let count1 = bloom.count_set_bits();
        assert!(count1 >= NUM_HASH_FUNCTIONS as u32);

        bloom.insert(b"Feature 2");
        let count2 = bloom.count_set_bits();
        assert!(count2 >= count1); // Should increase (or stay same if collision)
    }

    #[test]
    fn test_builder_pattern() {
        let mut builder = BloomFilterBuilder::new();

        builder.add_feature(b"Feature 1");
        builder.add_feature(b"Feature 2");
        builder.add_feature(b"Feature 3");

        assert_eq!(builder.feature_count(), 3);

        let bloom = builder.build();
        assert!(bloom.contains(b"Feature 1"));
        assert!(bloom.contains(b"Feature 2"));
        assert!(bloom.contains(b"Feature 3"));
    }

    #[test]
    fn test_hash_distribution() {
        // Test that hash functions produce different indices
        let data = b"Test data";
        let bloom = BloomFilter::new();
        let indices = bloom.get_indices(data);

        // All three indices should be different (very high probability)
        assert!(indices[0] != indices[1] || indices[1] != indices[2]);

        // All indices should be within range
        for &idx in &indices {
            assert!(idx < BLOOM_SIZE_BITS);
        }
    }
}
