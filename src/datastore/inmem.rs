use super::{Datastore, DatastoreError, DatastoreErrorType};
use crate::logic::domain;
use std::{
    collections::HashMap,
    result,
    sync::{Mutex, MutexGuard, PoisonError},
};

type Result<T> = result::Result<T, DatastoreError>;

pub struct InMemDatastore {
    users: Mutex<HashMap<String, String>>,                   // <uuid, json>
    subscriptions: Mutex<HashMap<(String, String), String>>, // <(user_id, journal_id), json>
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            users: Mutex::new(HashMap::new()),
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

    fn check_lock<'a, T>(
        lock_result: result::Result<MutexGuard<'a, T>, PoisonError<MutexGuard<'a, T>>>,
    ) -> Result<MutexGuard<'a, T>> {
        match lock_result {
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
    fn store_user(&self, obj: &domain::User) -> Result<()> {
        let item = DBUser::from_domain(obj);
        let data = InMemDatastore::to_json(&item)?;

        let mut db = InMemDatastore::check_lock(self.users.lock())?;
        db.insert(item.uuid, data);
        Ok(())
    }

    fn get_user(&self, uuid: &str) -> Result<domain::User> {
        let db = InMemDatastore::check_lock(self.users.lock())?;
        match db.get(uuid) {
            Some(data) => {
                let item = InMemDatastore::from_json::<DBUser>(data)?;
                Ok(item.to_domain()?)
            },

            None => Err(DatastoreError::new(
                format!("uuid: {}", uuid),
                DatastoreErrorType::NotFound,
            )),
        }
    }

    fn store_subscription(&self, sub: &domain::Subscription) -> Result<()> {
        let mut db = InMemDatastore::check_lock(self.subscriptions.lock())?;

        let item = DBSubscription::from_domain(sub);
        let data = InMemDatastore::to_json(&item)?;

        db.insert((item.user_id, item.journal_id), data);
        Ok(())
    }

    fn list_subscriptions_by_user(&self, user_id: &str) -> Result<Vec<domain::Subscription>> {
        let db = InMemDatastore::check_lock(self.subscriptions.lock())?;
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
        let user_uuid = crate::Uuid::try_from(self.user_id);
        if let Err(e) = user_uuid {
            return Err(DatastoreError {
                msg: format!("DBSubscription::to_domain() failed at user_id: {}", e),
                error_type: DatastoreErrorType::DataCorruption,
            });
        }

        let journal_uuid = crate::Uuid::try_from(self.journal_id);
        if let Err(e) = journal_uuid {
            return Err(DatastoreError {
                msg: format!("DBSubscription::to_domain() failed at journal_id: {}", e),
                error_type: DatastoreErrorType::DataCorruption,
            });
        }

        Ok(domain::Subscription::new(
            user_uuid.unwrap(),
            journal_uuid.unwrap(),
        ))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct DBUser {
    uuid: String,
    email: String,
    name: String,
}

impl DBUser {
    fn from_domain(inp: &domain::User) -> DBUser {
        DBUser {
            uuid: inp.uuid().to_string(),
            email: inp.email().to_string(),
            name: inp.name().to_string(),
        }
    }

    fn to_domain(self) -> Result<domain::User> {
        let uuid = crate::Uuid::try_from(self.uuid);
        if let Err(e) = uuid {
            return Err(DatastoreError {
                msg: format!("DBUser::to_domain() failed at uuid: {}", e),
                error_type: DatastoreErrorType::DataCorruption,
            });
        }

        let email = domain::Email::try_from(self.email);
        if let Err(e) = email {
            return Err(DatastoreError {
                msg: format!("DBUser::to_domain() failed at email: {}", e),
                error_type: DatastoreErrorType::DataCorruption,
            });
        }

        let name = domain::UserName::try_from(self.name);
        if let Err(e) = name {
            return Err(DatastoreError {
                msg: format!("DBUser::to_domain() failed at name: {}", e),
                error_type: DatastoreErrorType::DataCorruption,
            });
        }

        Ok(domain::User::new(uuid.unwrap(), email.unwrap(), name.unwrap()))
    }
}
