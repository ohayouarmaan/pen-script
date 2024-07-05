use crate::lexer::tokens::{Token, TokenType};

#[derive(Debug)]
pub enum Expr<'a> {
    BinOp(Box<BinaryOperation<'a>>),
    Litvalue(LiteralValue),
    UnaryOp(UnaryOperation<'a>),
}

#[derive(Debug)]
pub struct BinaryOperation<'a> {
    //todo: think about the fields required in a binop
    pub left: Box<Expr<'a>>,
    pub operator: TokenType<'a>,
    pub right: Box<Expr<'a>>
}

#[derive(Debug)]
pub struct UnaryOperation<'a> {
    //todo: think about the fields required in a binop
    pub operator: TokenType<'a>,
    pub right: Box<Expr<'a>>
}

#[derive(Debug)]
pub enum LiteralValue {
    PenNumber(f32),
    PenString(String),
}
