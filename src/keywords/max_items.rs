use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    validator::Validate,
};
use serde_json::{Map, Value};

pub struct MaxItemsValidator {
    limit: u64,
}

impl MaxItemsValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Some(limit) = schema.as_u64() {
            return Ok(Box::new(MaxItemsValidator { limit }));
        }
        Err(CompilationError::SchemaError)
    }
}

impl Validate for MaxItemsValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Array(items) = instance {
            if (items.len() as u64) > self.limit {
                return error(ValidationError::max_items(instance, self.limit));
            }
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Array(items) = instance {
            if (items.len() as u64) > self.limit {
                return false;
            }
        }
        true
    }

    fn name(&self) -> String {
        format!("maxItems: {}", self.limit)
    }
}

#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(MaxItemsValidator::compile(schema))
}
