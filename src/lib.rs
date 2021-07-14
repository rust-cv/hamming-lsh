#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use bitarray::BitArray;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

/// This structure allows one to generate balanced locality-sensitive hashes.
///
/// `B` is the number of bytes in the codewords.
/// `H` is the number of bytes in the hash.
#[derive(Debug)]
pub struct HammingDictionary<const B: usize, const H: usize> {
    codewords: Vec<BitArray<B>>,
    threshold: u32,
}

impl<const B: usize, const H: usize> HammingDictionary<B, H> {
    pub fn new() -> Self {
        assert_ne!(H, 0);
        let codewords = hamming_dict::generate_dict(H * 8);
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
        let mut distances: Vec<u32> = (0..256)
            .map(|_| {
                let mut arr = [0u8; B];
                for byte in &mut arr {
                    *byte = rng.gen();
                }
                arr
            })
            .map(BitArray::new)
            .flat_map(|cw| codewords.iter().map(move |other_cw| cw.distance(other_cw)))
            .collect();
        distances.sort_unstable();
        let threshold = distances[distances.len() / 2];
        Self {
            codewords,
            threshold,
        }
    }

    pub fn threshold(&self) -> u32 {
        self.threshold
    }

    /// Convert the input feature into a hash.
    pub fn hash(&self, feature: &BitArray<B>) -> BitArray<H> {
        let mut hash = BitArray::zeros();
        for (ix, word) in self.codewords.iter().enumerate() {
            // This makes sure to alternate between less than and less than or equal to every other codeword.
            if (feature.distance(word) < self.threshold()) ^ (ix & 1 == 1) {
                hash[ix >> 3] |= 1 << (ix & 0b111);
            }
        }
        hash
    }
}

impl<const B: usize, const H: usize> Default for HammingDictionary<B, H> {
    fn default() -> Self {
        Self::new()
    }
}
