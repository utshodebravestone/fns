use std::collections::HashMap;

use super::{builtin::get_builtin, types::Value};

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Box<Option<Self>>,
    pub variables: HashMap<String, (Value, bool)>,
}

impl Environment {
    pub fn new(parent: Option<Self>) -> Self {
        Self {
            parent: Box::new(parent),
            variables: get_builtin()
                .iter()
                .map(|(key, value)| (key.clone(), (value.clone(), true)))
                .collect(),
        }
    }

    pub fn define(&mut self, identifier: String, value: Value, is_constant: bool) {
        self.variables.insert(identifier, (value, is_constant));
    }

    pub fn is_constant(&self, identifier: &str) -> Option<bool> {
        if let Some((_, is_constant)) = self.variables.get(identifier) {
            Some(*is_constant)
        } else {
            match &*self.parent {
                Some(environment) => environment.is_constant(identifier),
                None => None,
            }
        }
    }

    pub fn access(&self, identifier: &str) -> Option<Value> {
        if let Some((value, _)) = self.variables.get(identifier) {
            Some(value.clone())
        } else {
            match &*self.parent {
                Some(environment) => environment.access(identifier),
                None => None,
            }
        }
    }
}
