use crate::error::CompilationError::SchemaError;
use crate::{
    compilation::{compile_validators, context::CompilationContext, JSONSchema},
    error::{error, no_error, ErrorIterator, ValidationError},
    keywords::{format_validators, CompilationResult, Validators},
    paths::InstancePath,
    validator::Validate,
};
use serde_json::{Map, Value};

pub(crate) struct ContainsValidator {
    validators: Validators,
}

impl ContainsValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value, context: &CompilationContext) -> CompilationResult {
        Ok(Box::new(ContainsValidator {
            validators: compile_validators(schema, context)?,
        }))
    }
}

impl Validate for ContainsValidator {
    fn is_valid(&self, schema: &JSONSchema, instance: &Value) -> bool {
        if let Value::Array(items) = instance {
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    fn validate<'a>(
        &self,
        schema: &'a JSONSchema,
        instance: &'a Value,
        instance_path: &InstancePath,
    ) -> ErrorIterator<'a> {
        if let Value::Array(items) = instance {
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    return no_error();
                }
            }
            error(ValidationError::contains(instance_path.into(), instance))
        } else {
            no_error()
        }
    }
}

impl ToString for ContainsValidator {
    fn to_string(&self) -> String {
        format!("contains: {}", format_validators(&self.validators))
    }
}

pub(crate) struct MinContainsValidator {
    validators: Validators,
    min_contains: u64,
}

impl MinContainsValidator {
    #[inline]
    pub(crate) fn compile(
        schema: &Value,
        context: &CompilationContext,
        min_contains: u64,
    ) -> CompilationResult {
        Ok(Box::new(MinContainsValidator {
            validators: compile_validators(schema, context)?,
            min_contains,
        }))
    }
}

impl Validate for MinContainsValidator {
    fn validate<'a>(
        &self,
        schema: &'a JSONSchema,
        instance: &'a Value,
        instance_path: &InstancePath,
    ) -> ErrorIterator<'a> {
        if let Value::Array(items) = instance {
            let mut matches = 0;
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    matches += 1;
                    if matches >= self.min_contains {
                        return no_error();
                    }
                }
            }
            if self.min_contains != 0 {
                error(ValidationError::contains(instance_path.into(), instance))
            } else {
                no_error()
            }
        } else {
            no_error()
        }
    }

    fn is_valid(&self, schema: &JSONSchema, instance: &Value) -> bool {
        if let Value::Array(items) = instance {
            let mut matches = 0;
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    matches += 1;
                    if matches >= self.min_contains {
                        return true;
                    }
                }
            }
            self.min_contains == 0
        } else {
            true
        }
    }
}

impl ToString for MinContainsValidator {
    fn to_string(&self) -> String {
        format!("contains: {}", format_validators(&self.validators))
    }
}
pub(crate) struct MaxContainsValidator {
    validators: Validators,
    max_contains: u64,
}

impl MaxContainsValidator {
    #[inline]
    pub(crate) fn compile(
        schema: &Value,
        context: &CompilationContext,
        max_contains: u64,
    ) -> CompilationResult {
        Ok(Box::new(MaxContainsValidator {
            validators: compile_validators(schema, context)?,
            max_contains,
        }))
    }
}

impl Validate for MaxContainsValidator {
    fn validate<'a>(
        &self,
        schema: &'a JSONSchema,
        instance: &'a Value,
        instance_path: &InstancePath,
    ) -> ErrorIterator<'a> {
        if let Value::Array(items) = instance {
            let mut matches = 0;
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    matches += 1;
                    if matches > self.max_contains {
                        return error(ValidationError::contains(instance_path.into(), instance));
                    }
                }
            }
            if matches == 0 {
                return error(ValidationError::contains(instance_path.into(), instance));
            }
            no_error()
        } else {
            no_error()
        }
    }

    fn is_valid(&self, schema: &JSONSchema, instance: &Value) -> bool {
        if let Value::Array(items) = instance {
            let mut matches = 0;
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    matches += 1;
                    if matches > self.max_contains {
                        return false;
                    }
                }
            }
            matches != 0
        } else {
            true
        }
    }
}

impl ToString for MaxContainsValidator {
    fn to_string(&self) -> String {
        format!("contains: {}", format_validators(&self.validators))
    }
}
pub(crate) struct MinMaxContainsValidator {
    validators: Validators,
    min_contains: u64,
    max_contains: u64,
}

impl MinMaxContainsValidator {
    #[inline]
    pub(crate) fn compile(
        schema: &Value,
        context: &CompilationContext,
        min_contains: u64,
        max_contains: u64,
    ) -> CompilationResult {
        Ok(Box::new(MinMaxContainsValidator {
            validators: compile_validators(schema, context)?,
            min_contains,
            max_contains,
        }))
    }
}

impl Validate for MinMaxContainsValidator {
    fn validate<'a>(
        &self,
        schema: &'a JSONSchema,
        instance: &'a Value,
        instance_path: &InstancePath,
    ) -> ErrorIterator<'a> {
        if let Value::Array(items) = instance {
            let mut matches = 0;
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    matches += 1;
                    if matches > self.max_contains {
                        return error(ValidationError::contains(instance_path.into(), instance));
                    }
                }
            }
            if matches > self.max_contains || matches < self.min_contains {
                error(ValidationError::contains(instance_path.into(), instance))
            } else {
                no_error()
            }
        } else {
            no_error()
        }
    }

    fn is_valid(&self, schema: &JSONSchema, instance: &Value) -> bool {
        if let Value::Array(items) = instance {
            let mut matches = 0;
            for item in items {
                if self
                    .validators
                    .iter()
                    .all(|validator| validator.is_valid(schema, item))
                {
                    matches += 1;
                    if matches > self.max_contains {
                        return false;
                    }
                }
            }
            matches <= self.max_contains && matches >= self.min_contains
        } else {
            true
        }
    }
}

impl ToString for MinMaxContainsValidator {
    fn to_string(&self) -> String {
        format!("contains: {}", format_validators(&self.validators))
    }
}

#[inline]
pub(crate) fn compile(
    parent: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    if let Some(min_contains) = parent.get("minContains") {
        if let Some(min_contains) = min_contains.as_u64() {
            if let Some(max_contains) = parent.get("maxContains") {
                if let Some(max_contains) = max_contains.as_u64() {
                    Some(MinMaxContainsValidator::compile(
                        schema,
                        context,
                        min_contains,
                        max_contains,
                    ))
                } else {
                    Some(Err(SchemaError))
                }
            } else {
                Some(MinContainsValidator::compile(schema, context, min_contains))
            }
        } else {
            Some(Err(SchemaError))
        }
    } else if let Some(max_contains) = parent.get("contains") {
        if let Some(max_contains) = max_contains.as_u64() {
            Some(MaxContainsValidator::compile(schema, context, max_contains))
        } else {
            Some(Err(SchemaError))
        }
    } else {
        Some(ContainsValidator::compile(schema, context))
    }
}
