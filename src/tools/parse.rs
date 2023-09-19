use std::fmt::Debug;
use crate::structure::position::{Located, Position};
use super::lex::Lexable;

pub type ParseResult<T> = Result<Located<T>, Located<String>>;
pub trait Parsable<T: Lexable> where Self: Sized + Debug + Clone + PartialEq {
    fn parse(parser: &mut Parser<T>) -> ParseResult<Self>;
}
pub struct Parser<T: Lexable> {
    tokens: Vec<Located<T>>
}
impl<T: Lexable> Parser<T> {
    pub fn new(tokens: Vec<Located<T>>) -> Self {
        Self { tokens }
    }
    pub fn get(&mut self) -> Option<Located<T>> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.remove(0))
        }
    }
    pub fn peek(&self) -> Option<&Located<T>> {
        self.tokens.first()
    }
    pub fn expect(&mut self) -> Result<Located<T>, Located<String>> {
        let Some(token) = self.get() else {
            return Err(Located::new("unexpected end of input".into(), Position::default()))
        };
        Ok(token)
    }
    pub fn expect_peek(&self) -> Result<&Located<T>, Located<String>> {
        let Some(token) = self.peek() else {
            return Err(Located::new("unexpected end of input".into(), Position::default()))
        };
        Ok(token)
    }
    pub fn expect_token(&mut self, expect: T) -> Result<Located<T>, Located<String>> {
        let token = self.expect()?;
        if expect != token.value {
            return Err(Located::new(format!("expected token {expect:?}, got token {token:?}"), token.pos))
        }
        Ok(token)
    }
    pub fn get_map<U, F: Fn(Located<T>) -> Option<U>>(&mut self, f: F) -> Option<U> {
        f(self.get()?)
    }
}