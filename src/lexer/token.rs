/**
 * The `Token` struct, and associated behavior.
 */

#[derive(PartialEq,Debug)]
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

#[derive(PartialEq,Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}