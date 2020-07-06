use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let letters: HashMap<char, u8> = char_count(word);

    for possible_anagram in possible_anagrams {
        let candidate = possible_anagram.to_lowercase();

        if candidate == word.to_lowercase() {
            continue;
        }

        if char_count(possible_anagram).eq(&letters) {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}

fn char_count(word: &str) -> HashMap<char, u8> {
    let mut letters: HashMap<char, u8> = HashMap::new();

    word.to_lowercase().chars().for_each(|letter| {
        letters.entry(letter).and_modify(|c| *c += 1).or_insert(1);
    });

    return letters;
}
