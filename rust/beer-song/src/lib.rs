const NO_BOTTLES: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const ONE_BOTTLE: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const TWO_BOTTLES: &str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(n: i32) -> String {
    match n {
        0 => NO_BOTTLES.to_string(),
        1 => ONE_BOTTLE.to_string(),
        2 => TWO_BOTTLES.to_string(),
        _ => format!("{current} bottles of beer on the wall, {current} bottles of beer.\nTake one down and pass it around, {next} bottles of beer on the wall.\n", current=n, next=n-1).to_string()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let verse_numbers: Vec<i32> = 
        match start > end {
            true => (end..start+1).rev().collect(),
            false => (start..end+1).collect(),
        };

    let mut song = String::new();

    for v in verse_numbers {
        song.push_str(&verse(v));
        song.push_str("\n");
    }
    
    song.pop();

    return song;
}
