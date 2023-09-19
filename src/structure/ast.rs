use crate::tools::parse::*;
use super::{
    position::{
        Located,
        Position
    },
    tokens::Token
};

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i64), Float(f64), Boolean(bool), None, String(String),
    Ident(String),
    Expression(Box<Located<Expression>>),
    Vector(Vec<Located<Expression>>),
    // Object(Box<Located<ObjectEntry>>),
    Field {
        head: Box<Located<Self>>,
        field: Located<String>
    },
    Index {
        head: Box<Located<Self>>,
        field: Box<Located<Expression>>
    },
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Pow, Mod
}
pub const BINARY_LAYERS: &[&[BinaryOperator]] = &[
    &[BinaryOperator::Add, BinaryOperator::Sub],
    &[BinaryOperator::Mul, BinaryOperator::Div, BinaryOperator::Mod],
    &[BinaryOperator::Pow],
];
impl BinaryOperator {
    pub fn layer(layer: usize) -> Option<&'static [Self]> {
        BINARY_LAYERS.get(layer).copied()
    }
    pub fn token(token: &Token) -> Option<Self> {
        match token {
            Token::Add => Some(Self::Add),
            Token::Sub => Some(Self::Sub),
            Token::Mul => Some(Self::Mul),
            Token::Div => Some(Self::Div),
            Token::Pow => Some(Self::Pow),
            Token::Mod => Some(Self::Mod),
            _ => None
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Neg, Not
}
pub const UNARY_LAYERS: &[&[UnaryOperator]] = &[
    &[UnaryOperator::Neg],
];
impl UnaryOperator {
    pub fn layer(layer: usize) -> Option<&'static [Self]> {
        UNARY_LAYERS.get(layer).copied()
    }
    pub fn token(token: &Token) -> Option<Self> {
        match token {
            Token::Sub => Some(Self::Neg),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Args(pub Vec<Located<Expression>>);

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(Atom),
    Binary { op: BinaryOperator, left: Box<Located<Self>>, right: Box<Located<Self>> },
    Unary { op: UnaryOperator, right: Box<Located<Self>> },
    Call { head: Located<Atom>, args: Located<Args> }
}
