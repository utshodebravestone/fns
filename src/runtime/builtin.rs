use std::f64::consts::{E, PI};

use super::types::Value;

pub fn get_builtin() -> Vec<(String, Value)> {
    vec![
        (
            "fns".to_string(),
            Value::Object(
                vec![("version", Value::String("0.0.1".to_string()))]
                    .iter()
                    .map(|(key, value)| (key.to_string(), Box::new(value.clone())))
                    .collect(),
            ),
        ),
        (
            "math".to_string(),
            Value::Object(
                vec![("pi", Value::Number(PI)), ("e", Value::Number(E))]
                    .iter()
                    .map(|(key, value)| (key.to_string(), Box::new(value.clone())))
                    .collect(),
            ),
        ),
    ]
}
