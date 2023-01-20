extern crate nth_prime as np;

#[test]
fn test_zeroth_prime() { assert_eq!(np::nth(0), None); }

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(1), Some(2));
}

#[test]
fn test_second_prime() { assert_eq!(np::nth(2), Some(3)); }

#[test]
fn test_third_prime() { assert_eq!(np::nth(3), Some(5)); }

#[test]
fn test_fourth_prime() { assert_eq!(np::nth(4), Some(7)); }

#[test]
fn test_fifth_prime() { assert_eq!(np::nth(5), Some(11)); }

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(6), Some(13));
}

#[test]
fn test_seventh_prime() { assert_eq!(np::nth(7), Some(17)); }

#[test]
fn test_eighth_prime() { assert_eq!(np::nth(8), Some(19)); }

#[test]
fn test_ninth_prime() { assert_eq!(np::nth(9), Some(23)); }

#[test]
fn test_tenth_prime() { assert_eq!(np::nth(10), Some(29)); }

#[test]
fn test_eleventh_prime() { assert_eq!(np::nth(11), Some(31)); }

//#[ignore]
#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10001), Some(104743));
}


