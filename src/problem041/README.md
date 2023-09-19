# problem041

This solution uses Heap's algorithm to generate each permutation of the digits `1..n`, then filter to primes and take the maximum. Digits are iterated over `1..9`, as the largest pandigital is `987654321`.

This is achieved by reversing `1..9` (Searching in decending order to reduce searchspace), mapping each number of digits to a Heap's algorithm permuter. These permuters are flatmapped to chain them into a single iterator, their responses converted from a vector of digits to a number, then filtered by the closure `is_prime` (used for neatness).

This solution runs in release mode in ~`0.025` seconds, over `1000x` faster than my first attempt.

This first solution iterated over all numbers `1_000_000_000` to `1`, filters to primes, then to pandigitals. This took around `30` seconds to run, on account of being so wasteful. This spurred the development of the `permutations` module.