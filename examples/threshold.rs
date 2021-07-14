use hamming_bow::HammingDictionary;

fn test_with_hash_size<const H: usize>() {
    let dict = HammingDictionary::<64, H>::new();
    eprintln!("dict bits {} threshold {}", H * 8, dict.threshold());
}

fn main() {
    test_with_hash_size::<4>();
    test_with_hash_size::<5>();
    test_with_hash_size::<6>();
    test_with_hash_size::<7>();
    test_with_hash_size::<8>();
    test_with_hash_size::<16>();
    test_with_hash_size::<32>();
}
