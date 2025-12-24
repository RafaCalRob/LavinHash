//! BuzHash - Context-Triggered Piecewise Hashing (CTPH)
//!
//! Implements a rolling hash algorithm for detecting content features.
//! More robust than Rabin-Karp for uniform distribution.

/// Pre-computed random lookup table for BuzHash
/// These values ensure uniform distribution across the hash space
const BUZHASH_TABLE: [u64; 256] = [
    0x9ae16a3b2f90404f, 0xc3a5c85c97cb3127, 0xb492b66fbe98f273, 0x9ae5d64c63989f0a,
    0xb0c8ac50f0403d1a, 0xaa0ab0867b8ecdef, 0x9dfedf456fc7ed6b, 0xc37b45b3f8cfc5e5,
    0x9c0e3c5ab49e4c2e, 0xb3cf8a7d7e5f4bb0, 0xa9e167b8abac98d1, 0x8b65a7dfd7abcf0e,
    0xf9b32e4ccaef8a91, 0xeb6bab16de5d4e9a, 0xcb5af2ae96e5c0d9, 0xd42b6c875f4e0fa3,
    0x8e31f3a5c729e60f, 0xf7cde9b0f45a4c8e, 0xa48e4b27dc5cf63a, 0xbfcad73ea652e1b9,
    0xdab7f0c3e69f5c1d, 0xe8a2c54796bf3e0d, 0x9f5e2b1dca847f6e, 0xc62d5f3ba4e09812,
    0xb91ef8d03c5a7264, 0xa0fd3e87b5c14926, 0xf3e8b6ca4d219057, 0xd15a9f4e2bc8703a,
    0x8db3e7205fa6c914, 0xec9f4b2d05a7c863, 0xc8d6a50e3f17b294, 0xb25d7f91e8c0a43f,
    0x96ab0f3e4dc25781, 0xfb42ed7c96a0185e, 0xd7819e05c2baf346, 0xa34cf21e8bd05967,
    0x8f5eac0d73b91e42, 0xe6bf35c8410a9f7d, 0xcf0b9e724da3681f, 0xba981f3d65e0c2a4,
    0x914c2d6ef3a07b85, 0xfe7db80a35c49162, 0xdb9ca4f2160e7583, 0xa7e05d834cbf9216,
    0x8a2f76be109c4d3a, 0xe19c4d05a8b7f362, 0xc546af2e9d087b1c, 0xb0de7f5a1c3e8946,
    0x9c8f1a3e46b20d75, 0xf58b2cd0e9a73641, 0xd31ef849bc706a52, 0xab0c5e76f8d1a923,
    0x87fd3c9a204e6b15, 0xef6b9a12c58d0f47, 0xc9a7f05e37b14d82, 0xb63ed1a90c8f2574,
    0x98d2a4b5f7c0e136, 0xf0cd87e14a3b6920, 0xd6a4f5c2390e1b7d, 0xae51bc307f9da684,
    0x8432fa6e0c15b97a, 0xeca91fd4758b2036, 0xc2f5d70b1e49a8cf, 0xb89e6a4cd5027f31,
    0x951da8e372bf06c4, 0xfc14a3db809e6527, 0xda2fc8560b3e7149, 0xb1a7e9c5f8402d63,
    0x8678b5fd9e0a31c2, 0xe8f6a4c2017d5b39, 0xc73d518e4fa209b6, 0xbf092da85e61c347,
    0x9a620f5c3db4e871, 0xf764ed91cb5a2308, 0xdd87b02f69c1a3e4, 0xb509a7fde8c3f162,
    0x88c0d439f10e2765, 0xe542b1a9cd8f7630, 0xcb84d6201f53a9be, 0xc0785f3d9e1a42b6,
    0x9eb4c70ad826f153, 0xf2a9d8e56c0f3471, 0xd0f68e2ca4b7519d, 0xb86d1a9c0f32e574,
    0x8b15e9f72c4da063, 0xe28dfc4b39a1e650, 0xced9a05e746bf312, 0xc3e2801da5f4c679,
    0xa30fde58c7b1624e, 0xeef794b20d3a5c81, 0xd45bc9e7368f210a, 0xbbd60fe8c741a532,
    0x8d68a24705ebf391, 0xdfe5a10834cb9267, 0xd2317ba0f5ce4698, 0xc7591e8d20a4fb35,
    0xa76cf92b38d5e104, 0xeb491d6fc8a02573, 0xd7c3a9012e4f6b85, 0xbf45e28cd1a30796,
    0x8fc176da3e50b142, 0xdd318bc547b2a906, 0xd58efa0921cb3d47, 0xcad90bc8f6e25173,
    0xabd2f0c4e379a618, 0xe79acf26b15da840, 0xdb2f61ea8dc503a9, 0xc2b58fd39e0a7164,
    0x9220ba9cd6f4e852, 0xe0c7d51fab9e2603, 0xd8a7c5eb046f2319, 0xce5df03a8c1b6745,
    0xb04ea7cfde923861, 0xe3fb5d8a70c1e492, 0xde81f7bc3ea04d65, 0xc6e08fb29d3a5127,
    0x96837c4fea105d23, 0xe41e9b07dc85f396, 0xe1b8c59adf0e7243, 0xd32176de0a9bc815,
    0xb3e49f206cd15a74, 0xe72a4d91056b3c82, 0xe2f098cf7da1e546, 0xd57feb5c9e1a4073,
    0xb7c9f65ed4a30821, 0xea5f8d2e3c097164, 0xe6304a7bfc0d9258, 0xd8e02bf91a3c5647,
    0xbbb7e093246da158, 0xed9bfca2078e1d43, 0xe9789ec5ba0f4132, 0xdc487f9d3e0a5b16,
    0xbfae94c5e61f2d07, 0xf0e0a97d3b15c682, 0xecc948f2076ad351, 0xe0c19b2d5a3f7648,
    0xc3ae05fb294e8d17, 0xf42d7a1ec50b9836, 0xf0239ed8c4051a67, 0xe54cd91f72a30b85,
    0xc7b69a4e0f13d526, 0xf7834c2d06e9a715, 0xf38701e5bc4a2d93, 0xe9e13a7f0c65b124,
    0xcbc847f1d620e935, 0xfae21bc9380d7542, 0xf6f3a8db074e1c29, 0xee7fbce12840d563,
    0xd0e41f958cb3a274, 0xfe49c07d15ab2e36, 0xfa69d420cb3f8157, 0xf327481e9d05a6b2,
    0xd509abf7e0c1432e, 0x81f2bc905da34761, 0xfe089c7d0ab41e53, 0xf7df93e60c5ab142,
    0xd93a704ebf1c2685, 0x8547fac1e0bd3297, 0x8193de542fca7068, 0xfc5eb2a9c0d31784,
    0xdd751fbc904e6a23, 0x88a0f394c7be1d56, 0x8506ea7b340fd921, 0x80d4bfe25a9c3167,
    0xe1b84a702f6d9c35, 0x8c06e21b5a3df948, 0x887dbc3f9e051c62, 0x843fa521d6b0e795,
    0xe605e2df1b4ca983, 0x8f7ad5e0c19b3624, 0x8c01fa9e2b4d0637, 0x87b3049df5e6c1a8,
    0xea5c87fdb0293164, 0x92fbd341e6a05c87, 0x8f94a6e01bc7d352, 0x8b2dc75fae031649,
    0xeec40fa5d16b8327, 0x968be52d0f3ca714, 0x9335e0a7d4b16f82, 0x8eb19a2cf7d0e435,
    0xf335abd820e94c16, 0x9a295fb7e3c10684, 0x96e5a109cb8d2e37, 0x9240ef71c5a3b628,
    0xf7b1adc6f09e3157, 0x9dd0f854c2e7b136, 0x9aa3e9cf02d5b174, 0x95da6f7e8bc0a329,
    0xfc37e2a1b504d968, 0xa182a76df03c5e94, 0x9e6bda057ecf1432, 0x997e20fc4da5b637,
    0x00c85f69ab2d3e14, 0xa53ed7c910f2ab68, 0xa2424e8df03cb169, 0x9d2c0ab7f51e6d83,
    0x0563ca92df1e047b, 0xa904f1b5e26c37ad, 0xa622f8390aed7b15, 0xa0e4abfc671d2e09,
    0x0a09f61e5cb37284, 0xaccf25bdea074c39, 0xaa0deb6fc95af127, 0xa4a766ed2fbc0138,
    0x0ec14dab709e8615, 0xb0a37c890e5b1d64, 0xae03a1f2564ec987, 0xa87460bc0f9d3a12,
    0x1384c0e592ab47f6, 0xb481f63a7c0d2e95, 0xb2037cf1ae4d8b26, 0xac4c91a6d3e07154,
    0x1853a74c0eb29d37, 0xb86d9b20f4ae1c68, 0xb60d92c7f6a04315, 0xb02fed3ae51cb947,
    0x1d27be3a8f6c5419, 0xbc5a4e7d0361f285, 0xb96d0a5f841e2c37, 0xb4e31d8c9fa05726,
    0x21fc9e68a0d73b5c, 0xc047e1b6f2da8395, 0xbce5d27a6f0b4183, 0xb8619ca4e73d0f52,
    0x26e4a057c31bf729, 0xc43b2ef5106a94b7, 0xc06eb8d9574fc021, 0xbbe70fa3c81d52e4,
    0x2bd3c816df52e8a3, 0xc82f9d4e3c5ba106, 0xc40f7a1b2ed36c95, 0xbf7ce0d2b1a59483,
    0x30c7f6e5fa094d21, 0xcc248a70689fc374, 0xc7b05e3cf6142a87, 0xc314ba0f8dc76351,
    0x35bde2a401c75b96, 0xd01a6f9b5e8d3142, 0xcb593ad7c0f5e169, 0xc6afcd41692b7f08,
    0x3ab4d0f3180eac67, 0xd412548c74b1e095, 0xcf0a21eb8a3d6f57, 0xca5de06f4b1c9a73,
    0x3facbe524e2d9f81, 0xd80e3d7fa9c64b12, 0xd2c3fc1a5b6e8094, 0xce13f29d2ea0bc45,
    0x44a39c71637be245, 0xdc0a28b4c5d7e916, 0xd68ecf5a329f07b4, 0xd1c3104e0fb8d627,
    0x499ebda0789c1653, 0xe00f1bc3da6ef827, 0xda52b0894e17c3a5, 0xd5754e8bc2fa1d09,
];

