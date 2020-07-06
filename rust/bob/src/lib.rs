const QUESTION: &str = "Sure.";
const SHOUT: &str = "Whoa, chill out!";
const SHOUT_QUESTION: &str = "Calm down, I know what I'm doing!";
const EMPTY: &str = "Fine. Be that way!";
const DEFAULT: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim().replace(" ", "");
    if trimmed == "" {
        return EMPTY
    }

    let shout = message
        .trim()
        .split_whitespace()
        .any(|word| word.len() > 2
             && word.chars().all(|c| c.is_uppercase() || c == '!'));

    let question = trimmed.ends_with("?");

    match (shout, question) {
        (true, true) => SHOUT_QUESTION,
        (true, _) => SHOUT,
        (_, true) => QUESTION,
        _ => DEFAULT,
    }
}
