use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use crate::domain;

use super::{Datastore, DatastoreError, DatastoreErrorType};

pub struct InMemDatastore {
    items: Mutex<HashMap<String, String>>, // <uuid, json>
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            items: Mutex::new(HashMap::new()),
        }
    }

    fn to_json<T>(item: T) -> Result<String, DatastoreError>
    where
        T: serde::Serialize,
    {
        match serde_json::to_string(&item) {
            Ok(v) => Ok(v),
            Err(e) => Err(DatastoreError::new(
                format!("InMemDatastore json error: {}", e),
                DatastoreErrorType::Other,
            )),
        }
    }

    fn from_json<'a, T>(js: &'a str) -> Result<T, DatastoreError>
    where
        T: serde::Deserialize<'a>,
    {
        match serde_json::from_str::<T>(js) {
            Ok(v) => Ok(v),
            Err(e) => Err(DatastoreError::new(
                format!("InMemDatastore json error: {}", e),
                DatastoreErrorType::Other,
            )),
        }
    }

    fn lock(&self) -> Result<MutexGuard<HashMap<String, String>>, DatastoreError> {
        match self.items.lock() {
            Ok(v) => Ok(v),
            Err(e) => Err(DatastoreError::new(
                format!("InMemDatastore lock error: {}", e),
                DatastoreErrorType::Other,
            )),
        }
    }
}

impl Default for InMemDatastore {
    fn default() -> Self {
        Self::new()
    }
}

impl Datastore for InMemDatastore {
    fn store_subscription(&self, sub: &domain::Subscription) -> Result<(), DatastoreError> {
        let item = DBSubscription::from_domain(sub);
        let data = InMemDatastore::to_json(&item)?;

        self.lock()?.insert(item.uuid, data);

        Ok(())
    }

    fn get_subscription(&self, uuid: &str) -> Result<domain::Subscription, DatastoreError> {
        match self.lock()?.get(uuid) {
            Some(data) => {
                let item = InMemDatastore::from_json::<DBSubscription>(data)?;
                Ok(item.to_domain())
            }

            None => Err(DatastoreError::new(
                format!("uuid: {}", uuid),
                DatastoreErrorType::NotFound,
            )),
        }
    }
}

// DTOs -------------------

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct DBSubscription {
    pub uuid: String,
    pub name: String,
    pub email: String,
}

impl DBSubscription {
    fn from_domain(inp: &domain::Subscription) -> DBSubscription {
        DBSubscription {
            uuid: inp.uuid().to_owned(),
            name: inp.name().to_owned(),
            email: inp.email().to_owned(),
        }
    }

    fn to_domain(self) -> domain::Subscription {
        domain::Subscription::new(self.uuid, self.email, self.name)
    }
}
