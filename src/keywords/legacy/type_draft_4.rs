use super::super::{type_, CompilationResult, Validate};
use crate::compilation::{CompilationContext, JSONSchema};
use crate::error::{no_error, CompilationError, ErrorIterator, PrimitiveType, ValidationError};
use serde_json::{Map, Number, Value};

pub struct MultipleTypesValidator {
    types: Vec<PrimitiveType>,
}

impl MultipleTypesValidator {
    pub(crate) fn compile(items: &[Value]) -> CompilationResult {
        let mut types = Vec::with_capacity(items.len());
        for item in items {
            match item {
                Value::String(string) => match string.as_str() {
                    "integer" => types.push(PrimitiveType::Integer),
                    "null" => types.push(PrimitiveType::Null),
                    "boolean" => types.push(PrimitiveType::Boolean),
                    "string" => types.push(PrimitiveType::String),
                    "array" => types.push(PrimitiveType::Array),
                    "object" => types.push(PrimitiveType::Object),
                    "number" => types.push(PrimitiveType::Number),
                    _ => return Err(CompilationError::SchemaError(String::from("type_draft4/item/unknown-type"))),
                },
                _ => return Err(CompilationError::SchemaError(String::from("type_draft4/item/not-string"))),
            }
        }
        Ok(Box::new(MultipleTypesValidator { types }))
    }
}

impl Validate for MultipleTypesValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if !self.is_valid(schema, instance) {
            return ValidationError::multiple_type_error(instance.clone(), self.types.clone());
        }
        no_error()
    }
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        for type_ in self.types.iter() {
            match (type_, instance) {
                (PrimitiveType::Integer, Value::Number(num)) if is_integer(num) => return true,
                (PrimitiveType::Null, Value::Null)
                | (PrimitiveType::Boolean, Value::Bool(_))
                | (PrimitiveType::String, Value::String(_))
                | (PrimitiveType::Array, Value::Array(_))
                | (PrimitiveType::Object, Value::Object(_))
                | (PrimitiveType::Number, Value::Number(_)) => return true,
                (_, _) => continue,
            };
        }
        false
    }

    fn name(&self) -> String {
        format!("<type: {:?}>", self.types)
    }
}

pub struct IntegerTypeValidator {}

impl IntegerTypeValidator {
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(IntegerTypeValidator {}))
    }
}

impl Validate for IntegerTypeValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if !self.is_valid(schema, instance) {
            return ValidationError::single_type_error(instance.clone(), PrimitiveType::Integer);
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(num) = instance {
            return is_integer(num);
        }
        false
    }

    fn name(&self) -> String {
        "<type: integer>".to_string()
    }
}

fn is_integer(num: &Number) -> bool {
    num.is_u64() || num.is_i64()
}

pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    match schema {
        Value::String(item) => compile_single_type(item.as_str()),
        Value::Array(items) => {
            if items.len() == 1 {
                if let Some(Value::String(item)) = items.iter().next() {
                    compile_single_type(item.as_str())
                } else {
                    Some(Err(CompilationError::SchemaError(String::from("type_draft4/item/not-string"))))
                }
            } else {
                Some(MultipleTypesValidator::compile(items))
            }
        }
        _ => Some(Err(CompilationError::SchemaError(String::from("type_draft4/not-array-or-string")))),
    }
}

fn compile_single_type(item: &str) -> Option<CompilationResult> {
    match item {
        "integer" => Some(IntegerTypeValidator::compile()),
        "null" => Some(type_::NullTypeValidator::compile()),
        "boolean" => Some(type_::BooleanTypeValidator::compile()),
        "string" => Some(type_::StringTypeValidator::compile()),
        "array" => Some(type_::ArrayTypeValidator::compile()),
        "object" => Some(type_::ObjectTypeValidator::compile()),
        "number" => Some(type_::NumberTypeValidator::compile()),
        _ => Some(Err(CompilationError::SchemaError(String::from("type_draft4/unknown-type")))),
    }
}
