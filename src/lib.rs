#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};
use bitarray::BitArray;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// This structure allows one to generate balanced locality-sensitive hashes.
///
/// `B` is the number of bytes in the codewords.
/// `H` is the number of bytes in the hash.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct HammingHasher<const B: usize, const H: usize> {
    codewords: Vec<BitArray<B>>,
    imbalance: f64,
}

impl<const B: usize, const H: usize> HammingHasher<B, H> {
    pub fn new() -> Self {
        Self::new_with_seed(0)
    }

    pub fn new_with_seed(seed: u64) -> Self {
        assert_ne!(H, 0);
        let codewords = hamming_dict::generate_dict_seed(H * 8, seed);
        let imbalance = hamming_volume_imbalance(B * 8);

        Self {
            codewords,
            imbalance,
        }
    }

    /// There must be exactly `H * 8` codewords.
    pub fn new_with_codewords(codewords: Vec<BitArray<B>>) -> Self {
        assert_eq!(codewords.len(), H * 8);
        let imbalance = hamming_volume_imbalance(B * 8);

        Self {
            codewords,
            imbalance,
        }
    }

    const fn threshold() -> u32 {
        // The correct hamming radius to encapsulate roughly half the space is always
        // half of the bits of the hamming space. However, this always overshoots half
        // the volume by a small margin, which is corrected for.
        (B * 4) as u32
    }

    /// Convert the input feature into a hash.
    pub fn hash(&self, feature: &BitArray<B>) -> BitArray<H> {
        let mut hash = BitArray::zeros();
        for (ix, word) in self.codewords.iter().enumerate() {
            // This makes sure to alternate between less than and less than or equal to every other codeword.
            hash[ix >> 3] |= ((feature.distance(word) <= Self::threshold()) as u8) << (ix & 0b111);
        }
        hash
    }

    /// Convert the input features into a hash.
    pub fn hash_bag<'a>(&self, features: impl IntoIterator<Item = &'a BitArray<B>>) -> BitArray<H> {
        let mut counts = vec![0isize; H * 8];
        let mut num_features = 0usize;
        for feature in features {
            for (ix, word) in self.codewords.iter().enumerate() {
                // This makes sure to alternate between less than and less than or equal to every other codeword.
                counts[ix] +=
                    (((feature.distance(word) <= Self::threshold()) as isize) << 1).wrapping_sub(1);
            }
            num_features += 1;
        }
        // Correct for the natural imbalance in ones caused by the hamming spheres being slightly
        // above half the volume. The more features in the bag, the more correct (and balanced) the result will be.
        let correction = (num_features as f64 * -self.imbalance + 0.5) as isize;
        for count in &mut counts {
            *count += correction;
        }
        let mut hash = BitArray::zeros();
        for ix in 0..H * 8 {
            hash[ix >> 3] |= (counts[ix].is_positive() as u8) << (ix & 0b111);
        }
        hash
    }
}

impl<const B: usize, const H: usize> Default for HammingHasher<B, H> {
    fn default() -> Self {
        Self::new()
    }
}

fn hamming_volume_imbalance(n: usize) -> f64 {
    // This just brings 2 to the power of n - 1 without the use of std or num_traits.
    let half_space = core::iter::repeat(2.0f64)
        .take(n - 1)
        .fold(1.0, |a, b| a * b);
    let mut volume = 0.0;
    // Iterate to half of the bits (rounded up) to encapsulate roughly half the volume.
    for radius in 0..=(n + 1) / 2 {
        // Add the hamming shell at radius to the volume.
        volume += n_choose_k(n, radius);
    }
    // Volume to half_space ratio should be 1.0, but it will overshoot by some amount.
    // This amount is the imbalance.
    volume / half_space - 1.0
}

// N choose K algorithm for unsigned integers.
//
// This is used to compute hamming shell areas/volumes (the notion of volume/area breaks down in discrete spaces).
fn n_choose_k(n: usize, k: usize) -> f64 {
    if k == 0 {
        1.0
    } else {
        n_choose_k(n - 1, k - 1) * n as f64 / k as f64
    }
}
