use super::{CompilationResult, Validate, Validators};
use crate::compilation::CompilationContext;
use crate::compilation::{compile_validators, JSONSchema};
use crate::error::{no_error, CompilationError, ErrorIterator};
use serde_json::{Map, Value};

pub struct PropertiesValidator {
    properties: Vec<(String, Validators)>,
}

impl PropertiesValidator {
    pub(crate) fn compile(schema: &Value, context: &CompilationContext) -> CompilationResult {
        match schema {
            Value::Object(map) => {
                let mut properties = Vec::with_capacity(map.len());
                for (key, subschema) in map {
                    properties.push((key.clone(), compile_validators(subschema, context)?));
                }
                Ok(Box::new(PropertiesValidator { properties }))
            }
            _ => Err(CompilationError::SchemaError(String::from("properties/not-object"))),
        }
    }
}

impl Validate for PropertiesValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Object(item) = instance {
            let errors: Vec<_> = self
                .properties
                .iter()
                .flat_map(move |(name, validators)| {
                    let option = item.get(name);
                    option.into_iter().flat_map(move |item| {
                        validators
                            .iter()
                            .flat_map(move |validator| validator.validate(schema, item))
                    })
                })
                .collect();
            return Box::new(errors.into_iter());
        }
        no_error()
    }

    fn is_valid(&self, schema: &JSONSchema, instance: &Value) -> bool {
        if let Value::Object(item) = instance {
            return self.properties.iter().all(move |(name, validators)| {
                let option = item.get(name);
                option.into_iter().all(move |item| {
                    validators
                        .iter()
                        .all(move |validator| validator.is_valid(schema, item))
                })
            });
        }
        true
    }

    fn name(&self) -> String {
        format!("<properties: {:?}>", self.properties)
    }
}
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    Some(PropertiesValidator::compile(schema, context))
}
