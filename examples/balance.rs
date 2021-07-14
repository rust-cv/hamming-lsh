use bitarray::BitArray;
use hamming_bow::HammingDictionary;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() {
    let dict = HammingDictionary::<64, 32>::new();

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);

    let hashes = (0..256)
        .map(|_| {
            let mut arr = [0u8; 64];
            for byte in &mut arr {
                *byte = rng.gen();
            }
            arr
        })
        .map(BitArray::new)
        .map(|ba| dict.hash(&ba))
        .collect::<Vec<_>>();

    eprintln!("hashes: {:02X?}", hashes);
    eprintln!(
        "average hamming weight: {}",
        hashes.iter().map(|hash| hash.weight() as u64).sum::<u64>() as f64 / hashes.len() as f64
    );
}
