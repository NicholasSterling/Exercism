fn add_part1(s: &mut String, n: i32) {
    if n == 0 {
        s.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, ");
    } else if n == 1 {
        s.push_str("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, ");
    } else {
        s.push_str(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, ", n, n).as_str());
    }
}

fn add_part2(s: &mut String, n: i32) {
    const SIMPLE_CASES: [&str; 3] = [
        "99 bottles of beer on the wall.\n",
        "no more bottles of beer on the wall.\n",
        "1 bottle of beer on the wall.\n",
    ];
    let n = n as usize;
    if n < SIMPLE_CASES.len() {
        s.push_str(SIMPLE_CASES[n]);
    } else {
        s.push_str(format!("{} bottles of beer on the wall.\n", n-1).as_str());
    }
}

fn add_verse(s: &mut String, n: i32) {
    add_part1(s, n);
    add_part2(s, n);
}

pub fn sing(start: i32, end: i32) -> String {
    let mut s = String::new();
    let mut first = true;
    for n in (end..=start).rev() {
        if first {
            first = false;
        } else {
            s.push('\n');
        }
        add_verse(&mut s, n);
    }
    s
}

pub fn verse(n: i32) -> String {
    sing(n, n)
}
