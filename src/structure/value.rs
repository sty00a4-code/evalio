use std::{
    fmt::{
        Display,
        Debug
    },
    collections::HashMap
};

use crate::structure::{
    position::Located,
    program::Program
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int, Float, Boolean, String, Vector, Object, Function
}
impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::Boolean(_) => Self::Boolean,
            Value::String(_) => Self::String,
            Value::Vector(_) => Self::Vector,
            Value::Function(_) => Self::Function,
            Value::Object(_) => Self::Object,
        }
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "boolean"),
            Type::String => write!(f, "string"),
            Type::Vector => write!(f, "vector"),
            Type::Object => write!(f, "object"),
            Type::Function => write!(f, "function"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Object {
    pub map: HashMap<String, Value>,
    pub meta: HashMap<String, Value>,
}
impl Object {
    pub fn insert(mut self, ident: &str, value: Value) -> Self {
        self.map.insert(ident.to_string(), value);
        self
    }
    pub fn set(&mut self, ident: &str, value: Value) -> Option<Value> {
        self.map.insert(ident.to_string(), value)
    }
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Int(i64), Float(f64),
    Boolean(bool), String(String),
    Vector(Vec<Self>),
    Object(usize),
    Function(usize)
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::Boolean(v) => write!(f, "{v}"),
            Value::String(v) => write!(f, "{v}"),
            Value::Vector(v) => write!(f, "[{}]", v.iter().map(|x| format!("{x:?}")).collect::<Vec<String>>().join(", ")),
            Value::Function(v) => write!(f, "function:{:8x?}", v as *const usize),
            Value::Object(v) => write!(f, "object:{:8x?}", v as *const usize),
        }
    }
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{v:?}"),
            Value::Float(v) => write!(f, "{v:?}"),
            Value::Boolean(v) => write!(f, "{v:?}"),
            Value::String(v) => write!(f, "{v:?}"),
            Value::Vector(v) => write!(f, "[{}]", v.iter().map(|x| format!("{x:?}")).collect::<Vec<String>>().join(", ")),
            Value::Function(v) => write!(f, "function:{:8x?}", v as *const usize),
            Value::Object(v) => write!(f, "object:{:8x?}", v as *const usize),
        }
    }
}