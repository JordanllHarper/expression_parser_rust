use lexer::{Lexer, Token};

mod lexer;
fn main() {
    let expression = "333+222";
    let tokens = Lexer::new(expression).collect::<Vec<Token>>();
    println!("Tokens:");
    for token in tokens {
        println!("{:?} ", token);
    }
}
