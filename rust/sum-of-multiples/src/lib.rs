use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    for factor in factors {
        let mut multiplier: u32 = 1;
        let mut multiple: u32 = *factor;

        match multiple {
            0 => (),
            _ => {
                while multiple < limit {
                    multiples.insert(multiple);
                    multiplier = multiplier + 1;
                    multiple = multiplier * factor;
                }
            }
        }
    }

    multiples.iter().sum()
}
