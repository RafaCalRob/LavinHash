//! WebAssembly bindings for LavinHash

use wasm_bindgen::prelude::*;
use crate::{generate_hash, compare_hashes, HashConfig};

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook();
}

/// Generate a fuzzy hash from data (WASM wrapper)
///
/// # Arguments
/// * `data` - Input data as Uint8Array
///
/// # Returns
/// Serialized fingerprint as Uint8Array
#[wasm_bindgen]
pub fn wasm_generate_hash(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    let config = HashConfig::default();

    let fingerprint = generate_hash(data, &config)
        .map_err(|e| JsValue::from_str(&format!("Error generating hash: {:?}", e)))?;

    Ok(fingerprint.to_bytes())
}

/// Compare two fuzzy hashes (WASM wrapper)
///
/// # Arguments
/// * `hash_a` - First fingerprint (serialized)
/// * `hash_b` - Second fingerprint (serialized)
///
/// # Returns
/// Similarity score 0-100
#[wasm_bindgen]
pub fn wasm_compare_hashes(hash_a: &[u8], hash_b: &[u8]) -> Result<u8, JsValue> {
    use crate::model::FuzzyFingerprint;

    let fp_a = FuzzyFingerprint::from_bytes(hash_a)
        .map_err(|e| JsValue::from_str(&format!("Error parsing hash A: {:?}", e)))?;

    let fp_b = FuzzyFingerprint::from_bytes(hash_b)
        .map_err(|e| JsValue::from_str(&format!("Error parsing hash B: {:?}", e)))?;

    Ok(compare_hashes(&fp_a, &fp_b, 0.3))
}

/// Generate hash and compare in one step (WASM wrapper)
///
/// # Arguments
/// * `data_a` - First data as Uint8Array
/// * `data_b` - Second data as Uint8Array
///
/// # Returns
/// Similarity score 0-100
#[wasm_bindgen]
pub fn wasm_compare_data(data_a: &[u8], data_b: &[u8]) -> Result<u8, JsValue> {
    let config = HashConfig::default();

    let hash_a = generate_hash(data_a, &config)
        .map_err(|e| JsValue::from_str(&format!("Error generating hash A: {:?}", e)))?;

    let hash_b = generate_hash(data_b, &config)
        .map_err(|e| JsValue::from_str(&format!("Error generating hash B: {:?}", e)))?;

    Ok(compare_hashes(&hash_a, &hash_b, 0.3))
}

/// Get fingerprint size in bytes (WASM wrapper)
#[wasm_bindgen]
pub fn wasm_fingerprint_size(hash: &[u8]) -> Result<usize, JsValue> {
    use crate::model::FuzzyFingerprint;

    let fp = FuzzyFingerprint::from_bytes(hash)
        .map_err(|e| JsValue::from_str(&format!("Error parsing hash: {:?}", e)))?;

    Ok(fp.size())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_generate_hash() {
        let data = b"Hello, WASM!";
        let result = wasm_generate_hash(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_wasm_compare_data() {
        let data1 = b"The quick brown fox";
        let data2 = b"The quick brown fox";

        let result = wasm_compare_data(data1, data2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
    }
}
