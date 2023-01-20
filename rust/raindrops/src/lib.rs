const FACTORS: [(usize, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: usize) -> String {

    /*
    // IMPERATIVE SOLUTION
    let mut text = String::new();
    for (factor, name) in FACTORS.iter() {
        if n % factor == 0 {
            text.push_str(*name);
        }
    }
    if text.is_empty() {
        n.to_string()
    } else {
        text
    }
    */

    // FUNCTIONAL SOLUTION
    let divisors: Vec<_> =
        FACTORS.iter()
            .filter_map(|(factor, name)|
                if n % *factor == 0 { Some(*name) } else { None }
            )
            .collect();
    if divisors.is_empty() {
        n.to_string()
    } else {
        divisors.join("")
    }

}
