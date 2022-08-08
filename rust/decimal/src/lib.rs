/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
    integer_part: usize,
    fractional_part: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        input.split_once('.').map(|(i, f)| Decimal {
            integer_part: i.parse().unwrap(),
            fractional_part: f.parse().unwrap(),
        })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        other.integer_part == self.integer_part && other.fractional_part == other.fractional_part
    }
}

impl std::ops::Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Decimal {
            integer_part: self.integer_part + rhs.integer_part,
            fractional_part: self.fractional_part + rhs.fractional_part,
        }
    }
}

impl std::ops::Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Decimal {
            integer_part: self.integer_part - rhs.integer_part,
            fractional_part: self.fractional_part - rhs.fractional_part,
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.integer_part != other.integer_part {
            self.integer_part.partial_cmp(&other.integer_part)
        } else {
            self.fractional_part.partial_cmp(&other.fractional_part)
        }
    }
}

impl std::ops::Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Decimal {
            integer_part: self.integer_part * rhs.integer_part,
            fractional_part: self.fractional_part * rhs.fractional_part,
        }
    }
}
