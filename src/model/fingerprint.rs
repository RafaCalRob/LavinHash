//! FuzzyFingerprint - Core data structure for LavinHash
//!
//! Represents a dual-layer fuzzy hash with structural and content components.

use crate::algo::{BloomFilter, BLOOM_SIZE_BYTES};
use std::fmt;

/// Magic byte for LavinHash format ('H')
const MAGIC_BYTE: u8 = 0x48;

/// Current version of the fingerprint format
const VERSION: u8 = 0x01;

/// Minimum size for a valid fingerprint
const MIN_FINGERPRINT_SIZE: usize = 4 + BLOOM_SIZE_BYTES; // Header + Bloom

/// FuzzyFingerprint - The core fingerprint structure
///
/// Binary format:
/// - Offset 0x00: Magic (0x48 = 'H')
/// - Offset 0x01: Version (0x01)
/// - Offset 0x02-0x03: Struct Length (u16 LE)
/// - Offset 0x04-0x403: Content Bloom Filter (1024 bytes)
/// - Offset 0x404+: Structure Data (variable length)
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct FuzzyFingerprint {
    /// Version of the fingerprint format
    pub version: u8,

    /// Flags for future use (currently unused)
    pub flags: u8,

    /// Length of the structural data in bytes
    pub struct_len: u16,

    /// Content Bloom Filter (8192 bits = 1024 bytes)
    pub content_hash: [u64; 128],

    /// Structural data (entropy nibbles packed)
    pub struct_data: Vec<u8>,
}

impl FuzzyFingerprint {
    /// Create a new fingerprint
    pub fn new(content_bloom: BloomFilter, structural_data: Vec<u8>) -> Self {
        let struct_len = structural_data.len() as u16;

        let bloom_bytes = content_bloom.to_bytes();
        let mut content_hash = [0u64; 128];
        for i in 0..128 {
            let offset = i * 8;
            content_hash[i] = u64::from_le_bytes([
                bloom_bytes[offset],
                bloom_bytes[offset + 1],
                bloom_bytes[offset + 2],
                bloom_bytes[offset + 3],
                bloom_bytes[offset + 4],
                bloom_bytes[offset + 5],
                bloom_bytes[offset + 6],
                bloom_bytes[offset + 7],
            ]);
        }

        Self {
            version: VERSION,
            flags: 0,
            struct_len,
            content_hash,
            struct_data: structural_data,
        }
    }

    /// Serialize the fingerprint to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let total_size = 4 + BLOOM_SIZE_BYTES + self.struct_data.len();
        let mut bytes = Vec::with_capacity(total_size);

        // Header
        bytes.push(MAGIC_BYTE);
        bytes.push(self.version);
        bytes.extend_from_slice(&self.struct_len.to_le_bytes());

        // Content Bloom Filter (16KB)
        for &word in &self.content_hash {
            bytes.extend_from_slice(&word.to_le_bytes());
        }

        // Structural data
        bytes.extend_from_slice(&self.struct_data);

        bytes
    }

    /// Deserialize fingerprint from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FingerprintError> {
        if bytes.len() < MIN_FINGERPRINT_SIZE {
            return Err(FingerprintError::InvalidSize);
        }

        // Validate magic byte
        if bytes[0] != MAGIC_BYTE {
            return Err(FingerprintError::InvalidMagic);
        }

        let version = bytes[1];
        if version != VERSION {
            return Err(FingerprintError::UnsupportedVersion(version));
        }

        let struct_len = u16::from_le_bytes([bytes[2], bytes[3]]);

        // Extract content hash
        let mut content_hash = [0u64; 128];
        for i in 0..128 {
            let offset = 4 + i * 8;
            content_hash[i] = u64::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]);
        }

        // Extract structural data
        let struct_data_offset = 4 + BLOOM_SIZE_BYTES;
        let expected_end = struct_data_offset + struct_len as usize;

        if bytes.len() < expected_end {
            return Err(FingerprintError::InvalidSize);
        }

        let struct_data = bytes[struct_data_offset..expected_end].to_vec();

        Ok(Self {
            version,
            flags: 0,
            struct_len,
            content_hash,
            struct_data,
        })
    }

    /// Get the content Bloom filter
    pub fn content_bloom(&self) -> BloomFilter {
        let mut bytes = [0u8; BLOOM_SIZE_BYTES];
        for i in 0..128 {
            let word_bytes = self.content_hash[i].to_le_bytes();
            bytes[i * 8..i * 8 + 8].copy_from_slice(&word_bytes);
        }
        BloomFilter::from_bytes(&bytes)
    }

    /// Get the structural data
    pub fn structural_data(&self) -> &[u8] {
        &self.struct_data
    }

    /// Calculate similarity with another fingerprint
    ///
    /// Δ(A, B) = α · Levenshtein_Norm(S_A, S_B) + (1-α) · Jaccard(C_A, C_B)
    ///
    /// Returns similarity score 0-100
    pub fn similarity(&self, other: &FuzzyFingerprint, alpha: f32) -> u8 {
        // Content similarity (Jaccard on Bloom filters)
        let content_sim = self.content_bloom().jaccard_similarity(&other.content_bloom());

        // Structural similarity (Levenshtein on structural vectors)
        let struct_sim = crate::algo::structural_similarity(
            &self.struct_data,
            &other.struct_data,
        );

        // Combined similarity
        let combined = alpha * struct_sim + (1.0 - alpha) * content_sim;

        // Convert to 0-100 scale
        // Use floor() for extreme sensitivity - even 99.96% shows as 99% (not identical)
        (combined * 100.0).floor().min(100.0).max(0.0) as u8
    }

    /// Get fingerprint size in bytes
    pub fn size(&self) -> usize {
        4 + BLOOM_SIZE_BYTES + self.struct_data.len()
    }
}

