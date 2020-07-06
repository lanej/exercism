use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut transformed: BTreeMap<char, i32> = BTreeMap::new();

    for (score, chars) in h.iter() {
        for char in chars.iter() {
            transformed.insert(char.to_ascii_lowercase(), *score);
        }
    }

    h.iter().for_each(|(score, chars)| {
        chars.iter().for_each(|c| {
            transformed
                .insert(c.to_ascii_lowercase(), *score)
                .expect("bad input");
        })
    });

    return transformed;
}
