use lazy_static::lazy_static;
use std::collections::HashMap;

extern crate lazy_static;

lazy_static! {
    static ref HASHMAP: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('A', 1);
        m.insert('E', 1);
        m.insert('I', 1);
        m.insert('O', 1);
        m.insert('U', 1);
        m.insert('L', 1);
        m.insert('N', 1);
        m.insert('R', 1);
        m.insert('S', 1);
        m.insert('T', 1);
        m.insert('D', 2);
        m.insert('G', 2);
        m.insert('B', 3);
        m.insert('C', 3);
        m.insert('M', 3);
        m.insert('P', 3);
        m.insert('F', 4);
        m.insert('H', 4);
        m.insert('V', 4);
        m.insert('W', 4);
        m.insert('Y', 4);
        m.insert('K', 5);
        m.insert('J', 5);
        m.insert('X', 8);
        m.insert('Q', 10);
        m.insert('Z', 10);
        m
    };
}

pub fn score(word: &str) -> u64 {
    word.chars().fold(0u64, |acc, c| {
        println!("{}", c.to_ascii_uppercase());
        acc + *HASHMAP.get(&c.to_ascii_uppercase()).unwrap() as u64
    })
}
