use std::fmt;

use crate::frontend::ast::Number;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(Number),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::None => write!(f, "none"),
        }
    }
}
