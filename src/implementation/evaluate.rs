use std::collections::HashMap;
use crate::{
    tools::{
        parse::Parsable,
        evaluate::Evaluate
    },
    structure::{
        position::Located,
        program::Program,
        ast::*,
        value::*
    }
};

pub fn evaluate<N: Evaluate<Value, Program>>(ast: N, program: &mut Program) -> Result<Option<Value>, Located<String>> {
    ast.evaluate(program)
}

impl Evaluate<Value, Program> for Located<Atom> {
    fn evaluate(self, program: &mut Program) -> Result<Option<Value>, Located<String>> {
        let Located { value: atom, pos } = self;
        match atom {
            Atom::Int(v) => Ok(Some(Value::Int(v))),
            Atom::Float(v) => Ok(Some(Value::Float(v))),
            Atom::Boolean(v) => Ok(Some(Value::Boolean(v))),
            Atom::None => Ok(None),
            Atom::String(v) => Ok(Some(Value::String(v))),
            Atom::Ident(ident) => if let Some(value) = program.vars.get(&ident) {
                Ok(Some(value.clone()))
            } else {
                Err(Located::new(format!("no variable with the name {ident:?} found"), pos))
            }
            Atom::Expression(expr) => expr.evaluate(program),
            Atom::Vector(exprs) => {
                let mut values = vec![];
                for expr in exprs {
                    let expr_pos = expr.pos.clone();
                    if let Some(value) = expr.evaluate(program)? {
                        values.push(value);
                    } else {
                        return Err(Located::new("return value is none".into(), expr_pos))
                    }
                }
                Ok(Some(Value::Vector(values)))
            }
            Atom::Field { head, field } => {
                let head_pos = head.pos.clone();
                let Some(head) = head.evaluate(program)? else {
                    return Err(Located::new("return value is none".into(), head_pos))
                };
                match head {
                    Value::Object(addr) => {
                        let obj = &program.objects.get(addr).unwrap();
                        if let Some(value) = obj.map.get(&field.value) {
                            Ok(Some(value.clone()))
                        } else {
                            Err(Located::new(format!("no field named {:?}", field.value), field.pos))
                        }
                    }
                    head => Err(Located::new(format!("cannot get field of {}", Type::from(head)), pos))
                }
            }
            Atom::Index { head, field } => {
                let head_pos = head.pos.clone();
                let Some(head) = head.evaluate(program)? else {
                    return Err(Located::new("return value is none".into(), head_pos))
                };
                let field_pos = field.pos.clone();
                let Some(field) = field.evaluate(program)? else {
                    return Err(Located::new("return value is none".into(), field_pos))
                };
                match head {
                    Value::Vector(values) => match field {
                        Value::Int(index) => {
                            let idx = if index >= 0 {
                                index as usize
                            } else {
                                return Err(Located::new(format!("invalid index: {index}"), field_pos));
                            };
                            Ok(values.get(idx).cloned())
                        }
                        field => Err(Located::new(format!("cannot index with {}", Type::from(field)), pos))
                    }
                    head => Err(Located::new(format!("cannot index {}", Type::from(head)), pos))
                }
            }
        }
    }
}

