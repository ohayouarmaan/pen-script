#[warn(dead_code, unused_imports)]
use crate::lexer::tokens::{Token,TokenType};
use crate::lexer::Lexer;

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(l: &Lexer) -> Self {
        Self {
            tokens: l.tokens.to_vec()
        }
    }
}

mod nodes;
