#[derive(Debug)]
enum Square {
    Mine,
    Flag(u8),
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut board: Vec<Vec<Square>> = minefield
        .iter()
        .map(|&f| {
            f.chars()
                .map(|c| match c {
                    '*' => Square::Mine,
                    _ => Square::Flag(0),
                })
                .collect()
        })
        .collect();

    let rows = board.len();
    for row_num in 0..rows {
        let cols = board[row_num].len();
        for col_num in 0..cols {
            if let Square::Mine = board[row_num][col_num] {
                let start_row = if row_num == 0 { 0 } else { row_num - 1 };
                let start_col = if col_num == 0 { 0 } else { col_num - 1 };
                for i in start_row..std::cmp::min(rows, row_num + 2) {
                    for j in start_col..std::cmp::min(cols, col_num + 2) {
                        if let Square::Flag(sum) = board[i][j] {
                            board[i][j] = Square::Flag(sum + 1);
                        }
                    }
                }
            }
        }
    }

    return board
        .iter()
        .map(|row| {
            row.iter()
                .map(|i| match i {
                    Square::Flag(0) => " ".to_owned(),
                    Square::Flag(mines) => mines.to_string(),
                    Square::Mine => "*".to_owned(),
                })
                .collect()
        })
        .collect();
}