/// BuzHash window size (must be power of 2 for efficiency)
const WINDOW_SIZE: usize = 64;

/// BuzHash rolling hash state
pub struct BuzHash {
    hash: u64,
    window: [u8; WINDOW_SIZE],
    position: usize,
}

impl BuzHash {
    /// Create a new BuzHash instance
    #[inline]
    pub fn new() -> Self {
        Self {
            hash: 0,
            window: [0; WINDOW_SIZE],
            position: 0,
        }
    }

    /// Reset the hash state
    #[inline]
    pub fn reset(&mut self) {
        self.hash = 0;
        self.window = [0; WINDOW_SIZE];
        self.position = 0;
    }

    /// Rotate left operation (crucial for BuzHash)
    #[inline(always)]
    fn rol(value: u64, shift: u32) -> u64 {
        value.rotate_left(shift)
    }

    /// Update hash with a new byte (rolling window)
    #[inline]
    pub fn update(&mut self, byte_in: u8) -> u64 {
        let byte_out = self.window[self.position];
        self.window[self.position] = byte_in;
        self.position = (self.position + 1) & (WINDOW_SIZE - 1); // Fast modulo for power of 2

        // Core BuzHash formula: R_next = rol(R_prev, 1) ⊕ RTL[byte_out] ⊕ RTL[byte_in]
        self.hash = Self::rol(self.hash, 1)
                    ^ Self::rol(BUZHASH_TABLE[byte_out as usize], WINDOW_SIZE as u32)
                    ^ BUZHASH_TABLE[byte_in as usize];

        self.hash
    }

