extern crate sum_of_multiples;

use sum_of_multiples::*;

#[test]
fn multiples_one() {
    assert_eq!(0, sum_of_multiples(1, &[3, 5]))
}

#[test]
fn multiples_two() {
    assert_eq!(3, sum_of_multiples(4, &[3, 5]))
}

#[test]
fn multiples_three() {
    assert_eq!(23, sum_of_multiples(10, &[3, 5]))
}

#[test]
fn multiples_four() {
    assert_eq!(2318, sum_of_multiples(100, &[3, 5]))
}

#[test]
fn multiples_five() {
    assert_eq!(233168, sum_of_multiples(1000, &[3, 5]))
}

#[test]
fn multiples_six() {
    assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]))
}

#[test]
fn multiples_seven() {
    assert_eq!(30, sum_of_multiples(15, &[4, 6]))
}

#[test]
fn multiples_eight() {
    assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]))
}

#[test]
fn multiples_nine() {
    assert_eq!(275, sum_of_multiples(51, &[5, 25]))
}

#[test]
fn multiples_ten() {
    assert_eq!(2203160, sum_of_multiples(10000, &[43, 47]))
}

#[test]
fn multiples_eleven() { assert_eq!(4950, sum_of_multiples(100, &[1])) }

#[test]
fn multiples_twelve() { assert_eq!(0, sum_of_multiples(10000, &[])) }

#[test]
fn multiples_thirteen() { assert_eq!(2_714_264_277, sum_of_multiples(100_000, &[3,5,7])) }

#[test]
fn huge_limit() { assert_eq!(271_428_572_071_428_566, fast_sum_of_multiples(1_000_000_000, &[3,5,7]))}
