use std::collections::HashMap;
use crate::structure::position::Located;

pub trait Evaluate<V, P> {
    fn evaluate(self, program: &mut P) -> Result<Option<V>, Located<String>>;
}