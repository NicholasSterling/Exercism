pub fn square_of_sum(n: usize) -> usize {
    let x = n * (n+1) / 2;
    x*x
}

pub fn sum_of_squares(n: usize) -> usize {
    (1..n+1).map(|x| x*x ).sum()
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
