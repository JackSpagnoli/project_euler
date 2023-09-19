# Solution Source

## Setup

Each problem's solution is contained in modules `problemXXX`, each of which contain a function `ans` returning either a `u128` or `i128`.

Some utility modules also exist. Details below.

`lib.rs` contains inports these problem modules, along with a macro `define_problem`, taking a problem module name and an expected answer. These answers can be either `u128` or `i128`, and gets parsed into an `enum` `Whole128` depending on sign (Negatives are cast as `Signed(i128)`, positives as `Unsigned(u128)`). This macro is used in `main.rs`.

`main.rs` contains a definition of a vector `problems`, containing calls to `define_problem` for each problem to be validated. These get passed into the `check_problem` function that runs the solution, unwraps the selected answer from the `Whole128` enum, and validates output. Solutions are also timed.

**SPOILER ALERT: `main.rs` contains the solutions to each problem**

Some problems are commented out, as either I haven't updated my previous solution to this new format, or they are unacceptably slow.

## Utilities

The following modules exist to provide utility functions for problem solutions:

### `number_utils`

Contains a struct to return iterators over the digits of a number (`digits`), and a struct to return the distribution of digits in a number (`digit_dist`).

### `permutations`

Contains an iterable implementation of Heap's Algorithm to generate the permutations of numbers `1` to `n`, or the permutations of a generic vector of elements which `impl Clone`.

### `primes`

Contains a struct which can be used to validate primes (`is_prime(n)`), to get the `nth` prime (`index(n)`), or finally as an iterator to return primes sequentially.

*n.b: since the struct dynamically generates primes it is unfortunately not possible to implement Index as this needs to work with an immutable pointer, hence the use of a seperate `index` function.*