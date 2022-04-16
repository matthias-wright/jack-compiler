//! Defines the token.
use std::rc::Rc;
use crate::io::line::Line;

/// Wrapper type that holds a token and the corresponding [`Line`](crate::io::line::Line).
#[derive(Debug)]
pub struct TokenWrapper {
    pub token: Token,
    pub line: Rc<Line>
}

#[derive(Debug)]
pub enum Token {
    Symbol(String),
    Keyword(String),
    Constant(Constant),
    Identifier(String),
}

#[derive(Debug)]
pub enum Constant {
    IntegerConstant(u32),
    StringConstant(String),
    BooleanConstant(bool),
}