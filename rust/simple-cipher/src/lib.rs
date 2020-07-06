pub fn encode(key: &str, plaintext: &str) -> Option<String> {
    assert!(key.len() >= plaintext.len());

    let mut plaintext_bytes = plaintext.bytes();

    Some(
        key.bytes()
            .map(|key_byte| (key_byte - (plaintext_bytes.next().unwrap() - 'a' as u8)) as char)
            .collect::<String>(),
    )
}

pub fn decode(key: &str, ciphertext: &str) -> Option<String> {
    assert!(key.len() >= ciphertext.len());

    let mut ciphertext_bytes = ciphertext.bytes();

    Some(
        key.bytes()
            .map(|key_byte| (ciphertext_bytes.next().unwrap() - (key_byte - 'a' as u8)) as char)
            .collect::<String>(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {}. Return tuple (key, encoded s)",
        s
    )
}
