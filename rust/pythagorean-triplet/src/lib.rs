use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    for a in 2..sum {
        let a_sq = a.pow(2);
        for b in (a + 1)..(sum - 2) {
            let c = a + b;
            let b_sq = b.pow(2);
            let c_sq = c.pow(2);
            if c_sq == a_sq + b_sq {
                result.insert([a, b, c]);
            }
        }
    }
    result
}
