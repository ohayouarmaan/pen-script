use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum TokenType {
    // Operators
    Plus,
    Minus,
    Divide,
    Multiply,
    Not,
    EqualsTo,
    EqualsEquals,
    NotEquals,

    // Syntax
    LBrace,
    RBrace,
    LParen,
    RParen,
    LSquare,
    RSquare,

    // Datatypes
    PenString(String),
    PenNumber(f32),

    // Userdefined Types
    // Later there will be some struct keywords and other stuffs
    Identifier,

    // Keywords
    Let,
    Print,
    Loop,
    Function,
    If,
    Else,
}

#[derive(Debug,Clone)]
pub struct Token {
    pub line: usize,
    pub index: usize,
    pub lexeme: String,
    pub token_type: TokenType
}

pub fn get_keyword(kw: String) -> Option<TokenType> {
    let string_to_hashmap: HashMap<&str, TokenType> = HashMap::from([
        ("print", TokenType::Print),
        ("let", TokenType::Let),
        ("loop", TokenType::Loop),
        ("function", TokenType::Function),
        ("if", TokenType::If),
        ("else", TokenType::Else),
    ]);
    let value = string_to_hashmap.get(&kw[..]);
    if let Some(tt) = value {
        return Some((tt).clone());
    } else {
        return None;
    }
}

