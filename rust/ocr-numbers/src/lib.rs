// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

static HEIGHT: usize = 4;
static WIDTH: usize = 3;

const ZERO: &'static str = " _ | ||_|   ";
const ONE: &'static str = "     |  |   ";
const TWO: &'static str = " _  _||_    ";
const THREE: &'static str = " _  _| _|   ";
const FOUR: &'static str = "   |_|  |   ";
const FIVE: &'static str = " _ |_  _|   ";
const SIX: &'static str = " _ |_ |_|   ";
const SEVEN: &'static str = " _   |  |   ";
const EIGHT: &'static str = " _ |_||_|   ";
const NINE: &'static str = " _ |_| _|   ";

pub fn convert(input: &str) -> Result<String, Error> {
    let cells: Vec<String> = input.split('\n').map(|l| l.to_string()).collect();

    if cells.len() % HEIGHT != 0 {
        return Err(Error::InvalidRowCount(cells.len()));
    };

    if let Some(invalid_columns) = cells.iter().find(|l| l.chars().count() % WIDTH != 0) {
        return Err(Error::InvalidColumnCount(invalid_columns.len()));
    }

    let mut solution = String::new();
    let mut buffer = String::with_capacity(WIDTH * HEIGHT);

    let rows = cells.len() / HEIGHT;
    let columns = cells.first().unwrap().len() / WIDTH;

    for row_cell in 0..rows {
        for column_cell in 0..columns {
            for row in (row_cell * HEIGHT)..((row_cell + 1) * HEIGHT) {
                buffer.push_str(&cells[row][(column_cell * WIDTH)..((column_cell + 1) * WIDTH)]);
            }

            match buffer.as_ref() {
                ZERO => solution.push('0'),
                ONE => solution.push('1'),
                TWO => solution.push('2'),
                THREE => solution.push('3'),
                FOUR => solution.push('4'),
                FIVE => solution.push('5'),
                SIX => solution.push('6'),
                SEVEN => solution.push('7'),
                EIGHT => solution.push('8'),
                NINE => solution.push('9'),
                _ => solution.push('?'),
            }

            buffer.clear();
        }

        if row_cell != rows - 1 {
            solution.push(',');
        }
    }

    return Ok(solution);
}
