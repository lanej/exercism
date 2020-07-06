pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::<Vec<u32>>::with_capacity(self.row_count as usize);

        if self.row_count == 0 {
            return rows;
        }

        rows.push(vec![1]);

        for i in 0..self.row_count - 1 {
            let prev_row = rows.get(i as usize).unwrap();
            let mut new_row: Vec<u32> = Vec::with_capacity(prev_row.len() + 1);
            new_row.push(1);

            for j in 1..prev_row.len() {
                new_row.push(prev_row.get(j - 1).unwrap() + prev_row.get(j).unwrap());
            }

            new_row.push(1);
            rows.push(new_row);
        }

        return rows;
    }
}
