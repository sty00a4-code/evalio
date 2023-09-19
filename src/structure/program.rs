use std::{collections::HashMap, fmt::Debug};

use crate::structure::value::{
    Value,
    Object
};
use super::{position::Located, value::Type};

pub type NativeFunction = fn(Vec<Value>, &mut Program) -> Result<Option<Value>, String>;

pub struct Map<T>(Vec<T>);
impl<T> Map<T> {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn create(&mut self, value: T) -> usize {
        let addr = self.0.len();
        self.0.push(value);
        addr
    }
    pub fn get(&mut self, addr: usize) -> Option<&T> {
        self.0.get(addr)
    }
    pub fn get_mut(&mut self, addr: usize) -> Option<&mut T> {
        self.0.get_mut(addr)
    }
}
impl<T: Debug> Debug for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T: Clone> Clone for Map<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T> Default for Map<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub vars: HashMap<String, Value>,
    pub objects: Map<Object>,
    pub native_fns: Map<NativeFunction>,
}
impl Program {
    pub fn set(&mut self, ident: &str, value: Value) -> Option<Value> {
        self.vars.insert(ident.to_string(), value)
    }
    pub fn new_object(&mut self, ident: &str, value: Object) -> Option<Value> {
        let addr = self.objects.create(value);
        self.set(ident, Value::Object(addr))
    }
    pub fn new_fn(&mut self, ident: &str, value: NativeFunction) -> Option<Value> {
        let addr = self.native_fns.create(value);
        self.set(ident, Value::Function(addr))
    }
    pub fn init() -> Self {
        let mut program = Self::default();
            program.new_fn("exit", _exit);
            program.new_fn("set", _set);
            program.new_fn("abs", _abs);
        program
    }
}

pub fn _exit(_: Vec<Value>, _: &mut Program) -> Result<Option<Value>, String> {
    panic!("exit")
}
pub fn _set(mut args: Vec<Value>, program: &mut Program) -> Result<Option<Value>, String> {
    if args.len() >= 2 {
        let ident = args.remove(0); let Value::String(ident) = ident else {
            return Err(format!("expected string for argument #1, got {}", Type::from(ident)))
        };
        let value = args.remove(0);
        program.set(&ident, value);
    }
    Ok(None)
}
pub fn _abs(mut args: Vec<Value>, _: &mut Program) -> Result<Option<Value>, String> {
    if args.is_empty() {
        return Ok(None)
    }
    let value = args.remove(0);
    match value {
        Value::Int(v) => Ok(Some(Value::Int(v.abs()))),
        Value::Float(v) => Ok(Some(Value::Float(v.abs()))),
        value => Ok(Some(value))
    }
}