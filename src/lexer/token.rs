pub enum TokenType {
    Illegal,
    Eof,
    Identifier,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    Let,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}