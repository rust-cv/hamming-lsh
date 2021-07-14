#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use bitarray::BitArray;

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
        let threshold = (codewords
            .iter()
            .enumerate()
            .map(|(ix, cw)| {
                codewords
                    .iter()
                    .enumerate()
                    .filter(|&(other_ix, _)| ix != other_ix)
                    .map(|(_, other_cw)| cw.distance(other_cw) as u64)
                    .sum::<u64>()
            })
            .sum::<u64>() as f64
            / codewords.len() as f64
            / (codewords.len() - 1) as f64) as u32;
        Self {
            codewords,
            threshold,
        }
    }

    pub fn threshold(&self) -> u32 {
        self.threshold
    }
}

impl<const B: usize, const H: usize> Default for HammingDictionary<B, H> {
    fn default() -> Self {
        Self::new()
    }
}
