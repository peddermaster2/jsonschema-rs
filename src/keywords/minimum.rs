use super::{CompilationResult, Validate};
use crate::compilation::{CompilationContext, JSONSchema};
use crate::error::{no_error, CompilationError, ErrorIterator, ValidationError};
use serde_json::{Map, Value};

pub struct MinimumValidator {
    limit: f64,
}

impl MinimumValidator {
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Number(limit) = schema {
            let limit = limit.as_f64().unwrap();
            return Ok(Box::new(MinimumValidator { limit }));
        }
        Err(CompilationError::SchemaError(String::from("minimum/not-number")))
    }
}

impl Validate for MinimumValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Number(item) = instance {
            let item = item.as_f64().unwrap();
            if item < self.limit {
                return ValidationError::minimum(item, self.limit);
            }
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(item) = instance {
            let item = item.as_f64().unwrap();
            if item < self.limit {
                return false;
            }
        }
        true
    }

    fn name(&self) -> String {
        format!("<minimum: {}>", self.limit)
    }
}

pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(MinimumValidator::compile(schema))
}
