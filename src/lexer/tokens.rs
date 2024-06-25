pub enum TokenType {
    Print,
    PenString(String)
}

pub struct Token {
    pub line: usize,
    pub index: usize,
    pub lexeme: String,
    pub token_type: TokenType
}
