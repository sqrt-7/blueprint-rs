use crate::logic::domain;
use opentelemetry::Key;
use std::{
    collections::HashMap,
    result,
    sync::{Mutex, MutexGuard},
};

use super::{Datastore, DatastoreError, DatastoreErrorType};

type Result<T> = result::Result<T, DatastoreError>;

pub struct InMemDatastore {
    items: Mutex<HashMap<String, String>>, // <uuid, json>
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            items: Mutex::new(HashMap::new()),
        }
    }

    fn to_json<T>(item: T) -> Result<String>
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

    fn from_json<'a, T>(js: &'a str) -> Result<T>
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

    fn lock(&self) -> Result<MutexGuard<HashMap<String, String>>> {
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
    fn store_subscription(&self, sub: &domain::Subscription) -> Result<()> {
        let result = {
            let item = DBSubscription::from_domain(sub);
            let data = InMemDatastore::to_json(&item)?;

            self.lock()?.insert(item.uuid, data);
            Ok(())
        };

        crate::custom_log(
            log::Level::Info,
            "InMemDatastore",
            "store_subscription",
            vec![Key::new("result").string(format!("{:?}", result))],
        );

        result
    }

    fn get_subscription(&self, uuid: &str) -> Result<domain::Subscription> {
        let result = {
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
        };

        crate::custom_log(
            log::Level::Info,
            "InMemDatastore",
            "get_subscription",
            vec![Key::new("result").string(format!("{:?}", result))],
        );
        result
    }
}

impl std::fmt::Debug for InMemDatastore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InMemDatastore",)
    }
}

// DTOs -------------------

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct DBSubscription {
    uuid: String,
    name: String,
    email: String,
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
