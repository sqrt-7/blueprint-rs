use super::{Datastore, DatastoreError, DatastoreErrorType};
use crate::logic::domain;
use std::{collections::HashMap, result, sync::Mutex};

type Result<T> = result::Result<T, DatastoreError>;

pub struct InMemDatastore {
    // HashMap<String, String> and Vec<String> are Send+Sync
    // so Mutex is also Send+Sync => no Arc needed
    users: Mutex<HashMap<String, String>>,    // <uuid, json>
    journals: Mutex<HashMap<String, String>>, // <uuid, json>
    subs: Mutex<DatastoreSubs>,
}

struct DatastoreSubs {
    subs_db: HashMap<String, String>,                 // <uuid, json>
    subs_index_user: HashMap<String, Vec<String>>,    // <user_id, uuid>
    subs_index_journal: HashMap<String, Vec<String>>, // <journal_id, uuid>
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            users: Mutex::new(HashMap::new()),
            journals: Mutex::new(HashMap::new()),
            subs: Mutex::new(DatastoreSubs {
                subs_db: HashMap::new(),
                subs_index_user: HashMap::new(),
                subs_index_journal: HashMap::new(),
            }),
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
                DatastoreErrorType::DataCorruption,
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
        let data = InMemDatastore::to_json(obj)?;
        let mut db = self.users.lock().unwrap();
        db.insert(obj.uuid().to_string(), data);
        Ok(())
    }

    fn get_user(&self, uuid: &domain::Uuid) -> Result<domain::User> {
        let db = self.users.lock().unwrap();
        match db.get(&uuid.to_string()) {
            Some(data) => {
                let item = InMemDatastore::from_json::<domain::User>(data)?;
                Ok(item)
            },

            None => {
                Err(DatastoreError::new(format!("uuid: {}", uuid), DatastoreErrorType::NotFound))
            },
        }
    }

    fn store_journal(&self, obj: &domain::Journal) -> Result<()> {
        let data = InMemDatastore::to_json(obj)?;
        let mut db = self.journals.lock().unwrap();
        db.insert(obj.uuid().to_string(), data);
        Ok(())
    }

    fn get_journal(&self, uuid: &domain::Uuid) -> Result<domain::Journal> {
        let db = self.journals.lock().unwrap();
        match db.get(&uuid.to_string()) {
            Some(data) => {
                let item = InMemDatastore::from_json::<domain::Journal>(data)?;
                Ok(item)
            },

            None => {
                Err(DatastoreError::new(format!("uuid: {}", uuid), DatastoreErrorType::NotFound))
            },
        }
    }

    fn store_subscription(&self, sub: &domain::Subscription) -> Result<()> {
        let mut db = self.subs.lock().unwrap();
        let data = InMemDatastore::to_json(sub)?;

        // Add to db
        db.subs_db.insert(sub.uuid().to_string(), data);

        // Add to user index
        if let Some(v) = db.subs_index_user.get_mut(&sub.user_id().to_string()) {
            let to_add = sub.uuid().to_string();
            if !v.contains(&to_add) {
                v.push(to_add);
            }
        } else {
            db.subs_index_user
                .insert(sub.user_id().to_string(), Vec::from([sub.uuid().to_string()]));
        }

        // Add to journal index
        if let Some(v) = db.subs_index_journal.get_mut(&sub.journal_id().to_string()) {
            let to_add = sub.uuid().to_string();
            if !v.contains(&to_add) {
                v.push(to_add);
            }
        } else {
            db.subs_index_journal
                .insert(sub.journal_id().to_string(), Vec::from([sub.uuid().to_string()]));
        }

        Ok(())
    }

    fn list_subscriptions_by_user(
        &self,
        user_id: &domain::Uuid,
    ) -> Result<Vec<domain::Subscription>> {
        let db = self.subs.lock().unwrap();

        let mut found = Vec::new();
        if let Some(sub_ids) = db.subs_index_user.get(&user_id.to_string()) {
            for sid in sub_ids {
                let entry = db.subs_db.get(sid);
                if entry.is_none() {
                    // This should never happen
                    return Err(DatastoreError::new(
                        format!(
                            "subs_index_user exists for missing item: (user_id: {}, uuid: {})",
                            user_id, sid
                        ),
                        DatastoreErrorType::DataCorruption,
                    ));
                }

                let item = InMemDatastore::from_json::<domain::Subscription>(entry.unwrap())?;
                found.push(item);
            }
        }

        Ok(found)
    }
}

