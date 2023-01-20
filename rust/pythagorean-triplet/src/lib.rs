// See https://exercism.io/my/solutions/e4aca7b2502244dba57e8c279fd0530f
// I chose to make the algorithm efficient, at the expense of simplicity.
// For example, rather than computing a*a + b*b each time, I use the fact
// that (b+1)*(b+1) - b*b = b*b + 2b + 1 - b*b = 2b + 1, and similar for
// the decremented value of a, meaning that the new value of a*a + b*b is
// the old value plus 2*(b-a), where a is the next (decremented) value.
// To make things even more efficient, we label the edges in order by length,
// so that a <= b <= c, and constrain the loop ranges accordingly, but also
// breaking when we know that looking further is futile.  Even more efficient
// solutions are probably possible by doing nonlinear searches, e.g. on b.
// Compare with other solutions the number of tries it takes to determine
// that there is no solution for a given number (e.g. 41).
// And here is a great video about Pythagorean triples:
// https://www.youtube.com/watch?v=QJYmyhnaaek
pub fn find_triple_with_sum(sum: u32) -> Option<u32> {
    let mut num_tries = 0;
    // We label the values a,b,c such that a <= b <= c.
    // Since c is the largest, the least it can be is 1/3 of the sum
    // (rounded up).  The most it can be is the sum - 2, because the
    // other numbers must be at least 1.
    'c: for c in (sum+2) / 3 .. sum-1 {
        let c2 = c*c;
        // Since a <= b, the least b can be is half the remainder
        // (rounded up).  We'll start with that value and work our
        // way up (incrementing b and decrementing a) until either
        // a hits 0 or a*a + b*b (which will be increasing) >= c*c.
        let remainder = sum - c;
        let mut b = (remainder+1) / 2;
        let mut a = sum - c - b;
        let mut a2b2 = a*a + b*b;
        let mut increment = 2*(b-a+1);
        loop {
            num_tries += 1;
            println!("Trying {:?} -- {} vs {}", (a,b,c), a2b2, c2);
            if a2b2 >= c2 {
                if a2b2 > c2 { break }
                println!("Found {:?} in {} tries", (a,b,c), num_tries);
                return Some(a*b*c);
            }
            a -= 1;
            b += 1;
            a2b2 += increment;
            increment += 4;
            if a == 0 { break 'c }  // no point checking bigger values of c
        }
    }
    println!("No solution after {} tries", num_tries);
    return None;
}

pub fn find() -> Option<u32> {
    find_triple_with_sum(
        // 15  // 3 4 5
        // 40  // 8 15 17
        // 41  // no solution after 40 tries
        1000  // 200 375 425 in 765 tries
    )
}
