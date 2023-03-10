use std::{error::Error, fmt::Display};

use crate::domain;

pub mod inmem;

pub trait Datastore: Send + Sync {
    fn store_subscription(&self, sub: &domain::Subscription) -> Result<(), Box<dyn Error>>;
    fn get_subscription(&self, uuid: String) -> Result<domain::Subscription, Box<dyn Error>>;
}

// DTOs -------------------

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DBSubscription {
    pub uuid: String,
    pub name: String,
    pub email: String,
}

impl DBSubscription {
    pub fn from_domain(inp: &domain::Subscription) -> DBSubscription {
        DBSubscription {
            uuid: inp.uuid().to_owned(),
            name: inp.name().to_owned(),
            email: inp.email().to_owned(),
        }
    }

    pub fn to_domain(self) -> domain::Subscription {
        domain::Subscription::new(self.uuid, self.email, self.name)
    }
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
