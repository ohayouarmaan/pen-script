#[derive(Debug)]
pub enum TokenType {
    Print,
    Plus,
    Minus,
    Divide,
    Multiply,
    PenString(String),
    PenNumber(f32),
}

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub index: usize,
    pub lexeme: String,
    pub token_type: TokenType
}
