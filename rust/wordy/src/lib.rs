enum Word {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Operand(i32),
    Invalid,
}

pub fn answer(command: &str) -> Option<i32> {
    let equation: Vec<Word> = command
        .trim_end_matches("?")
        .split_whitespace()
        .skip(2)
        .filter(|word| *word != "by")
        .map(|word| match word {
            "plus" => Word::Addition,
            "minus" => Word::Subtraction,
            "multiplied" => Word::Multiplication,
            "divided" => Word::Division,
            _ => {
                if let Ok(operand) = word.parse::<i32>() {
                    Word::Operand(operand)
                } else {
                    Word::Invalid
                }
            }
        })
        .collect();

    if equation.is_empty() {
        return None;
    }

    let mut answer = 0i32;
    let mut words = equation.into_iter();

    while let Some(word) = words.next() {
        match word {
            Word::Invalid => return None,
            Word::Operand(operand) => {
                if answer == 0 {
                    answer = operand
                } else {
                    return None;
                }
            }
            operator @ _ => {
                if let Some(Word::Operand(operand)) = words.next() {
                    match operator {
                        Word::Addition => answer += operand,
                        Word::Subtraction => answer -= operand,
                        Word::Multiplication => answer *= operand,
                        Word::Division => answer /= operand,
                        _ => {}
                    }
                } else {
                    return None;
                }
            }
        }
    }

    Some(answer)
}
