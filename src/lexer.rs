#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Paren {
    Opening,
    Closing,
}

#[derive(Debug, Clone)]
pub enum Token {
    IntegerNum(i32),
    Operator(Operator),
    Paren(Paren),
}

pub struct Lexer {
    raw_expression: String,
    current_index: i32,
}

impl Lexer {
    pub fn tokenize(raw_expression: &str) -> Result<Vec<Token>, ()> {
        Ok([].to_vec())
    }
}
