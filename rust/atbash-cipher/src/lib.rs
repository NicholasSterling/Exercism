#![feature(nll)]

extern crate itertools;
extern crate boolinator;

use itertools::Itertools;
use boolinator::Boolinator;

pub fn encode(plaintext: &str) -> String {
    /* Jane's version
    plaintext
        .chars()
        .filter_map(atbash)
        .chunks(5)
        .into_iter()
        .flat_map(|chunk| Some(' ').into_iter().chain(chunk))
        .skip(1)
        .collect()
    */
//    /* My version
    plaintext.chars()
        .filter_map(atbash)
        .chunks(5).into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join(" ")
//    */
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext.chars().filter_map(atbash).collect()
}

fn atbash(c: char) -> Option<char> {
    c.is_ascii_alphanumeric().as_some_from(||
        if c.is_ascii_digit() { c } else {
            (b'z' - c.to_ascii_lowercase() as u8 + b'a') as char
        }
    )
}

// This version borrows from DuBistKomisch the use of filter_map and join.
// And lummax's solution helped me get Itertools's chunks to join.
// It would have been nice to be able to join the chunk iterators directly
// without having to map them to strings.
