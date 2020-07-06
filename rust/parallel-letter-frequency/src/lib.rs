use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut results = HashMap::new();
    let source = input.chunks(worker_count);

    for chunk in source {
        thread::spawn(move || println!("chunk\n{:?}", chunk));
        // thread::spawn(move || println!("chunk: {:?}\n", chunk));
        // thread::spawn(|| results.entry('a').and_modify(|e| *e += 1).or_insert(1))
    }

    results.entry('a').and_modify(|e| *e += 1).or_insert(1);

    results
}
