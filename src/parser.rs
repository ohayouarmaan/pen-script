#[warn(dead_code, unused_imports)]
use crate::lexer::tokens::{Token, TokenType};
use crate::lexer::Lexer;

pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(l: &Lexer<'a>) -> Self {
        Self {
            tokens: l.tokens.to_vec(),
            index: 0,
        }
    }

    fn match_tokens(&self, tokens: &[TokenType]) -> bool {
        tokens.contains(&self.tokens[self.index].token_type)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn parse_factor(&mut self) -> nodes::Expr<'a> {
        self.create_binary_expr(vec![TokenType::Divide, TokenType::Multiply], Self::parse_unary)
    }

    fn parse_unary(&mut self) -> nodes::Expr<'a> {
        if self.match_tokens(&[TokenType::Not, TokenType::Minus]) {
            let operator_type = self.tokens[self.index].token_type.clone();
            self.advance();
            nodes::Expr::UnaryOp(nodes::UnaryOperation {
                operator: operator_type,
                right: Box::new(self.parse_unary()),
            })
        } else {
            self.parse_literal()
        }
    }

    fn parse_literal(&mut self) -> nodes::Expr<'a> {
        match self.tokens[self.index].token_type {
            TokenType::PenNumber(x) => {
                self.advance();
                return nodes::Expr::Litvalue(
                    nodes::LiteralValue::PenNumber(x)
                )
            }
            TokenType::LParen => {
                self.advance();
                let exp = self.parse();
                if(self.tokens[self.index].token_type == TokenType::RParen) {
                    self.advance();
                    return exp;
                } else {
                    panic!("Expected a ')'");
                }
            }
            _ => {
                todo!();
            }
        }
    }

    fn create_binary_expr(&mut self, match_tokens: Vec<TokenType>, precedent_function: fn(&mut Self) -> nodes::Expr<'a>) -> nodes::Expr<'a> {
        let mut expr = precedent_function(self);
        println!("expr: {:?}", expr);
        while {
            self.index < self.tokens.len() &&
            self.match_tokens(&match_tokens)
        } {
            let operator = self.tokens[self.index].token_type.clone();
            self.advance();
            let rhs = precedent_function(self);
            expr = nodes::Expr::BinOp(Box::new(nodes::BinaryOperation {
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            }));
        }

        expr
    }

    fn parse_additive(&mut self) -> nodes::Expr<'a> {
        self.create_binary_expr(vec![TokenType::Plus, TokenType::Minus], Self::parse_factor)
    }

    pub fn parse(&mut self) -> nodes::Expr<'a> {
        let expr = self.parse_additive();
        return expr;
    }
}

mod nodes;
