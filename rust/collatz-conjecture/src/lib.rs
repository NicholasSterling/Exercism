pub fn collatz(n: u64) -> Option<u64> {
    fn recur(n: u64, count: u64) -> u64 {
        if n == 1 {
            count
        } else {
            recur(if n % 2 == 0 { n/2 } else { 3*n + 1 }, count + 1)
        }
    }
    if n == 0 {
        None
    } else {
        Some(recur(n, 0))
    }
}
