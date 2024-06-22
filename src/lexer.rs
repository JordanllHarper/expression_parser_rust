use std::{fmt::Debug, thread::current, usize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Paren {
    Opening,
    Closing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    IntegerNum(i32),
    Operator(Operator),
    Paren(Paren),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer {
    raw_expression: String,
    current_index: usize,
}

impl Lexer {
    pub fn new(raw_expression: &str) -> Self {
        Self {
            raw_expression: raw_expression
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect(),
            current_index: 0,
        }
    }
}

fn parse_operator_or_paren(c: &char) -> Option<Token> {
    let token = match c {
        '+' => Token::Operator(Operator::Add),
        '-' => Token::Operator(Operator::Subtract),
        '/' => Token::Operator(Operator::Divide),
        '*' => Token::Operator(Operator::Multiply),
        '(' => Token::Paren(Paren::Opening),
        ')' => Token::Paren(Paren::Closing),
        _ => return None,
    };
    Some(token)
}
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let characters = self.raw_expression.chars().collect::<Vec<char>>();

        let current_character = characters.get(self.current_index);

        if let Some(c) = current_character {
            let maybe_token = parse_operator_or_paren(c);
            if let Some(t) = maybe_token {
                self.current_index += 1;
                return Some(t);
            } else {
                let value = characters
                    .split_at(self.current_index)
                    .1
                    .iter()
                    .take_while(|c| {
                        if c.is_ascii_digit() {
                            self.current_index += 1;
                            true
                        } else {
                            dbg!(c);
                            false
                        }
                    })
                    .collect::<String>()
                    .parse::<i32>()
                    .ok()?;
                return Some(Token::IntegerNum(value));
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Operator;

    use super::{Lexer, Token};

    #[test]
    fn single_number() {
        let data = "2";
        let lexer = Lexer::new(data);
        let expected = vec![Token::IntegerNum(2)];
        let actual = lexer.collect::<Vec<Token>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn double_digit() {
        let data = "22";
        let lexer = Lexer::new(data);
        let expected = vec![Token::IntegerNum(22)];
        let actual = lexer.collect::<Vec<Token>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn addition() {
        let data = "2+2";
        let lexer = Lexer::new(data);
        let expected = vec![
            Token::IntegerNum(2),
            Token::Operator(Operator::Add),
            Token::IntegerNum(2),
        ];
        let actual = lexer.collect::<Vec<Token>>();
        assert_eq!(expected, actual);
    }
}
