pub struct Lexer {
    source_code: String,
    token_types: Vec<tokens::Token>
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Lexer {
            source_code,
            token_types: vec![]
        }
    }
}

pub mod tokens;
