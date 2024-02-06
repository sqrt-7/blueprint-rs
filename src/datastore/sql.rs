use super::{Datastore, DatastoreError};
use sqlx::MySql;

static DB_NAME: &str = "blueprint_db";
static MAX_CONN: u32 = 5;

pub struct SqlDatastore {
    pool: sqlx::Pool<MySql>,
}

impl SqlDatastore {
    pub async fn new(
        addr: &str, port: u16, user: &str, pw: &str,
    ) -> Result<SqlDatastore, sqlx::Error> {
        let url = format!("mysql://{user}:{pw}@{addr}:{port}/{DB_NAME}");

        let conn = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(MAX_CONN)
            .connect(url.as_str())
            .await?;

        let db = SqlDatastore {
            pool: conn,
        };

        Ok(db)
    }
}

impl Datastore for SqlDatastore {
    fn store_user(
        &self, usr: &crate::logic::domain::User,
    ) -> std::prelude::v1::Result<(), DatastoreError> {
        todo!()
    }

    fn get_user(
        &self, id: &crate::logic::domain::ID,
    ) -> std::prelude::v1::Result<crate::logic::domain::User, DatastoreError> {
        todo!()
    }

    fn store_journal(
        &self, j: &crate::logic::domain::Journal,
    ) -> std::prelude::v1::Result<(), DatastoreError> {
        todo!()
    }

    fn get_journal(
        &self, id: &crate::logic::domain::ID,
    ) -> std::prelude::v1::Result<crate::logic::domain::Journal, DatastoreError> {
        todo!()
    }

    fn store_subscription(
        &self, sub: &crate::logic::domain::Subscription,
    ) -> std::prelude::v1::Result<(), DatastoreError> {
        todo!()
    }

    fn list_subscriptions_by_user(
        &self, user_id: &crate::logic::domain::ID,
    ) -> std::prelude::v1::Result<Vec<crate::logic::domain::Subscription>, DatastoreError> {
        todo!()
    }
}
