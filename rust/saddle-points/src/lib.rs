pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_maxes = vec![];
    input.iter().enumerate().for_each(|(i, row)| {
        row_maxes.push((i, row.iter().enumerate().max_by(|a, b| b.0.cmp(&a.0))));
    });

    dbg!(row_maxes);

    return vec![];
}
