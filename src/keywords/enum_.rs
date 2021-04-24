use super::{helpers, CompilationResult, Validate};
use crate::compilation::{CompilationContext, JSONSchema};
use crate::error::{no_error, CompilationError, ErrorIterator, ValidationError};
use serde_json::{Map, Value};

pub struct EnumValidator {
    options: Value,
    items: Vec<Value>,
}

impl EnumValidator {
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Array(items) = schema {
            return Ok(Box::new(EnumValidator {
                options: schema.clone(),
                items: items.clone(),
            }));
        }
        Err(CompilationError::SchemaError(String::from("enum/not-array")))
    }
}

impl Validate for EnumValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if !self.is_valid(schema, instance) {
            return ValidationError::enumeration(instance.clone(), self.options.clone());
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        self.items.iter().any(|item| helpers::equal(instance, item))
    }

    fn name(&self) -> String {
        format!("<enum: {:?}>", self.items)
    }
}

pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(EnumValidator::compile(schema))
}
