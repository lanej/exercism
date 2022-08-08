use std::collections::BTreeMap;

type Domino = (u8, u8);

trait Tile {
    fn contains(&self, _: u8) -> bool;
}

impl Tile for Domino {
    fn contains(&self, num: u8) -> bool {
        self.0 == num || self.1 == num
    }
}

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    if input.len() < 1 || (input.len() == 1 && input[0].0 == input[0].1) {
        return Some(input.to_vec());
    }
    let mut count: BTreeMap<u8, u8> = BTreeMap::new();
    input.iter().for_each(|d| {
        count.entry(d.0).and_modify(|l| *l += 1).or_insert(1);
        count.entry(d.1).and_modify(|l| *l += 1).or_insert(1);
    });

    if count.values().any(|v| v % 2 != 0) {
        return None;
    }

    let mut chain: Vec<Domino> = Vec::new();
    let mut remaining: Vec<Domino> = input.to_vec();
    let mut last = remaining.pop().unwrap();
    let (mut ordered, mut ends): (Vec<Domino>, Vec<Domino>) = remaining
        .iter()
        .partition(|d| d.contains(last.0));

    ordered.append(&mut ends);

    chain.push(last);

    while !ordered.is_empty() {
        let i = ordered.iter().position(|d| d.contains(last.1));
        if i.is_none() {
            return None;
        }
        let m = ordered.remove(i.unwrap());

        if last.1 == m.0 {
            last = (m.0, m.1);
            chain.push(last)
        } else {
            last = (m.1, m.0);
            chain.push(last)
        }
        dbg!(&chain);
        dbg!(&last);
    }

    dbg!(&chain);

    Some(chain)
}
