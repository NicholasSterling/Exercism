/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
//    is_valid_imperative(code)
    is_valid_functional(code)
}

pub fn is_valid_imperative(code: &str) -> bool {
    let mut count = 0;
    let mut double = false;
    let mut sum = 0;
    for c in code.chars().rev() {
        if c != ' ' {
            match c.to_digit(10) {
                None => return false,
                Some(d) => {
                    sum += if double {
                        if d < 5 { d + d } else { d + d - 9 }
                    } else {
                        d
                    };
                    double = !double;
                }
            }
            count += 1;
        }
    }
    count > 1 && sum % 10 == 0
}

pub fn is_valid_functional(code: &str) -> bool {
    code.chars().rev()
        .filter(|&c| c != ' ')
        .try_fold((0,0), |(count,sum), c: char|
            c.to_digit(10).map(|d| {
                let inc =
                    if count % 2 == 0 { d } else {
                        if d < 5 { d+d } else { d+d-9 }
                    };
                (count+1, sum+inc)
            })
        ).map_or(false, |(count, sum)| count > 1 && sum % 10 == 0)
}
