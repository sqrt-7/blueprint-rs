use crate::logic::domain;
use std::{error::Error, fmt::Display};

pub mod inmem;
pub mod sql;

pub type DataResult<T> = std::result::Result<T, DatastoreError>;

// INTERFACE --------------

#[tonic::async_trait]
pub trait Datastore {
    async fn store_user(&self, usr: &domain::User) -> DataResult<()>;
    async fn get_user(&self, id: &domain::ID) -> DataResult<domain::User>;
    async fn store_journal(&self, j: &domain::Journal) -> DataResult<()>;
    async fn get_journal(&self, id: &domain::ID) -> DataResult<domain::Journal>;
    async fn store_subscription(&self, sub: &domain::Subscription) -> DataResult<()>;
    async fn list_subscriptions_by_user(
        &self, user_id: &domain::ID,
    ) -> DataResult<Vec<domain::Subscription>>;
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
    Duplicate,
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
