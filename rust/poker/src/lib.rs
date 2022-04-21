#[repr(u8)]
enum Suit {
    Spades = 'S' as u8,
    Hearts = 'H' as u8,
    Clubs = 'C' as u8,
    Diamonds = 'D' as u8,
}

#[repr(u8)]
enum Class {
    Hack = 'J' as u8,
    Queen = 'Q' as u8,
    King = 'K' as u8,
    Ace = 'A' as u8,
}

struct Card {
    suit: Suit,
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
