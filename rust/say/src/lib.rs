pub fn encode(n: u64) -> String {

    const names20: [&str; 20] = [
        "", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
    ];

    const tens_names: [&str; 10] = [
        "oops", "oops", "twenty", "thirty", "forty",
        "fifty", "sixty", "seventy", "eighty", "ninety"
    ];

    const big_chunk_names: [&str; 7] = [
        "",
        " thousand",
        " million",
        " billion",
        " trillion",
        " quadrillion",
        " quintillion",
    ];

    let mut text = String::with_capacity(32);

    let mut last_was_hyphen = false;

    let say = |s: &str| {
        if !(s.is_empty() || last_was_hyphen) {
            text.push(' ');
        }
        text.push_str(s);
        last_was_hyphen = false;
    };

    let hyphen = || {
        text.push('-');
        last_was_hyphen = true;
    };

    // Say n under 100.
    let say_little_num = |mut n: u64| {
        if n < 20 {
            if n > 0 {
                say(names20[n as usize]);
            }
        } else {
            say(names20[(n/10) as usize]);
            if n%10 > 0 {
                hyphen();
                say(names20[(n%10) as usize]);
            }
        }
    };

    // Say n under 1000.
    let say_chunk = |mut n: u64| {
        if n > 99 {
            say(names20[(n/100) as usize]);
            say(" hundred");
            n %= 100;
        }
        say_little_num(n);
    };

    fn chunks_of(size: u64, mut n: u64) -> Vec<u64> {
        let mut chunks = Vec::<u64>::new();
        while n > 0 {
            chunks.push(n % size);
            n /= size;
        }
        chunks
    }

    if n == 0 {
        "zero".to_string()
    } else {
        // Break up the given number into chunks of 1000 and add
        // the corresponding text for each.
        for (i, &m) in chunks_of(1000, n).iter().enumerate().rev() {
            say_chunk(m as u64);
            say(big_chunk_names[i as usize]);
        }
        text
    }

}
