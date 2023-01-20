use std::collections::LinkedList;
use std::ops::{Mul, Add};
//use std::iter::Filter;

// As a learning exercise, I decided to write a more complex but
// presumably more efficient version than simply checking whether
// each number is evenly divisible by integers up to its square
// root.  In it we keep track of the primes we have found and
// the next multiple we expect to see from each, and use that
// to filter out non-primes, leaving the primes.


// A CandidateIterator very efficiently generates integers > 3
// that are not multiples of 2 or 3, i.e. 5,7,11,13,17,19,23,25,29,31,...
// This leaves only 1/3 of the integers to be tested.
// This improves the performance of the naive version by a factor of 9,
// since it nests two loops through these numbers.

struct CandidateIterator {
    last: u32,
    increment: u32
}

impl CandidateIterator {
    fn new() -> CandidateIterator {
        CandidateIterator { last: 1, increment: 4 }
    }
}

impl Iterator for CandidateIterator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.last += self.increment;
        self.increment = 6 - self.increment;
        Some(self.last)
    }
}

// WithIntSqrt provides an Iterator adapter that adds the integer
// square root to a stream of integers.  It does so without using
// the square root function.
// (n+1)^2 = n^2 + (2n + 1)
trait WithIntSqrt<I,T> where
    I: Iterator<Item = T>,
    T: Copy + Eq + Ord
     + Add<T, Output = T>
     + Mul<T, Output = T>
{
    fn with_asc_int_sqrt_from(self, sqrt: T) -> AscIntSqrtIterator<I,T>;
}

impl<I,T> WithIntSqrt<I,T> for I where
    I: Iterator<Item = T>,
    T: Copy + Eq + Ord
     + Add<T, Output = T>
     + Mul<T, Output = T>
{
    fn with_asc_int_sqrt_from(self, sqrt: T) -> AscIntSqrtIterator<I,T> {
        let n = sqrt + T::one();
        AscIntSqrtIterator {
            iter: self,
            sqrt,
            end: n*n
        }
    }
}

struct AscIntSqrtIterator<I,T> where
    I: Iterator<Item = T>,
    T: Copy + Eq + Ord
     + Add<Output = T>
     + Mul<Output = T>
{
    iter: I,
    sqrt: T,
    end: T
}

impl<I,T> Iterator for AscIntSqrtIterator<I,T> where
    I: Iterator<Item = T>,
    T: Copy + Eq + Ord
     + Add<Output = T>
     + Mul<Output = T>
{
    type Item = (T,T);
    fn next(&mut self) -> Option<(T,T)> {
        self.iter.next().map( |n|
            if n >= self.end {
                let e = self.end + self.sqrt + self.sqrt + T::one();
                if n < e {
                    self.sqrt = self.sqrt + T::one();
                    self.end = e;
                } // else TODO
            }
            (n, self.sqrt)
        )
    }
}

pub fn nth(n: u32) -> Option<u32> {
    if n > 2 {
        Some(
            nth_prime_naive(n-3)
//          nth_prime_vec(n-3)
//          nth_prime_ll(n-3)
        )
    } else if n == 0 {
        None
    } else {
        Some(n+1) // 1st = 2, 2nd = 3
    }
}

//pub fn primes_from_5() -> impl Iterator<Item = u32> {
//pub fn primes_from_5() -> Filter<CandidateIterator, fn(&u32) -> bool)> {
pub fn nth_prime_naive(n: u32) -> u32 {
    let mut sqrt = 2;
    let mut sqrt_end = 3 * 3;
    CandidateIterator::new()
        .filter( |&candidate| {
            if candidate >= sqrt_end {
                sqrt += 1;
                sqrt_end = (sqrt + 1) * (sqrt + 1);
            }
            CandidateIterator::new()
                .take_while(|&c| c <= sqrt)
                .all(|c| candidate % c != 0)
        })
        .nth(n as usize).unwrap()
}

/*
// Return the nth odd prime, counting from 0, by checking each number
// up to its square root for even divisors.
pub fn nth_prime_naive(n: u32) -> u32 {
    primes_from_5().nth(n as usize).unwrap()
}
*/

