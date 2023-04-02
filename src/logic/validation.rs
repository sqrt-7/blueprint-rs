use super::{
    domain::Subscription,
    error::{ServiceError, ServiceErrorType, CODE_SUB_INVALID_DATA, CODE_UNEXPECTED_ERROR},
};
use crate::datastore::Datastore;
use std::sync::Arc;

pub struct Validator {
    ds: Arc<dyn Datastore>,
}

pub trait ValidateDomain<D> {
    fn validate(&self, obj: D) -> Result<(), ServiceError>;
}

impl Validator {
    pub fn new(ds: Arc<dyn Datastore>) -> Self {
        Validator { ds }
    }
}

impl ValidateDomain<&Subscription> for Validator {
    fn validate(&self, obj: &Subscription) -> Result<(), ServiceError> {
        if obj.uuid() == "" {
            return Err(ServiceError::new(CODE_UNEXPECTED_ERROR)
                .with_type(ServiceErrorType::Internal)
                .with_internal("subscription.uuid field empty".to_owned()));
        }

        if obj.email() == "" {
            return Err(ServiceError::new(CODE_SUB_INVALID_DATA)
                .with_type(ServiceErrorType::Validation)
                .with_internal("subscription.email field empty".to_owned()));
        }

        // todo

        Ok(())
    }
}
