// I re-did this exercise because I wanted to explore creating my own
// Iterator for the digits.

use std::ops::{Rem, Div};

trait Digits<T> {
    fn digits_in_base(self, base: T) -> DigitsIterator<T>;
}

impl<T> Digits<T> for T
where T: Copy + Ord + Div<Output = T> + Rem<Output = T>
{
    fn digits_in_base(self, base: T) -> DigitsIterator<T> {
        DigitsIterator { num: Some(self), base }
    }
}

struct DigitsIterator<T> {
    num: Option<T>,
    base: T,
}

impl<T> Iterator for DigitsIterator<T>
where T: Copy + Ord + Div<Output = T> + Rem<Output = T>
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if let Some(n) = self.num {
            if n >= self.base {
                let tmp = n % self.base;
                self.num = Some(n / self.base);
                Some(tmp)
            } else {
                std::mem::replace(&mut self.num, None)
            }
        } else {
            None
        }
    }
}


pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = num.digits_in_base(10).collect();
    let len = digits.len() as u32;
    dbg!(&digits);
    digits.iter().map(|x| x.pow(len)).sum::<u32>() == num
}

/*
// Original, straightforward solution.
pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::<u32>::new();
    let mut n = num;
    while n >= 10 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);
    let m = digits.len() as u32;
    let armstrong_num: u32 = digits.iter().map(|&x| x.pow(m)).sum();
    armstrong_num == num
}
*/
