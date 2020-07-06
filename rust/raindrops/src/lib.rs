pub fn raindrops(n: u32) -> String {
    let mut sound = String::new();

    if n % 3 == 0 {
        sound = sound + "Pling"
    }

    if n % 5 == 0 {
        sound = sound + "Plang"
    }

    if n % 7 == 0 {
        sound = sound + "Plong"
    }

    if sound == "" {
        sound = n.to_string()
    }

    return sound
}
