use crate::logic::domain;
use std::{error::Error, fmt::Display};

pub mod inmem;
pub mod sql;

// INTERFACE --------------

pub trait Datastore {
    fn store_user(&self, usr: &domain::User) -> Result<(), DatastoreError>;
    fn get_user(&self, id: &domain::ID) -> Result<domain::User, DatastoreError>;
    fn store_journal(&self, j: &domain::Journal) -> Result<(), DatastoreError>;
    fn get_journal(&self, id: &domain::ID) -> Result<domain::Journal, DatastoreError>;
    fn store_subscription(&self, sub: &domain::Subscription) -> Result<(), DatastoreError>;
    fn list_subscriptions_by_user(
        &self,
        user_id: &domain::ID,
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
        DatastoreError {
            msg,
            error_type,
        }
    }
}

impl Display for DatastoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "DatastoreError (msg: {}, error_type: {})",
            self.msg, self.error_type
        ))
    }
}

impl Display for DatastoreErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DatastoreError {}
