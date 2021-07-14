# hamming-lsh

Generates and utilizes deterministic dictionaries to generate locality-sensitive hashes (LHS) for arbitrary hamming space features

This allows you to generate relatively balanced locality-sensitive hashes (LSH) hashes for bags of features. This also allows you to reduce the dimensionality of a single hamming feature into a smaller hamming space, but it is slightly imbalanced when doing so, especially for small input hamming spaces (number of input bits).

## How it works

This works by using `hamming-dict` to create codewords in the hamming space that are as maximally spaced out as possible.

When hashing an input key, the key is compared to all of the codewords. For each codeword that it is less than or equal to half the input key hamming space bits, a bit is set to `1` in the output hash, and otherwise `0`. This is done because half of the bits of the input hamming space encapsulates roughly half of the space, though it is not perfect and will encapsulate slightly more than half of the space. Due to this imperfection, it does not generate a perfectly balanced hash. However, since it is fairly close to the median, the output hash is useful, and its dimensionality is reduced.

When hashing a bag (set of features), it performs the same procedure as above for every feature. It then counts the number of features which were or were not in the threshold radius of half the input key hamming space bits. It then applies a corrective factor. The corrective factor corrects for the probability of each bit being `1` more often than `0` by subtracting from the count the number of `1` encounters. The reason this is done is because, in absence of this correction, the more items that are added to the final hash, the hash value would be less discriminating as all of the bits would tend towards `1`. By performing this correction, we actually increase the amount of retained information from the features towards the maximum amount (`1` bit per bit of output) as the number of features in the bag increases.
