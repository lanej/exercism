pub fn encode(source: &str) -> String {
    let mut result = String::new();
    if source.is_empty() {
        return result;
    }
    let mut chars = source.chars().into_iter();
    let mut target = chars.next().unwrap();
    let mut target_count = 1;
    chars.for_each(|letter| {
        if letter == target {
            target_count += 1
        } else {
            if target_count > 1 {
                result.push_str(&format!("{}{}", target_count, target));
            } else {
                result.push(target);
            }
            target = letter;
            target_count = 1;
        }
    });
    if target_count > 1 {
        result.push_str(&format!("{}{}", target_count, target));
    } else {
        result.push(target);
    }
    result
}

pub fn decode(source: &str) -> String {
    // result.push_str(
    //     std::iter::repeat(letter)
    //         .take(target_count)
    //         .collect::<String>()
    //         .as_str(),
    // );
    unimplemented!("Return the run-length decoding of {}.", source);
}
