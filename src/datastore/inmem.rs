use super::{Datastore, DatastoreError, DatastoreErrorType};
use crate::logic::domain;
use std::{
    collections::HashMap,
    result,
    sync::{Mutex, MutexGuard},
};

type Result<T> = result::Result<T, DatastoreError>;

pub struct InMemDatastore {
    subscriptions: Mutex<HashMap<(String, String), String>>, // <(user_id, journal_id), json>
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            subscriptions: Mutex::new(HashMap::new()),
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

    fn lock(&self) -> Result<MutexGuard<HashMap<(String, String), String>>> {
        match self.subscriptions.lock() {
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
        let item = DBSubscription::from_domain(sub);
        let data = InMemDatastore::to_json(&item)?;

        self.lock()?.insert((item.user_id, item.journal_id), data);
        Ok(())
    }

    fn list_subscriptions_by_user(&self, user_id: &str) -> Result<Vec<domain::Subscription>> {
        let db = self.lock()?;
        let filtered = db
            .iter()
            .filter(|entry| entry.0 .0 == user_id)
            .collect::<HashMap<_, _>>();

        let mut found = Vec::new();

        for js in filtered.values() {
            let item = InMemDatastore::from_json::<DBSubscription>(js)?;
            found.push(item.to_domain()?);
        }

        Ok(found)
    }

    // fn get_subscription(&self, uuid: &str) -> Result<domain::Subscription> {
    //     let result = {
    //         match self.lock()?.get(uuid) {
    //             Some(data) => {
    //                 let item = InMemDatastore::from_json::<DBSubscription>(data)?;
    //                 Ok(item.to_domain())
    //             }

    //             None => Err(DatastoreError::new(
    //                 format!("uuid: {}", uuid),
    //                 DatastoreErrorType::NotFound,
    //             )),
    //         }
    //     };

    //     crate::custom_log(
    //         log::Level::Info,
    //         file!(),
    //         "get_subscription",
    //         vec![Key::new("result").string(format!("{:?}", result))],
    //     );
    //     result
    // }
}

impl std::fmt::Debug for InMemDatastore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InMemDatastore",)
    }
}

// DTOs -------------------

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct DBSubscription {
    user_id: String,
    journal_id: String,
}

impl DBSubscription {
    fn from_domain(inp: &domain::Subscription) -> DBSubscription {
        DBSubscription {
            user_id: inp.user_id().to_string(),
            journal_id: inp.journal_id().to_string(),
        }
    }

    fn to_domain(self) -> Result<domain::Subscription> {
        let user_uuid = domain::Uuid::try_parse(&(self.user_id));
        if user_uuid.is_err() {
            return Err(DatastoreError {
                msg: format!("DBSubscription::to_domain() failed at user_id: {}", self.user_id),
                error_type: DatastoreErrorType::DataCorruption,
            });
        }

        let journal_uuid = domain::Uuid::try_parse(&(self.journal_id));
        if journal_uuid.is_err() {
            return Err(DatastoreError {
                msg: format!(
                    "DBSubscription::to_domain() failed at journal_id: {}",
                    self.journal_id
                ),
                error_type: DatastoreErrorType::DataCorruption,
            });
        }

        Ok(domain::Subscription::new(
            user_uuid.unwrap(),
            journal_uuid.unwrap(),
        ))
    }
}
