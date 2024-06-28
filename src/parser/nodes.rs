use crate::lexer::tokens::Token;

pub struct BinaryOperation {
    //todo: think about the fields required in a binop
    left: Token,
    operator: Token,
    right: Token
}
