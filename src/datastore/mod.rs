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
    async fn list_users(&self) -> DataResult<Vec<domain::User>>;
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
    Conflict,
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
