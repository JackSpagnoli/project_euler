use super::Primes;

#[test]
fn test_default() {
    let primes = Primes::default();

    assert_eq!(primes.index, 0);
    assert_eq!(primes.primes[..], [2, 3])
}

#[test]
fn test_next_primes() {
    let expected_primes: Vec<u128> = vec![2, 3, 5, 7, 11, 13, 17, 19];

    let mut primes = Primes::default();
    expected_primes
        .iter()
        .for_each(|prime| assert_eq!(prime, &primes.next().unwrap()))
}

#[test]
fn test_reset() {
    let mut primes = Primes::default();

    primes.next();
    primes.next();
    primes.next();
    primes.next();
    primes.next();
    primes.next();

    primes = primes.reset();

    assert_eq!(primes.index, 0);
    assert_eq!(primes.primes[..], [2, 3, 5, 7, 11, 13])
}

#[test]
fn test_is_prime() {
    let cases = [
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (10, false),
        (44, false),
        (101, true),
    ];

    let mut primes = Primes::default();

    cases
        .iter()
        .for_each(|(n, expected)| assert_eq!(&primes.is_prime(*n), expected));
}

#[test]
fn test_index() {
    let mut primes = Primes::default();

    assert_eq!(primes.index(100), 547)
}