    /// Get current hash value
    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// Check if current hash triggers a feature point
    /// M is the modulus, dynamically calculated based on file size
    #[inline]
    pub fn is_trigger(&self, modulus: u64) -> bool {
        self.hash % modulus == 0
    }
}

/// Calculate dynamic modulus based on file size
/// Smaller files use smaller M (more features), larger files use larger M (fewer features)
#[inline]
pub fn calculate_modulus(file_size: usize) -> u64 {
    if file_size == 0 {
        return 64; // Minimum modulus
    }

    // M = 2^(log2(file_size) / 2 + 6)
    // This ensures roughly sqrt(file_size) features are selected
    let log_size = (file_size as f64).log2();
    let exponent = (log_size / 2.0 + 6.0).min(16.0); // Cap at 2^16 to avoid too few features
    (1u64 << exponent as u32).max(64)
}

impl Default for BuzHash {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buzhash_deterministic() {
        let mut hash1 = BuzHash::new();
        let mut hash2 = BuzHash::new();

        let data = b"Hello, World! This is a test.";

        for &byte in data {
            hash1.update(byte);
            hash2.update(byte);
        }

        assert_eq!(hash1.hash(), hash2.hash(), "Hash should be deterministic");
    }

    #[test]
    fn test_buzhash_rolling() {
        let mut hash = BuzHash::new();

        // Fill window
        for i in 0..WINDOW_SIZE {
            hash.update(i as u8);
        }

        let h1 = hash.hash();

        // Add one more byte - should evict first byte
        hash.update(255);
        let h2 = hash.hash();

        assert_ne!(h1, h2, "Hash should change when window rolls");
    }

    #[test]
    fn test_modulus_calculation() {
        assert_eq!(calculate_modulus(0), 64);
        assert_eq!(calculate_modulus(1024), 2048); // 2^(log2(1024)/2 + 6) = 2^11
        assert!(calculate_modulus(1_000_000) > calculate_modulus(1000));
    }

    #[test]
    fn test_avalanche_effect() {
        let mut hash1 = BuzHash::new();
        let mut hash2 = BuzHash::new();

        let data1 = b"Hello World";
        let data2 = b"Hello Xorld"; // Single bit flip

        for &byte in data1 {
            hash1.update(byte);
        }

        for &byte in data2 {
            hash2.update(byte);
        }

        let h1 = hash1.hash();
        let h2 = hash2.hash();

        // Hashes should be very different despite small input change
        let diff_bits = (h1 ^ h2).count_ones();
        assert!(diff_bits > 10, "Avalanche effect insufficient: {} bits different", diff_bits);
    }

    #[test]
    fn test_trigger_detection() {
        let mut hash = BuzHash::new();
        let modulus = 64; // Lower modulus for more frequent triggers

        // Use larger data set to ensure triggers
        let data: Vec<u8> = (0..500).map(|i| (i % 256) as u8).collect();
        let mut trigger_count = 0;

        for &byte in &data {
            hash.update(byte);
            if hash.is_trigger(modulus) {
                trigger_count += 1;
            }
        }

        // Should have at least some triggers in this data
        assert!(trigger_count > 0, "Should have detected at least one trigger");
    }
}
