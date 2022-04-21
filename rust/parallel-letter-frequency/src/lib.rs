use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut frequency: HashMap<char, usize> = HashMap::new();

    let words: Vec<Vec<char>> = input
        .join(" ")
        .chars()
        .collect::<Vec<char>>()
        .chunks(worker_count)
        .map(|chars| chars.to_vec())
        .collect();

    if words.is_empty() {
        return frequency;
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let mut threads = Vec::with_capacity(worker_count);

    for chunk in words {
        let ttx = tx.clone();
        threads.push(std::thread::spawn(move || {
            for char in chunk.iter().filter(|c| char::is_alphabetic(**c)) {
                ttx.send(*char).unwrap();
            }
        }));
    }

    drop(tx);

    while let Ok(c) = rx.recv() {
        frequency
            .entry(c.to_ascii_lowercase())
            .and_modify(|ec| {
                *ec += 1;
            })
            .or_insert(1);
    }

    frequency
}