impl Evaluate<Value, Program> for Located<Expression> {
    fn evaluate(self, program: &mut Program) -> Result<Option<Value>, Located<String>> {
        let Located { value: expr, pos } = self;
        match expr {
            Expression::Atom(atom) => Located::new(atom, pos).evaluate(program),
            Expression::Binary { op, left, right } => {
                let left_pos = left.pos.clone();
                let Some(left) = left.evaluate(program)? else {
                    return Err(Located::new("return value is none".into(), left_pos))
                };
                let right_pos = right.pos.clone();
                let Some(right) = right.evaluate(program)? else {
                    return Err(Located::new("return value is none".into(), right_pos))
                };
                match op {
                    BinaryOperator::Add => match (left, right) {
                        (Value::Int(v1), Value::Int(v2)) => Ok(Some(Value::Int(v1 + v2))),
                        (Value::Float(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 + v2))),
                        (Value::Int(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 as f64 + v2))),
                        (Value::Float(v1), Value::Int(v2)) => Ok(Some(Value::Float(v1 + v2 as f64))),
                        (left, right) => Err(Located::new(format!("cannot perform binary operator {op:?} on {} with {}", Type::from(left), Type::from(right)), pos))
                    }
                    BinaryOperator::Sub => match (left, right) {
                        (Value::Int(v1), Value::Int(v2)) => Ok(Some(Value::Int(v1 - v2))),
                        (Value::Float(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 - v2))),
                        (Value::Int(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 as f64 - v2))),
                        (Value::Float(v1), Value::Int(v2)) => Ok(Some(Value::Float(v1 - v2 as f64))),
                        (left, right) => Err(Located::new(format!("cannot perform binary operator {op:?} on {} with {}", Type::from(left), Type::from(right)), pos))
                    }
                    BinaryOperator::Mul => match (left, right) {
                        (Value::Int(v1), Value::Int(v2)) => Ok(Some(Value::Int(v1 * v2))),
                        (Value::Float(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 * v2))),
                        (Value::Int(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 as f64 * v2))),
                        (Value::Float(v1), Value::Int(v2)) => Ok(Some(Value::Float(v1 * v2 as f64))),
                        (left, right) => Err(Located::new(format!("cannot perform binary operator {op:?} on {} with {}", Type::from(left), Type::from(right)), pos))
                    }
                    BinaryOperator::Div => match (left, right) {
                        (Value::Int(v1), Value::Int(v2)) => Ok(Some(Value::Float(v1 as f64 / v2 as f64))),
                        (Value::Float(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 / v2))),
                        (Value::Int(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 as f64 / v2))),
                        (Value::Float(v1), Value::Int(v2)) => Ok(Some(Value::Float(v1 / v2 as f64))),
                        (left, right) => Err(Located::new(format!("cannot perform binary operator {op:?} on {} with {}", Type::from(left), Type::from(right)), pos))
                    }
                    BinaryOperator::Pow => match (left, right) {
                        (Value::Int(v1), Value::Int(v2)) => Ok(Some(Value::Float((v1 as f64).powf(v2 as f64)))),
                        (Value::Float(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1.powf(v2)))),
                        (Value::Int(v1), Value::Float(v2)) => Ok(Some(Value::Float((v1 as f64).powf(v2)))),
                        (Value::Float(v1), Value::Int(v2)) => Ok(Some(Value::Float(v1.powf(v2 as f64)))),
                        (left, right) => Err(Located::new(format!("cannot perform binary operator {op:?} on {} with {}", Type::from(left), Type::from(right)), pos))
                    }
                    BinaryOperator::Mod => match (left, right) {
                        (Value::Int(v1), Value::Int(v2)) => Ok(Some(Value::Int(v1 % v2))),
                        (Value::Float(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 % v2))),
                        (Value::Int(v1), Value::Float(v2)) => Ok(Some(Value::Float(v1 as f64 % v2))),
                        (Value::Float(v1), Value::Int(v2)) => Ok(Some(Value::Float(v1 % v2 as f64))),
                        (left, right) => Err(Located::new(format!("cannot perform binary operator {op:?} on {} with {}", Type::from(left), Type::from(right)), pos))
                    }
                }
            }
            Expression::Unary { op, right } => {
                let right_pos = right.pos.clone();
                let Some(right) = right.evaluate(program)? else {
                    return Err(Located::new("return value is none".into(), right_pos))
                };
                match op {
                    UnaryOperator::Neg => match right {
                        Value::Int(v) => Ok(Some(Value::Int(-v))),
                        Value::Float(v) => Ok(Some(Value::Float(-v))),
                        right => Err(Located::new(format!("cannot perform unary operator {op:?} on {}", Type::from(right)), pos))
                    }
                    UnaryOperator::Not => match right {
                        Value::Boolean(v) => Ok(Some(Value::Boolean(!v))),
                        right => Err(Located::new(format!("cannot perform unary operator {op:?} on {}", Type::from(right)), pos))
                    }
                }
            }
            Expression::Call { head, args } => {
                let head_pos = head.pos.clone();
                let Some(head) = head.evaluate(program)? else {
                    return Err(Located::new("return value is none".into(), head_pos))
                };
                let mut value_args = vec![];
                for arg in args.value.0 {
                    let arg_pos = arg.pos.clone();
                    if let Some(arg) = arg.evaluate(program)? {
                        value_args.push(arg);
                    } else {
                        return Err(Located::new("return value is none".into(), arg_pos))
                    }
                }
                match head {
                    Value::Function(addr) => {
                        let native_fn = program.native_fns.get(addr).unwrap();
                        native_fn(value_args, program).map_err(|err| Located::new(err, pos))
                    }
                    head => Err(Located::new(format!("cannot call {}", Type::from(head)), pos))
                }
            }
        }
    }
}