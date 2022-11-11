use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

pub mod inmem;

pub trait Datastore: Send + Sync {
    fn store_subscription(&self, sub: &Subscription) -> Result<(), Box<dyn Error>>;
    fn get_subscription(&self, uuid: String) -> Result<Subscription, Box<dyn Error>>;
}

// DTOs -------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub uuid: String,
    pub name: String,
    pub email: String,
}

// ERRORS -----------------

pub type NotFoundError = DatastoreError;
pub type InternalError = DatastoreError;

#[derive(Debug)]
pub struct DatastoreError {
    pub msg: String,
}

impl DatastoreError {
    pub fn new_box(msg: String) -> Box<Self> {
        Box::new(DatastoreError { msg })
    }
}

impl Display for DatastoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DatastoreError ({})", self.msg)
    }
}

impl Error for DatastoreError {}
