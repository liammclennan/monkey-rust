use std::io::{self, BufRead};
use lexer::token::TokenType;
mod lexer;

fn main() {
    println!("Monkey");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut lex = lexer::Lexer::new(line);
        let mut token;
        
        loop {
            token = lex.next_token();
            if token.token_type == TokenType::Eof {
                break;
            }
            println!("{:?}", token);
        }

    }
}