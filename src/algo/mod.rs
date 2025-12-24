//! Core algorithms for fuzzy hashing

pub mod bloom;
pub mod buzhash;
pub mod entropy;

pub use bloom::{BloomFilter, BloomFilterBuilder, BLOOM_SIZE_BYTES};
pub use buzhash::{BuzHash, calculate_modulus};
pub use entropy::{calculate_entropy, generate_structural_vector, structural_similarity};
