#![feature(range_contains)]

// Returns the # of grains of rice on square s (1...64) of the chess board.
pub fn square(s: u32) -> u64 {
    assert!((1..65).contains(&s), "Square must be between 1 and 64");
    1u64 << (s-1)
}

// Returns the # of grains of rice on all squares combined.
// Since each square holds a power of 2, and each power of 2 corresponds
// to a bit in an integer, just set them all at once.
pub fn total() -> u64 {
    !0u64
}
