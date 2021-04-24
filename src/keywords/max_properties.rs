use super::{CompilationResult, Validate};
use crate::compilation::{CompilationContext, JSONSchema};
use crate::error::{no_error, CompilationError, ErrorIterator, ValidationError};
use serde_json::{Map, Value};

pub struct MaxPropertiesValidator {
    limit: usize,
}

impl<'a> MaxPropertiesValidator {
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Number(limit) = schema {
            let limit = limit.as_u64().unwrap() as usize;
            return Ok(Box::new(MaxPropertiesValidator { limit }));
        }
        Err(CompilationError::SchemaError(String::from("max properties/not-number")))
    }
}

impl Validate for MaxPropertiesValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Object(item) = instance {
            if item.len() > self.limit {
                return ValidationError::max_properties(instance.clone());
            }
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Object(item) = instance {
            if item.len() > self.limit {
                return false;
            }
        }
        true
    }

    fn name(&self) -> String {
        format!("<max properties: {}>", self.limit)
    }
}
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(MaxPropertiesValidator::compile(schema))
}
