use std::{ops::Range, fmt::{Debug, Display}};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    pub ln: Range<usize>,
    pub col: Range<usize>,
}
impl Position {
    pub fn new(ln: Range<usize>, col: Range<usize>) -> Self {
        Self { ln, col }
    }
    pub fn between(start: &Self, end: &Self) -> Self {
        Self { ln: start.ln.start..end.ln.end, col: start.col.start..end.col.end }
    }
    pub fn extend(&mut self, other: &Self) {
        self.ln.end = other.ln.end;
        self.col.end = other.col.end;
    }
}

pub struct Located<T> {
    pub value: T,
    pub pos: Position
}
impl<T> Located<T> {
    pub fn new(value: T, pos: Position) -> Self {
        Self { value, pos }
    }
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Located<U> {
        let Self { value, pos } = self;
        Located { value: f(value), pos }
    }
}
impl<T: Debug> Debug for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Display> Display for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Default> Default for Located<T> {
    fn default() -> Self {
        Self { value: T::default(), pos: Position::default() }
    }
}
impl<T: Clone> Clone for Located<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), pos: self.pos.clone() }
    }
}
impl<T: PartialEq> PartialEq for Located<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}