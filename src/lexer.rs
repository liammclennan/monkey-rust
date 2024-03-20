use self::token::{Token,TokenType};

mod token;

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
            self.ch = b'\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphanumeric() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn next_token(&mut self) -> Token {
        let t = match self.ch {
            b'=' => Token { token_type: TokenType::Assign, literal: "=".to_string()},
            b'+' => Token { token_type: TokenType::Plus, literal: "+".to_string()},
            b';' => Token { token_type: TokenType::Semicolon, literal: ";".to_string()},
            b'(' => Token { token_type: TokenType::LeftParen, literal: "(".to_string()},
            b')' => Token { token_type: TokenType::RightParen, literal: ")".to_string()},
            b',' => Token { token_type: TokenType::Comma, literal: ",".to_string()},
            b'{' => Token { token_type: TokenType::LeftBrace, literal: "{".to_string()},
            b'}' => Token { token_type: TokenType::RightBrace, literal: "}".to_string()},
            b'\0' => Token { token_type: TokenType::Eof, literal: "".to_string()},
            _ => {
                // if self.ch.is_ascii_alphabetic() {

                // }
                panic!("Gaahhhh")
                
            }
        };
        self.read_char();
        t
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
            (TokenType::Let, "let".to_string()),
        ];

        let mut lex = Lexer::new(input.to_string());

        for (tt, lit) in expecteds {
            let Token { token_type, literal } = lex.next_token();
            assert_eq!(token_type, tt);
            assert_eq!(literal, lit);
        }
    }

    #[test]
    fn read_word() {
        let input = "The cat, in the hat";
        let mut lex = Lexer::new(input.to_string());
        assert_eq!("The", lex.read_identifier());
    }
}
