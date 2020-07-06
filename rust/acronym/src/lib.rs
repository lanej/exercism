pub fn abbreviate(phrase: &str) -> String {
    let mut answer = String::new();

    phrase
        .split_whitespace()
        .flat_map(|w| w.split_terminator('-'))
        .map(|w| w.replace(":", ""))
        .map(|w| w.replace(",", ""))
        .for_each(|word| {
            let capital_letters = word
                .chars()
                .filter(|&c| c == c.to_ascii_uppercase())
                .collect::<String>();

            let abbreviation =
                if capital_letters.is_empty() || word.len() == capital_letters.len() {
                    word.chars()
                        .next()
                        .unwrap()
                        .to_ascii_uppercase()
                        .to_string()
                } else {
                    capital_letters
                };

            answer.push_str(&abbreviation);
        });

    return answer;
}
