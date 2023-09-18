use super::is_pandigital;

#[test]
fn test_is_pandigital() {
    let cases = [
        (0, false),
        (1, true),
        (2143, true),
        (2243, false),
        (21, true),
        (12, true),
        (13, false),
        (31, false),
        (401, false),
        (102, false),
        (999994321, false)
    ];

    cases.iter().for_each(|(n, expected)| {
        println!("n:{n}");
        assert_eq!(is_pandigital(n), *expected)
    });
}
