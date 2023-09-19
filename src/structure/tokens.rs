use super::position::{
    Position,
    Located
};
use crate::tools::lex::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i64), Float(f64), Boolean(bool), None, String(String),
    Ident(String),
    ExprIn, ExprOut, ArrayIn, ArrayOut, ObjIn, ObjOut,
    Add, Sub, Mul, Div, Pow, Mod,
    Field, Seperate
}