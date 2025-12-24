//! Basic usage example for LavinHash
//!
//! This example demonstrates how to:
//! - Generate fuzzy hashes from data
//! - Compare hashes for similarity
//! - Configure hash parameters

use lavinhash::{generate_hash, compare_hashes, HashConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("LavinHash Basic Usage Example\n");

    // Example 1: Hash two similar texts
    println!("=== Example 1: Similar Texts ===");
    let text1 = b"The quick brown fox jumps over the lazy dog";
    let text2 = b"The quick brown fox leaps over the lazy dog";

    let config = HashConfig::default();

    let hash1 = generate_hash(text1, &config)?;
    let hash2 = generate_hash(text2, &config)?;

    let similarity = compare_hashes(&hash1, &hash2, 0.3);
    println!("Text 1: {:?}", String::from_utf8_lossy(text1));
    println!("Text 2: {:?}", String::from_utf8_lossy(text2));
    println!("Similarity: {}%\n", similarity);

    // Example 2: Hash identical data
    println!("=== Example 2: Identical Data ===");
    let data = b"This is test data for fuzzy hashing";

    let hash_a = generate_hash(data, &config)?;
    let hash_b = generate_hash(data, &config)?;

    let similarity = compare_hashes(&hash_a, &hash_b, 0.3);
    println!("Data: {:?}", String::from_utf8_lossy(data));
    println!("Similarity (should be 100%): {}%\n", similarity);

    // Example 3: Hash completely different data
    println!("=== Example 3: Different Data ===");
    let data1 = b"Hello, World!";
    let data2 = b"ZZZZZZZZZZZZZ";

    let hash1 = generate_hash(data1, &config)?;
    let hash2 = generate_hash(data2, &config)?;

    let similarity = compare_hashes(&hash1, &hash2, 0.3);
    println!("Data 1: {:?}", String::from_utf8_lossy(data1));
    println!("Data 2: {:?}", String::from_utf8_lossy(data2));
    println!("Similarity (should be low): {}%\n", similarity);

    // Example 4: Custom configuration
    println!("=== Example 4: Custom Configuration ===");
    let mut custom_config = HashConfig::default();
    custom_config.alpha = 0.5;  // 50% structure, 50% content
    custom_config.enable_parallel = false;  // Disable parallel processing
    custom_config.min_modulus = 32;  // Lower modulus for more features

    let hash = generate_hash(b"Custom config test", &custom_config)?;
    println!("Generated hash with custom config");
    println!("Fingerprint size: {} bytes", hash.size());

    // Example 5: Serialize and deserialize
    println!("\n=== Example 5: Serialization ===");
    let data = b"Serialization test data";
    let hash = generate_hash(data, &config)?;

    let serialized = hash.to_bytes();
    println!("Original size: {} bytes", serialized.len());

    use lavinhash::model::FuzzyFingerprint;
    let deserialized = FuzzyFingerprint::from_bytes(&serialized)?;
    println!("Deserialized successfully");

    // Verify they're identical
    let similarity = compare_hashes(&hash, &deserialized, 0.3);
    println!("Similarity after deserialization: {}%", similarity);

    Ok(())
}
