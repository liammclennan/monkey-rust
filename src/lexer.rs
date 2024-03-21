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

    fn read_word(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphanumeric() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

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
                if self.ch.is_ascii_alphabetic() {
                    let word = self.read_word();
                    Token { token_type: Token::lookup_word(&word), literal: word }
                } else if self.ch.is_ascii_digit() {
                    let word = self.read_word();
                    Token { token_type: TokenType::Int, literal: word }
                } else {
                    Token { token_type: TokenType::Illegal, literal: (self.ch as char).to_string() }
                }
            }
        };
        self.read_char();
        t
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_test() {
        let input = "=+let ,521 if apple {}";

        let expecteds: Vec<(TokenType, String)> = vec![
            (TokenType::Assign, "=".to_string()),
            (TokenType::Plus, "+".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Int, "521".to_string()),
            (TokenType::If, "if".to_string()),
            (TokenType::Identifier, "apple".to_string()),
            (TokenType::LeftBrace, "{".to_string()),
            (TokenType::RightBrace, "}".to_string()),
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
        assert_eq!("The", lex.read_word());
    }
}
