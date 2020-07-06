use std::collections::HashMap;

const DNA: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !DNA.contains(nucleotide) {
        return Err(nucleotide);
    }

    match nucleotide_counts(dna) {
        Err(e) => Err(e),
        Ok(map) => {
            Ok(*map.get(&nucleotide).unwrap())
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = "ACGT".chars().map(|c| (c, 0)).collect();

    for letter in dna.chars() {
        match letter {
            'A' | 'C' | 'G' | 'T' => {
                counts.entry(letter).and_modify(|n| *n += 1);
            }
            _ => return Err(letter),
        }
    }

    Ok(counts)
}
