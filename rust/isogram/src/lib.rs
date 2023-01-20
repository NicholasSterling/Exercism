use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut chars = HashSet::<char>::new();
    let s: String =
        candidate.to_lowercase().chars().filter(|&c| c.is_lowercase()).collect();
    for c in s.chars() {
        chars.insert(c);
    }
    chars.len() == s.len()
}
