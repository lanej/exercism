use std::fmt::{Display, Formatter, Result};

static NUMERALS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman {
    numerals: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "{}", self.numerals);
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut numerals = "".to_string();
        let mut remainder = num.clone();

        for (q, n) in NUMERALS.iter() {
            while remainder > 0 {
                if remainder >= *q {
                    remainder -= q;
                    numerals.push_str(n);
                } else {
                    break
                }
            }
        }

        return Roman { numerals };
    }
}
