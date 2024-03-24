use std::collections::HashMap;

/**
 * The `Token` struct, and associated behavior.
 */

#[derive(PartialEq,Debug,Clone)]
 pub enum TokenType {
    Illegal,
    Eof,
    Identifier,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(PartialEq,Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn lookup_word(word: &str) -> TokenType {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert("fn".to_string(), TokenType::Function);
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("return".to_string(), TokenType::Return);
        if let Some(tt) = keywords.get(word) {
            tt.clone()
        } else {
            TokenType::Identifier
        }
    }
}