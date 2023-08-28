use std::collections::HashMap;

use super::types::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Box<Option<Self>>,
    pub variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new(parent: Option<Self>) -> Self {
        Self {
            parent: Box::new(parent),
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, identifier: String, value: Value) {
        self.variables.insert(identifier, value);
    }

    pub fn access(&self, identifier: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(identifier) {
            Some(value.clone())
        } else {
            match &*self.parent {
                Some(environment) => environment.access(identifier),
                None => None,
            }
        }
    }
}
