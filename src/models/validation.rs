/*! 字段参数验证模块
 *
 */
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket::serde::json::{json, Json};
use serde::Serialize;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::models::response::APIResponse;

#[derive(Debug, Serialize)]
pub struct Errors {
    errors: ValidationErrors,
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

impl Errors {
    #[allow(dead_code)]
    pub fn new(errs: &[(FieldName, FieldErrorCode)]) -> Self {
        let mut errors = ValidationErrors::new();
        for (field, code) in errs {
            errors.add(field, ValidationError::new(code));
        }
        Self { errors }
    }
}

impl<'r> Responder<'r, 'static> for Errors {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        use validator::ValidationErrorsKind::Field;

        let mut errors = json!({});
        for (field, field_errors) in self.errors.into_errors() {
            if let Field(field_errors) = field_errors {
                errors[field] = field_errors
                    .into_iter()
                    .map(|field_error| field_error.code)
                    .collect();
            }
        }

        status::Custom(
            Status::UnprocessableEntity,
            Json(json!({ "errors": errors })),
        )
        .respond_to(req)
    }
}

pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), APIResponse> {
        if self.errors.is_empty() {
            return Ok(());
        }

        let err_string = serde_json::to_string(&Errors {
            errors: self.errors,
        })
        .map_or("".to_string(), |v| v);
        Err(APIResponse::build().code(0).msg(&err_string))
    }

    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
    where
        T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be blank"));
            T::default()
        })
    }
}
