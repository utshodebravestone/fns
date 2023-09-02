use std::{collections::HashMap, fmt};

use crate::frontend::ast::Number;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Object(HashMap<String, Box<Value>>),
    String(String),
    Number(Number),
    Boolean(bool),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Object(o) => {
                if o.is_empty() {
                    write!(f, "{{}}")
                } else {
                    writeln!(f, "{{")?;
                    for (key, value) in o.iter() {
                        writeln!(f, "  {key} : {value}")?;
                    }
                    write!(f, "}}")
                }
            }
            Value::String(s) => write!(f, "{s}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::None => write!(f, "none"),
        }
    }
}
