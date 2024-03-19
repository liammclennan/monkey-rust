use self::token::{Token,TokenType};

mod token;

/**
 * The `Token` struct, and associated behavior.
 */

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        match self.ch {
            b'=' => Token { token_type: TokenType::Assign, literal: "=".to_string()},
            _ => Token { token_type: TokenType::Eof, literal: "".to_string() },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;

    #[test]
    fn next_token_test() {
        let input = "=+(){},;";

        let expecteds: Vec<(TokenType, String)> = vec![
            (TokenType::Assign, "=".to_string()),
            (TokenType::Plus, "+".to_string()),
        ];

        let lex = Lexer::new(input.to_string());

        for (tt, lit) in expecteds {
            assert!(false)
        }
    }
}
