use super::error::ServiceError;

pub trait Validation {
    fn validate(&self) -> ServiceError;
}

pub struct Validator {}