impl std::fmt::Debug for InMemDatastore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InMemDatastore",)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        datastore::{Datastore, DatastoreErrorType},
        logic::domain::{self, Email, Subscription, User, UserName, Uuid},
    };

    use super::InMemDatastore;

    #[test]

    fn datastore_is_send_sync() {
        let _: Box<dyn Send + Sync> = Box::new(InMemDatastore::new());
    }

    fn add_user_get_user() {
        let ds = InMemDatastore::new();
        let usr = User::new(
            Uuid::new(),
            Email::try_from("test@test.com".to_owned()).unwrap(),
            UserName::try_from("Jeff Jeffries".to_owned()).unwrap(),
        );

        ds.store_user(&usr).unwrap();

        let res = ds.get_user(usr.uuid()).unwrap();
        assert_eq!(res.uuid().to_string(), usr.uuid().to_string());
        assert_eq!(res.email().to_string(), usr.email().to_string());
        assert_eq!(res.name().to_string(), usr.name().to_string());
    }

    #[test]
    fn get_user_corrupt_data() {
        let ds = InMemDatastore::new();
        let user_id = Uuid::new();

        // Add invalid json
        {
            let mut lock = ds.users.lock().unwrap();
            lock.insert(user_id.to_string(), "{\"hello\": \"world\"}".to_owned());
        }

        let res = ds.get_user(&user_id).expect_err("should be error");

        assert!(matches!(res.error_type, DatastoreErrorType::DataCorruption));
    }

    #[test]
    fn add_subs_list_subs() {
        let ds = InMemDatastore::new();

        let user1 = Uuid::new();
        let user2 = Uuid::new();

        let sub1 = Subscription::new(Uuid::new(), user1.clone(), Uuid::new());
        let sub2 = Subscription::new(Uuid::new(), user1.clone(), Uuid::new());
        let sub3 = Subscription::new(Uuid::new(), user1.clone(), Uuid::new());
        let sub4 = Subscription::new(Uuid::new(), user2.clone(), Uuid::new());
        let sub5 = Subscription::new(Uuid::new(), user2.clone(), Uuid::new());
        let sub6 = Subscription::new(Uuid::new(), user2.clone(), Uuid::new());

        ds.store_subscription(&sub1).unwrap();
        ds.store_subscription(&sub1).unwrap(); // duplicate
        ds.store_subscription(&sub1).unwrap(); // duplicate
        ds.store_subscription(&sub2).unwrap();
        ds.store_subscription(&sub2).unwrap(); // duplicate
        ds.store_subscription(&sub3).unwrap();
        ds.store_subscription(&sub4).unwrap();
        ds.store_subscription(&sub5).unwrap();
        ds.store_subscription(&sub6).unwrap();

        {
            let res = ds.list_subscriptions_by_user(&user1).unwrap();

            assert!(res.len() == 3);
            assert!(res.contains(&sub1));
            assert!(res.contains(&sub2));
            assert!(res.contains(&sub3));
        }

        {
            let res = ds.list_subscriptions_by_user(&user2).unwrap();

            assert!(res.len() == 3);
            assert!(res.contains(&sub4));
            assert!(res.contains(&sub5));
            assert!(res.contains(&sub6));
        }

        {
            let some_fake_uuid = domain::Uuid::new();
            let res = ds.list_subscriptions_by_user(&some_fake_uuid).unwrap();
            assert!(res.len() == 0);
        }
    }
}
