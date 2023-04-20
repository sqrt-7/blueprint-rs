use crate::logic::domain;
use std::{error::Error, fmt::Display};

pub mod inmem;

// INTERFACE --------------

pub trait Datastore: Send + Sync {
    fn store_user(&self, obj: &domain::User) -> Result<(), DatastoreError>;
    fn get_user(&self, uuid: &str) -> Result<domain::User, DatastoreError>;
    fn store_subscription(&self, sub: &domain::Subscription) -> Result<(), DatastoreError>;
    fn list_subscriptions_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<domain::Subscription>, DatastoreError>;
}

// ERRORS -----------------

#[derive(Debug)]
pub struct DatastoreError {
    pub msg: String,
    pub error_type: DatastoreErrorType,
}

#[derive(Debug)]
pub enum DatastoreErrorType {
    NotFound,
    DataCorruption,
    Other,
}

impl DatastoreError {
    pub fn new(msg: String, error_type: DatastoreErrorType) -> Self {
        DatastoreError { msg, error_type }
    }
}

impl Display for DatastoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DatastoreError (msg: {}, error_type: {})",
            self.msg, self.error_type
        )
    }
}

impl Display for DatastoreErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DatastoreError {}