impl fmt::Display for FuzzyFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LavinHash(v{}, struct_len={}, size={})",
            self.version,
            self.struct_len,
            self.size()
        )
    }
}

/// Errors that can occur during fingerprint operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FingerprintError {
    /// Invalid size for fingerprint data
    InvalidSize,

    /// Invalid magic byte
    InvalidMagic,

    /// Unsupported version
    UnsupportedVersion(u8),
}

impl fmt::Display for FingerprintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize => write!(f, "Invalid fingerprint size"),
            Self::InvalidMagic => write!(f, "Invalid magic byte"),
            Self::UnsupportedVersion(v) => write!(f, "Unsupported version: {}", v),
        }
    }
}

impl std::error::Error for FingerprintError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::BloomFilter;

    #[test]
    fn test_fingerprint_creation() {
        let mut bloom = BloomFilter::new();
        bloom.insert(b"feature1");
        bloom.insert(b"feature2");

        let structural = vec![0x12, 0x34, 0x56, 0x78];

        let fp = FuzzyFingerprint::new(bloom, structural.clone());

        assert_eq!(fp.version, VERSION);
        assert_eq!(fp.struct_len, 4);
        assert_eq!(fp.struct_data, structural);
    }

    #[test]
    fn test_fingerprint_serialization() {
        let mut bloom = BloomFilter::new();
        bloom.insert(b"test feature");

        let structural = vec![0xAB, 0xCD];
        let fp = FuzzyFingerprint::new(bloom, structural);

        let bytes = fp.to_bytes();

        // Check magic and version
        assert_eq!(bytes[0], MAGIC_BYTE);
        assert_eq!(bytes[1], VERSION);

        // Check total size
        assert_eq!(bytes.len(), 4 + BLOOM_SIZE_BYTES + 2);
    }

    #[test]
    fn test_fingerprint_deserialization() {
        let mut bloom = BloomFilter::new();
        bloom.insert(b"test data");

        let structural = vec![0x11, 0x22, 0x33];
        let fp1 = FuzzyFingerprint::new(bloom, structural);

        let bytes = fp1.to_bytes();
        let fp2 = FuzzyFingerprint::from_bytes(&bytes).unwrap();

        assert_eq!(fp1.version, fp2.version);
        assert_eq!(fp1.struct_len, fp2.struct_len);
        assert_eq!(fp1.struct_data, fp2.struct_data);
        assert_eq!(fp1.content_hash, fp2.content_hash);
    }

    #[test]
    fn test_fingerprint_invalid_magic() {
        let mut bytes = vec![0xFF, VERSION]; // Invalid magic
        bytes.extend_from_slice(&[0, 0]); // struct_len
        bytes.extend_from_slice(&[0u8; BLOOM_SIZE_BYTES]); // bloom

        let result = FuzzyFingerprint::from_bytes(&bytes);
        assert_eq!(result, Err(FingerprintError::InvalidMagic));
    }

    #[test]
    fn test_fingerprint_invalid_size() {
        let bytes = vec![MAGIC_BYTE, VERSION]; // Too small

        let result = FuzzyFingerprint::from_bytes(&bytes);
        assert_eq!(result, Err(FingerprintError::InvalidSize));
    }

    #[test]
    fn test_fingerprint_similarity_identical() {
        let mut bloom = BloomFilter::new();
        bloom.insert(b"same");

        let structural = vec![0x42];
        let fp1 = FuzzyFingerprint::new(bloom.clone(), structural.clone());
        let fp2 = FuzzyFingerprint::new(bloom, structural);

        let sim = fp1.similarity(&fp2, 0.3);
        assert_eq!(sim, 100);
    }

    #[test]
    fn test_fingerprint_similarity_different() {
        let mut bloom1 = BloomFilter::new();
        bloom1.insert(b"data A");

        let mut bloom2 = BloomFilter::new();
        bloom2.insert(b"data B very different to avoid collision");

        let fp1 = FuzzyFingerprint::new(bloom1, vec![0x11]);
        let fp2 = FuzzyFingerprint::new(bloom2, vec![0xFF]);

        let sim = fp1.similarity(&fp2, 0.3);
        assert!(sim < 100);
    }

    #[test]
    fn test_fingerprint_display() {
        let bloom = BloomFilter::new();
        let fp = FuzzyFingerprint::new(bloom, vec![0x12, 0x34]);

        let display = format!("{}", fp);
        assert!(display.contains("LavinHash"));
        assert!(display.contains("v1"));
    }
}
