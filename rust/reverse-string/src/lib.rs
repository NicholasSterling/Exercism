pub fn reverse(input: &str) -> String {

    #[cfg(feature = "grapheme")]
    {
        use unicode_segmentation::UnicodeSegmentation;
        input.graphemes(true).rev().collect()
    }

    #[cfg(not(feature = "grapheme"))]
    {
        input.chars().rev().collect()
    }
}

