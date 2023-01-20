pub fn is_leap_year(year: i32) -> bool {
    let is_mult_of = |n| year % n == 0;
    is_mult_of(4) && (!is_mult_of(100) || is_mult_of(400))
}
