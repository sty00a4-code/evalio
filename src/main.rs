#![allow(unused)]
use std::{
    io::{
        prelude::*,
        stdout,
        stdin
    },
    collections::HashMap
};

use structure::value::{
    Object,
    Value
};
use structure::program::Program;

use crate::{
    implementation::{
        evaluate::evaluate,
        lex::lex,
        parse::parse
    },
    structure::tokens::Token
};
use crate::structure::{
    position::Located,
    ast::Expression
};

pub mod structure;
pub mod tools;
pub mod implementation;

pub fn eval(input: &str, program: &mut Program) -> Result<Option<Value>, Located<String>> {
    let tokens = lex(input.to_string())?;
    let ast = parse::<Token, Expression>(tokens)?;
    evaluate(ast, program)
}

fn main() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook(Box::new(|_info| {}));
    std::panic::catch_unwind(|| {
        let mut program = Program::init();
        let args_string = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
        if !args_string.is_empty() {
            match eval(&args_string, &mut program) {
                Ok(Some(value)) => println!("{value}"),
                Err(Located { value: err, pos }) => println!("ERROR: {err}"),
                _ => {}
            }
            return
        }
        loop {
            let mut input = String::new();
            print!("> ");
            stdout().flush();
            stdin().read_line(&mut input);
            match eval(&input, &mut program) {
                Ok(Some(value)) => println!("{value}"),
                Err(Located { value: err, pos }) => println!("ERROR: {err}"),
                _ => {}
            }
        }
    });
}
