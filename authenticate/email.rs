use super::prelude::*;
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Email(pub String);

#[Scalar(name = "Email")]
impl ScalarType for Email {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(mut s) => {
                s = s.trim().to_lowercase();
                if ValidateEmail::validate_email(&s) {
                    Ok(Email(s.clone()))
                } else {
                    Err(InputValueError::custom("Invalid email"))
                }
            }
            v => Err(InputValueError::expected_type(v)),
        }
    }
    fn to_value(&self) -> Value {
        Value::String(self.0.clone())
    }
}
