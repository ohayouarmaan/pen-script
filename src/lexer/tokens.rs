use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum TokenType {
    Print,
    Plus,
    Minus,
    Divide,
    Multiply,
    Not,
    EqualsTo,
    PenString(String),
    PenNumber(f32),
    Let,
}

#[derive(Debug)]
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
    ]);
    let value = string_to_hashmap.get(&kw[..]);
    if let Some(tt) = value {
        return Some((tt).clone());
    } else {
        return None;
    }
}

