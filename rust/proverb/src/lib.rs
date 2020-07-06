pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.len() < 1 {
        return proverb
    }

    let mut prev = &list[0];

    let footer = format!("And all for the want of a {}.", prev);

    for next in &list[1..] {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", prev, next));
        prev = next;
    }

    proverb.push_str(&footer);

    return proverb
}
