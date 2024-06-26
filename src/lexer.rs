static NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']; 
static NUMERICS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.']; 

use tokens::{Token,TokenType};

pub struct Lexer {
    source_code: String,
    pub tokens: Vec<tokens::Token>,
    index: usize,
    current_line: usize
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Lexer {
            source_code,
            tokens: vec![],
            index: 0,
            current_line: 1
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

    fn build_number(&mut self) {
        let mut built_number = String::from("");
        let start_index = self.index;
        let mut decimal_encountered = false;
        while NUMERICS.contains(&self.get_current_character()) {
            let current_char = &self.get_current_character();
            if decimal_encountered && *current_char == '.' {
                //todo: later, we will have to make it so that the second . acts as an accessor for
                //the properties of the number itself
                panic!("ERROR: line:{:?}:{:?} Can not have 2 decimal points", self.current_line, self.index);
            } else {
                if *current_char == '.' {
                    decimal_encountered = true;
                }
                built_number.push(self.get_current_character());
                self.advance();
            }
        }
        let lexeme = built_number.clone();
        let built_number = built_number.parse::<f32>();
        if let Ok(converted_number) = built_number {
            self.build_token(TokenType::PenNumber(converted_number), start_index, lexeme);
        } else {
            panic!("ERROR: line:{:?}:{:?} Invalid float type: {:?}", self.current_line, self.index, lexeme);
        }
    }

    fn build_string(&mut self) {
        let mut built_string = String::from("");
        let start_index = self.index;
        self.advance();
        while self.get_current_character() != '"' {
            if self.get_current_character() == '\\' {
                built_string.push(self.get_current_character());
                self.advance();
            }
            built_string.push(self.get_current_character());
            self.advance();
        }
        let lexeme = built_string.clone();
        self.build_token(TokenType::PenString(built_string), start_index, lexeme);
    }

    fn build_token(&mut self,tt: TokenType, start_index: usize, lexeme: String) {
        self.tokens.push(Token {
            index: start_index,
            lexeme: lexeme,
            line: self.current_line,
            token_type: tt
        });
    }

    pub fn advance(&mut self) {
        if self.should_advance() {
            self.index += 1;
        }
    }

    pub fn lex(&mut self) {
        while self.should_advance() {
            let current_character = self.get_current_character();
            match current_character {
                '\t' | ' ' => {
                    // these characters should be ignored.
                    self.advance();
                },
                '\n' => {
                    self.current_line += 1;
                    self.advance();
                }
                '+' => {
                    self.build_token(TokenType::Plus, self.index, "+".to_string());
                    self.advance();
                }
                '"' => {
                    self.build_string();
                    self.advance();
                }
                c => {
                    if NUMBERS.contains(&c) {
                        self.build_number();
                    } else {
                        println!("ERROR: Unexpected Token: '{:?}'", r#c);
                        todo!();
                    }
                }
            }
        }
    }
}

pub mod tokens;
