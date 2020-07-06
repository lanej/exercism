pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(self) -> bool {
        let luhn = self
            .code
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap_or_else(|| return false))
            .rev()
            .enumerate()
            .map(|(i, e)| if i % 2 != 0 { e * 2 % 9 } else { e })
            .collect::<Vec<u32>>();

        luhn.len() > 2 && luhn.iter().sum::<u32>() % 10 == 0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
