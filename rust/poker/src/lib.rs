#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
#[repr(u8)]
enum Value {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Win {
    HighCard(u8, u8, u8, u8, u8),
    Pair(u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8, u8, u8, u8, u8),
}

fn suit(card: &str) -> Suit {
    match card.chars().last() {
        Some('C') => Suit::Clubs,
        Some('D') => Suit::Diamonds,
        Some('H') => Suit::Hearts,
        Some('S') => Suit::Spades,
        _ => panic!("Invalid suit on card: {}", card),
    }
}

fn value(card: &str) -> Value {
    match &card[0..card.len() - 1] {
        "2" => Value::Two,
        "3" => Value::Three,
        "4" => Value::Four,
        "5" => Value::Five,
        "6" => Value::Six,
        "7" => Value::Seven,
        "8" => Value::Eight,
        "9" => Value::Nine,
        "10" => Value::Ten,
        "J" => Value::Jack,
        "Q" => Value::Queen,
        "K" => Value::King,
        "A" => Value::Ace,
        _ => panic!("Invalid value on card: {}", card),
    }
}

fn hand(hand: Vec<&str>) -> Win {
    let mut cards: Vec<(u8, u8)> = hand
        .into_iter()
        .map(|c| (suit(c) as u8, value(c) as u8))
        .collect();
    cards.sort_by(|(_, a), (_, b)| a.cmp(b));

    match cards[0..5] {
        [(sa, va), (sb, vb), (sc, vc), (sd, vd), (se, ve)]
            if (sa == sb)
                && (sb == sc)
                && (sc == sd)
                && (sd == se)
                && (vb == va + 1)
                && (vc == va + 2)
                && (vd == va + 3)
                && (ve == va + 4) =>
        {
            Win::StraightFlush(va, vb, vc, vd, ve)
        }
        [(sa, va), (sb, vb), (sc, vc), (sd, vd), (se, ace)]
            if (sa == sb)
                && (sb == sc)
                && (sc == sd)
                && (sd == se)
                && (vb == va + 1)
                && (vc == va + 2)
                && (vd == va + 3)
                && ace == Value::Ace as u8 =>
        {
            Win::StraightFlush(1, va, vb, vc, vd)
        }
        [(_, 2), (_, 3), (_, 4), (_, 5), (_, ace)] if ace == Value::Ace as u8 => Win::Straight(5),
        [(_, a), (_, b), (_, c), (_, d), (_, e)] | [(_, e), (_, a), (_, b), (_, c), (_, d)]
            if (a == b) && (b == c) && (c == d) =>
        {
            Win::FourOfAKind(a, e)
        }
        [(_, aa), (_, ab), (_, ac), (_, ba), (_, bb)]
        | [(_, ba), (_, bb), (_, aa), (_, ab), (_, ac)]
            if (aa == ab) && (ab == ac) && (ba == bb) =>
        {
            Win::FullHouse(aa, ba)
        }
        [(sa, va), (sb, vb), (sc, vc), (sd, vd), (se, ve)]
            if (sa == sb) && (sb == sc) && (sc == sd) && (sd == se) =>
        {
            Win::Flush(va, vb, vc, vd, ve)
        }
        [(_, va), (_, vb), (_, vc), (_, vd), (_, ve)]
            if (vb == va + 1) && (vc == va + 2) && (vd == va + 3) && (ve == va + 4) =>
        {
            Win::Straight(ve)
        }
        [(_, 2), (_, 3), (_, 4), (_, 5), (_, ace)] if ace == Value::Ace as u8 => Win::Straight(5),
        [(_, va), (_, vb), (_, vc), (_, vd), (_, ve)]
        | [(_, ve), (_, va), (_, vb), (_, vc), (_, vd)]
        | [(_, ve), (_, vd), (_, va), (_, vb), (_, vc)]
            if (va == vb) && (vb == vc) =>
        {
            Win::ThreeOfAKind(va, vd, ve)
        }
        [(_, va), (_, vb), (_, vc), (_, vd), (_, ve)]
        | [(_, ve), (_, va), (_, vb), (_, vc), (_, vd)]
        | [(_, va), (_, vb), (_, ve), (_, vc), (_, vd)]
            if (va == vb) && (vc == vd) =>
        {
            Win::TwoPair(vd, vb, ve)
        }
        [(_, va), (_, vb), ..]
        | [_, (_, va), (_, vb), ..]
        | [_, _, (_, va), (_, vb), ..]
        | [.., (_, va), (_, vb)]
            if va == vb =>
        {
            Win::Pair(va)
        }
        [(_, va), (_, vb), (_, vc), (_, vd), (_, ve)] => Win::HighCard(va, vb, vc, vd, ve),
        _ => panic!("Could not score hand"),
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winning_hands = hands
        .into_iter()
        .map(|h| (h, hand(h.split_whitespace().collect())))
        .collect::<Vec<(&&str, Win)>>();

    winning_hands.sort_by(|(_, a), (_, b)| a.cmp(b));

    let best_win = winning_hands.last().unwrap().1;

    winning_hands
        .into_iter()
        .rev()
        .take_while(|(_, v)| v == &best_win)
        .map(|(h, _)| *h)
        .collect()
}
