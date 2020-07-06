use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split(|c: char| c == ',' || c.is_ascii_whitespace())
        .map(|word| {
            word.trim_start_matches(|c: char| c.is_ascii_control() || c.is_ascii_punctuation())
                .trim_end_matches(|c: char| c.is_ascii_control() || c.is_ascii_punctuation())
        })
        .filter(|word| !word.chars().all(|c| c.is_ascii_control()))
        .fold(HashMap::new(), |mut acc, word| {
            acc.entry(word.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            acc
        })
}