// Return the nth odd prime, counting from 1, using a Vec.
pub fn nth_prime_vec(n: u32) -> u32 {

    // Maintain a list of primes we have found, in ascending order
    // by the next multiple of it that we will see.  Make sure that
    // there is only one entry with a given multiple.
    #[derive(Debug)]
    struct Prime {
        num: u32,
        multiple: u32,
    }
    impl Prime {
        fn new(num: u32) -> Prime {
            Prime { num, multiple: 3*num }
        }
    }
    let mut primes: Vec<Prime> = Vec::new();

    fn insert(mut prime: Prime, primes: &mut Vec<Prime>) {
        let mut insert_point: usize = 0;
        for (i, p) in primes.iter().enumerate().rev() {
            if p.multiple >= prime.multiple {
                if p.multiple > prime.multiple {
                    insert_point = i+1;
                    break;
                }
                prime.multiple += prime.num;
            }
        }
        primes.insert(insert_point, prime);
    }

    let mut count = 1;
    let mut candidate = 3;
    loop {
        let mut is_prime = true;
        if let Some(prime) = primes.last() {
            if candidate == prime.multiple {
                is_prime = false;
            }
        }
        if is_prime {
            println!("Found prime: {}; list: {:?}", candidate, primes);
            if count == n { return candidate }
            insert( Prime::new(candidate), &mut primes);
            count += 1;
        } else {
            if let Some(mut prime) = primes.pop() {
                prime.multiple += prime.num;
                insert(prime, &mut primes);
            }
        }
        candidate += 2;
    }
}

// Return the nth odd prime, counting from 0, using a LinkedList.
pub fn nth_prime_ll(n: u32) -> u32 {

    // Maintain a list of primes we have found, in descending order
    // by the next multiple of it that we will see.  Make sure that
    // there is only one entry with a given multiple.
    #[derive(Debug)]
    struct Prime {
        num: u32,
        multiple: u32,
    }
    impl Prime {
        fn new(num: u32) -> Prime {
            Prime { num, multiple: 2*num }
        }
    }
    let mut primes: LinkedList<Prime> = LinkedList::new();
    fn insert(mut prime: Prime, primes: &mut LinkedList<Prime>) {
        let mut insert_before_ix: usize = std::usize::MAX;  // means add at end
        for (i, p) in primes.iter().enumerate() {
            if p.multiple >= prime.multiple {
                if p.multiple > prime.multiple {
                    insert_before_ix = i;
                    break;
                }
                prime.multiple += prime.num;
            }
        }
        match insert_before_ix {
            std::usize::MAX => primes.push_back(prime),
            0 => primes.push_front(prime),
            _  => {
                let mut back = primes.split_off(insert_before_ix);
                primes.push_back(prime);
                primes.append(&mut back);
            }
        }
    }

    let mut count = 1;
    let mut candidate = 3;
    loop {
        let mut is_prime = true;
        if let Some(prime) = primes.front() {
            if candidate == prime.multiple {
                is_prime = false;
            }
        }
        if is_prime {
            if count == n { return candidate }
            insert( Prime::new(candidate), &mut primes);
            count += 1;
        } else {
            if let Some(mut prime) = primes.pop_front() {
                prime.multiple += prime.num;
                insert(prime, &mut primes);
            }
        }
        candidate += 2;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_candidates() {
        let first_few: Vec<_> = CandidateIterator::new().take(10).collect();
        assert_eq!(first_few, vec![5,7,11,13,17,19,23,25,29,31]);
    }

    /*
    #[test]
    fn test_primes_from_5() {
        let first_few: Vec<_> = primes_from_5().take(10).collect();
        assert_eq!(first_few, vec![5,7,11,13,17,19,23,29,31,37]);
    }
    */

    #[test]
    fn test_with() {
        let xs: Vec<(u32, u32)> = (0..19).with_asc_int_sqrt_from(0).collect();
        let ys: Vec<(u32, u32)> = vec![
            ( 0,0), ( 1,1), ( 2,1), ( 3,1),
            ( 4,2), ( 5,2), ( 6,2), ( 7,2),
            ( 8,2), ( 9,3), (10,3), (11,3),
            (12,3), (13,3), (14,3), (15,3),
            (16,4), (17,4), (18,4), (19,4),
        ];
        assert_eq!(xs, ys);
    }
}
