use std::collections::BTreeMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    commands: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Vec::new(),
            commands: BTreeMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens: Vec<String> = input
            .split_ascii_whitespace()
            .map(|t| t.to_lowercase())
            .rev()
            .collect();

        while let Some(token) = tokens.pop() {
            if let Ok(digit) = token.parse::<i32>() {
                self.stack.push(digit);
                continue;
            }

            if let Some(commands) = self.commands.get(&token) {
                tokens.extend(&mut commands.clone().into_iter().rev());
                continue;
            }

            match token.as_str() {
                ":" => {
                    let mut command_tokens: Vec<String> = Vec::new();
                    let name: String;

                    if let Some(word) = tokens.pop() {
                        name = word;
                    } else {
                        return Err(Error::InvalidWord);
                    }

                    if name.parse::<i32>().is_ok() {
                        return Err(Error::InvalidWord);
                    }

                    let mut end = false;

                    while let Some(word) = tokens.pop() {
                        if word == ";" {
                            end = true;
                            break;
                        } else if let Some(commands) = self.commands.get(&word) {
                            command_tokens.append(&mut commands.clone());
                        } else {
                            command_tokens.push(word);
                        }
                    }

                    if !end {
                        return Err(Error::InvalidWord);
                    }

                    if command_tokens.is_empty() {
                        return Err(Error::StackUnderflow);
                    }

                    self.commands.insert(name, command_tokens);
                }
                "+" => {
                    let (a, b) = (self.stack.pop(), self.stack.pop());
                    if a.is_none() || b.is_none() {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(a.unwrap() + b.unwrap())
                }
                "-" => {
                    let (a, b) = (self.stack.pop(), self.stack.pop());
                    if a.is_none() || b.is_none() {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(b.unwrap() - a.unwrap())
                }
                "*" => {
                    let (a, b) = (self.stack.pop(), self.stack.pop());
                    if a.is_none() || b.is_none() {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(b.unwrap() * a.unwrap())
                }
                "dup" => {
                    let a = self.stack.pop();
                    if a.is_none() {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(a.unwrap());
                    self.stack.push(a.unwrap());
                }
                "drop" => {
                    if let None = self.stack.pop() {
                        return Err(Error::StackUnderflow);
                    }
                }
                "swap" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => {
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                    _ => return Err(Error::StackUnderflow),
                },
                "over" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => {
                        self.stack.push(b);
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                    _ => return Err(Error::StackUnderflow),
                },
                "/" => {
                    // match with tuple
                    let (a, b) = (self.stack.pop(), self.stack.pop());
                    if a.is_none() || b.is_none() {
                        return Err(Error::StackUnderflow);
                    }
                    if a == Some(0) {
                        return Err(Error::DivisionByZero);
                    }
                    self.stack.push(b.unwrap() / a.unwrap())
                }
                _ => {
                    return Err(Error::UnknownWord);
                }
            }
        }
        return Ok(());
    }
}
