// I blogged about this here:
// https://nicholassterling.wordpress.com/2018/08/31/o1-sum_of_multiples-in-rust/

#![feature(slice_patterns)]

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//    naive_sum_of_multiples(limit, factors)
    let factors64: Vec<_> = factors.iter().map(|&x| x as u64).collect();
    fast_sum_of_multiples((limit-1) as u64, factors64.as_ref()) as u32
}

pub fn fast_sum_of_multiples(limit: u64, factors: &[u64]) -> u64 {
    fast_sum_of_multiples1(limit, factors)
//    fast_sum_of_multiples2(limit, factors)
}

// The naive version iterates through all of the numbers,
// filtering out any that are divisible by one of the factors.
// Obviously this is going to be slow if the limit is large.
pub fn naive_sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|&i|
        factors.iter().any(|&j| i % j == 0 )
    ).sum()
}

// Sum of multiples of factors up to AND INCLUDING limit,
// done extremely quickly using a little math and recursion.
// This version's execution time does not depend on the limit.
pub fn fast_sum_of_multiples1(limit: u64, factors: &[u64]) -> u64 {
    fn lcm(a: u64, b: u64) -> u64 { a*b / gcd(a, b) }
    fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a%b) } }
    fn sum_from_ix(i: usize, limit: u64, factors: &[u64]) -> u64 {
        if i == factors.len() {  // we've processed all factors
            0
        } else {
            let factor = factors[i];
            let n = limit / factor;  // # of multiples of factor within limit
            let sum_of_multiples_of_factor = factor * (n * (n+1) / 2);
            let new_factors: Vec<_> = factors[..i].iter()
                .map(|&prev_factor| lcm(prev_factor, factor))
                .filter(|&factor| factor <= limit)
                .collect();
            let sum_of_previously_seen_multiples_of_factor =
                sum_from_ix(0, limit, &new_factors[..]);
            let sum_of_multiples_of_rest_of_factors =
                sum_from_ix(i+1, limit, factors);
            sum_of_multiples_of_factor
                - sum_of_previously_seen_multiples_of_factor
                + sum_of_multiples_of_rest_of_factors
        }
    };
    sum_from_ix(0, limit, factors)
}

// Sum of multiples of factors up to AND INCLUDING limit,
// done extremely quickly using a little math and recursion.
// This version's execution time does not depend on the limit.
// This differs from the previous version in that we pass the
// previously used factors as a separate Vec.
pub fn fast_sum_of_multiples2(limit: u64, factors: &[u64]) -> u64 {
    fn lcm(a: u64, b: u64) -> u64 { a*b / gcd(a, b) }
    fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a%b) } }
    fn sum_mults(limit: u64, factors: &[u64], prev_factors: &mut Vec<u64>) -> u64 {
        match factors {
            [] => 0,
            [factor, rest..] => {  // hmmm, why can't I say &factor here?
                let factor = *factor;  // have to do this here for now
                let n = limit / factor;  // # of multiples of factor within limit
                let sum_of_multiples_of_factor = factor * (n * (n+1) / 2);
                let new_factors: Vec<_> = prev_factors.iter()
                    .map(|&x| lcm(x, factor))
                    .filter(|&x| x <= limit)
                    .collect();
                let sum_of_previously_seen_multiples_of_factor =
                    sum_mults(limit, &new_factors[..], &mut Vec::new());
                prev_factors.push(factor);
                let sum_of_multiples_of_rest_of_factors =
                    sum_mults(limit, rest, prev_factors);
                sum_of_multiples_of_factor
                    - sum_of_previously_seen_multiples_of_factor
                    + sum_of_multiples_of_rest_of_factors
            }
        }
    };
    sum_mults(limit, factors, &mut Vec::new())
}


// Ignore this -- it's just to remind me of stuff I learned.

//        println!("limit: {}, factors: {:?}, prev: {:?}", limit, factors, prev_factors);
//
//                println!("limit: {}, factors: {:?}, SMF: {}, SPSMF: {}, SMRF: {}",
//                         limit, factors, sum_of_multiples_of_factor, sum_of_previously_seen_multiples_of_factor, sum_of_multiples_of_rest_of_factors);
//
// println!("i: {}, limit: {}, factors: {:?}, SMF: {}, SMFSP: {}, SMRF: {}", i, limit, factors, sum_of_multiples_of_factor, sum_of_previously_seen_multiples_of_factor, sum_of_multiples_of_rest_of_factors);
//
// let mut factors = factors.to_vec();
// let slice = &factors[..];
// let mut sorted_factors = factors.clone();
// sorted_factors.sort_unstable();
//
// I wanted to use a closure so I could close over some values,
// but Rust can't do recursive closures (nicely).  This doesn't compile:
/*
pub fn fast_sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    fn lcm(a: u32, b: u32) -> u32 { a*b / gcd(a, b) }
    fn gcd(a: u32, b: u32) -> u32 { if b == 0 { a } else { gcd(b, a%b) } }
    let end = factors.len();
    let sum_from_ix = |i: usize| {
        if i == end {
            0
        } else {
            let factor = factors[i];
            let n = limit / factor;  // # of multiples of factor within limit
            let sum_of_multiples_of_factor = factor * (n * (n+1) / 2);
            let new_factors = factors[..i].iter()  // factors from previous iterations
                .filter_map(|&prev_factor| {
                    let new_factor = prev_factor * factor;
                    if new_factor <= limit { Some(new_factor) } else { None }
                }).collect();
            let sum_of_previously_seen_multiples_of_factor =
                fast_sum_of_multiples(limit, new_factors);
            let sum_of_multiples_of_rest_of_factors = sum_from_ix(i+1);
            sum_of_multiples_of_factor
                - sum_of_previously_seen_multiples_of_factor
                + sum_of_multiples_of_rest_of_factors
        }
    };
    sum_from_ix(0)
}
*/
