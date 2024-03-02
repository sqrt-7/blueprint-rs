use super::{DataResult, Datastore, DatastoreError, DatastoreErrorType};
use crate::logic::domain;
use std::{collections::HashMap, sync::Mutex};

pub struct InMemDatastore {
    // HashMap<String, String> and Vec<String> are Send+Sync
    // so Mutex is also Send+Sync => no Arc needed
    users: Mutex<HashMap<String, String>>, // <id, json>
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            users: Mutex::new(HashMap::new()),
        }
    }

    fn to_json<T>(item: T) -> DataResult<String>
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

    fn from_json<'a, T>(js: &'a str) -> DataResult<T>
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

#[tonic::async_trait]
impl Datastore for InMemDatastore {
    async fn store_user(&self, obj: &domain::User) -> DataResult<()> {
        println!("id: {:?}", std::thread::current().id());
        println!("name: {:?}", std::thread::current().name());

        let data = InMemDatastore::to_json(obj)?;
        let mut db = self.users.lock().unwrap();
        db.insert(obj.id().to_string(), data);
        Ok(())
    }

    async fn get_user(&self, id: &domain::ID) -> DataResult<domain::User> {
        let db = self.users.lock().unwrap();
        match db.get(&id.to_string()) {
            Some(data) => {
                let item = InMemDatastore::from_json::<domain::User>(data)?;
                Ok(item)
            },

            None => Err(DatastoreError::new(
                format!("id: {}", id),
                DatastoreErrorType::NotFound,
            )),
        }
    }

    async fn list_users(&self) -> DataResult<Vec<domain::User>> {
        let db = self.users.lock().unwrap();
        let mut items: Vec<domain::User> = Vec::new();

        for (_, data) in db.iter() {
            let u = InMemDatastore::from_json::<domain::User>(data)?;
            items.push(u);
        }

        Ok(items)
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
        logic::domain::{Email, User, UserName, ID},
    };

    use super::InMemDatastore;

    #[test]
    fn datastore_is_send_sync() {
        let _: Box<dyn Send + Sync> = Box::new(InMemDatastore::new());
    }

    #[tokio::test]
    async fn add_user_get_user() {
        let ds = InMemDatastore::new();
        let usr = User::new(
            ID::new(),
            Email::try_from("test@test.com".to_owned()).unwrap(),
            UserName::try_from("Jeff Jeffries".to_owned()).unwrap(),
        );

        ds.store_user(&usr).await.unwrap();

        let res = ds.get_user(usr.id()).await.unwrap();
        assert_eq!(res.id().to_string(), usr.id().to_string());
        assert_eq!(res.email().to_string(), usr.email().to_string());
        assert_eq!(res.name().to_string(), usr.name().to_string());
    }

    #[tokio::test]
    async fn get_user_corrupt_data() {
        let ds = InMemDatastore::new();
        let user_id = ID::new();

        // Add invalid json
        {
            let mut lock = ds.users.lock().unwrap();
            lock.insert(
                user_id.to_string(),
                "{\"hello\": \"world\"}".to_owned(),
            );
        }

        let res = ds
            .get_user(&user_id)
            .await
            .expect_err("should be error");

        assert!(matches!(
            res.error_type,
            DatastoreErrorType::DataCorruption
        ));
    }

    #[tokio::test]
    async fn add_users_list_users() {
        let ds = InMemDatastore::new();

        let id1 = ID::new();
        let id2 = ID::new();
        let id3 = ID::new();
        let id4 = ID::new();
        let id5 = ID::new();

        let email1 = Email::try_from("user1@test.com".to_string()).unwrap();
        let email2 = Email::try_from("user2@test.com".to_string()).unwrap();
        let email3 = Email::try_from("user3@test.com".to_string()).unwrap();
        let email4 = Email::try_from("user4@test.com".to_string()).unwrap();
        let email5 = Email::try_from("user5@test.com".to_string()).unwrap();

        let name1 = UserName::try_from("Person One".to_string()).unwrap();
        let name2 = UserName::try_from("Person Two".to_string()).unwrap();
        let name3 = UserName::try_from("Person Three".to_string()).unwrap();
        let name4 = UserName::try_from("Person Four".to_string()).unwrap();
        let name5 = UserName::try_from("Person Five".to_string()).unwrap();

        let user1 = User::new(id1, email1, name1);
        let user2 = User::new(id2, email2, name2);
        let user3 = User::new(id3, email3, name3);
        let user4 = User::new(id4, email4, name4);
        let user5 = User::new(id5, email5, name5);

        ds.store_user(&user1).await.unwrap();
        ds.store_user(&user2).await.unwrap();
        ds.store_user(&user3).await.unwrap();
        ds.store_user(&user4).await.unwrap();
        ds.store_user(&user5).await.unwrap();

        {
            let res = ds.list_users().await.unwrap();

            assert!(res.len() == 5);
            assert!(res.contains(&user1));
            assert!(res.contains(&user2));
            assert!(res.contains(&user3));
            assert!(res.contains(&user4));
            assert!(res.contains(&user5));
        }
    }
}
