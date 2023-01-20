// Returns a Vec of all of the substrings of digits of length len.
// Create a range for all of the substring starting indices and map
// those indices to the substrings.
pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..(digits.len()+1-len)).map(|start| digits[start..(start+len)].to_string()).collect()
}
