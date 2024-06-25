pub struct Lexer {
    source_code: String,
    token_types: Vec<tokens::Token>,
    index: usize,
    current_line: usize
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Lexer {
            source_code,
            token_types: vec![],
            index: 0,
            current_line: 0
        }
    }

    fn should_advance(&self) -> bool {
        if self.index < self.source_code.len() {
            return true;
        }
        return false;
    }

    fn get_current_character(&self) -> char {
        if let Some(ch) = self.source_code.chars().nth(self.index) {
            return ch;
        } else {
            return '\0';
        }
    }

    pub fn lex(&mut self) {
        while self.should_advance() {
            let current_character = self.get_current_character();
            dbg!(current_character);
            self.index += 1;
        }
    }
}

pub mod tokens;
