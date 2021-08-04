use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
}

pub struct Command<'a> {
    name: &'a str,
    instructions: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let commands: HashMap<&str, &str> = HashMap::new();
        let mut sequence: Option<Command> = None;

        for token in input.split(" ") {
            if let Ok(value) = i32::from_str_radix(token, 10) {
                self.stack.push(value);
                continue;
            }

            match token.to_lowercase().as_str() {
                ":" => {
                    match sequence {
                        Some(mut command) => command.instructions.push(token),
                        None => sequence = Some(Command { name: token, instructions: vec![] }),
                    }
                }
                "dup" => {
                    if self.stack.last().is_none() {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(*self.stack.last().unwrap());
                    continue;
                }
                "drop" => {
                    if self.stack.pop().is_none() {
                        return Err(Error::StackUnderflow);
                    }
                    continue;
                }
                "over" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let operand = *self.stack.get(self.stack.len() - 2).unwrap();
                    self.stack.push(operand);
                    continue;
                }
                "swap" => {
                    let last = self.stack.len();
                    if last < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.swap(last - 2, last - 1);
                    continue;
                }
                "+" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let mut terms = self.stack.drain(..);
                    let operand = terms.next().unwrap();
                    let result = terms.fold(operand, |acc, n| acc + n);
                    self.stack.push(result);
                }
                "-" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let mut terms = self.stack.drain(..);
                    let operand = terms.next().unwrap();
                    let result = terms.fold(operand, |acc, n| acc - n);
                    self.stack.push(result);
                }
                "*" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let mut terms = self.stack.drain(..);
                    let operand = terms.next().unwrap();
                    let result = terms.fold(operand, |acc, n| acc * n);
                    self.stack.push(result);
                }
                "/" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    if self.stack.iter().any(|&t| t == 0) {
                        return Err(Error::DivisionByZero);
                    }

                    let mut terms = self.stack.drain(..);
                    let operand = terms.next().unwrap();
                    let result = terms.fold(operand, |acc, n| acc / n);
                    self.stack.push(result);
                }
                _ => return Err(Error::UnknownWord),
            };
        }
        Ok(())
    }
}
