use std::fmt::Debug;

use crate::structure::position::{Position, Located};

pub trait Lexable where Self: Sized + Debug + Clone + PartialEq {
    fn step(lexer: &mut Lexer<Self>) -> Result<Option<Located<Self>>, Located<String>>;
}

pub struct Lexer<T: Lexable> {
    input: String,
    idx: usize, ln: usize, col: usize,
    tokens: Vec<Located<T>>
}
impl<T: Lexable> Lexer<T> {
    pub fn new(input: String) -> Self {
        Self { input, idx: 0, ln: 0, col: 0, tokens: vec![] }
    }
    pub fn get(&self) -> Option<char> {
        self.input.get(self.idx..=self.idx)?.chars().next()
    }
    pub fn pos(&self) -> Position {
        Position::new(self.ln..self.ln+1, self.col..self.col+1)
    }
    pub fn check<F: Fn(char) -> bool>(&self, f: F) -> bool {
        if let Some(c) = self.get() {
            f(c)
        } else {
            false
        }
    }
    pub fn advance(&mut self) {
        if self.get() == Some('\n') {
            self.ln += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        self.idx += 1;
    }
    pub fn advance_while<F: Fn(char) -> bool>(&mut self, f: &F) {
        while self.check(f) {
            self.advance();
        }
    }
    pub fn collect_while<F: Fn(char) -> bool>(&mut self, f: &F) -> Option<(String, Position)> {
        if !self.check(f) { return None; }
        let mut string = String::from(self.get()?);
        let mut pos = self.pos();
        self.advance();
        while self.check(f) {
            let Some(c) = self.get() else { break };
            string.push(c);
            pos.extend(&self.pos());
            self.advance();
        }
        Some((string, pos))
    }
    pub fn delimit(&mut self, start: char, end: char, escape: Option<char>) -> Option<Result<(String, Position), Located<String>>> {
        if !self.check(|c| c == start) { return None; }
        let mut string = String::new();
        let mut pos = self.pos();
        self.advance();
        while !self.check(|c| c == end) {
            let Some(c) = self.get() else { break };
            if let Some(escape) = escape {
                match c {
                    c if c == escape => {
                        string.push(c);
                        self.advance();
                        let Some(c) = self.get() else {
                            return Some(Err(Located::new("expected character, not end of input".into(), self.pos())))
                        };
                        string.push(c);
                    }
                    c => string.push(c)
                }
            } else {
                string.push(c);
            }
            pos.extend(&self.pos());
            self.advance();
        }
        if self.get().is_none() {
            return Some(Err(Located::new("unclosed string".into(), pos)))
        }
        self.advance();
        Some(Ok((string, pos)))
    }
    pub fn advance_ws(&mut self) {
        while self.check(|c| c.is_ascii_whitespace()) {
            self.advance();
        }
    }
    pub fn step(&mut self) -> Result<Option<Located<T>>, Located<String>> {
        self.advance_ws();
        if self.get().is_none() { return Ok(None) }
        T::step(self)
    }
    pub fn lex(mut self) -> Result<Vec<Located<T>>, Located<String>> {
        while let Some(token) = self.step()? {
            self.tokens.push(token);
        }
        Ok(self.tokens)
    }
}