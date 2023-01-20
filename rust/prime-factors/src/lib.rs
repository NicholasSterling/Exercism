pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut primes: Vec<u64> = Vec::new();
    for i in 2.. {
        while n % i == 0 {
            primes.push(i);
            n /= i;
        }
        if i > n { break }
    }
    primes
}
