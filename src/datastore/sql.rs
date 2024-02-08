use super::{DataResult, Datastore, DatastoreError, DatastoreErrorType};
use crate::logic::domain;
use sqlx::{Executor, MySql};

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

        Ok(SqlDatastore {
            pool: conn,
        })
    }
}

/*
CREATE TABLE IF NOT EXISTS `users` (`id` VARCHAR(36) PRIMARY KEY,`email` VARCHAR(255) UNIQUE NOT NULL, `name` VARCHAR(255) NOT NULL);
*/

#[tonic::async_trait]
impl Datastore for SqlDatastore {
    async fn store_user(&self, usr: &domain::User) -> DataResult<()> {
        let q = sqlx::query("INSERT INTO `users` (`id`, `email`,`name` ) VALUES (?, ?, ?)")
            .bind(usr.id().to_string())
            .bind(usr.email().to_string())
            .bind(usr.name().to_string());

        match self.pool.execute(q).await {
            Err(e) => sql_error("store_user", e),
            Ok(_) => Ok(()),
        }
    }

    async fn get_user(&self, id: &domain::ID) -> DataResult<domain::User> {
        let res = sqlx::query_as::<_, UserRow>("SELECT * FROM `users` WHERE `id` = ?")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await;

        match res {
            Err(e) => sql_error("get_user", e),
            Ok(row) => match domain::User::try_from(row) {
                Ok(v) => Ok(v),
                Err(err) => Err(DatastoreError::new(
                    err,
                    DatastoreErrorType::DataCorruption,
                )),
            },
        }
    }

    async fn store_journal(&self, _j: &domain::Journal) -> DataResult<()> {
        todo!()
    }

    async fn get_journal(&self, _id: &domain::ID) -> DataResult<domain::Journal> {
        todo!()
    }

    async fn store_subscription(&self, _sub: &domain::Subscription) -> DataResult<()> {
        todo!()
    }

    async fn list_subscriptions_by_user(
        &self, _user_id: &domain::ID,
    ) -> DataResult<Vec<domain::Subscription>> {
        todo!()
    }
}

fn sql_error<T>(func: &str, err: sqlx::Error) -> Result<T, DatastoreError> {
    let transform = match err {
        sqlx::Error::Database(ref boxed_error) => {
            if boxed_error.is_unique_violation() {
                DatastoreErrorType::Duplicate
            } else {
                DatastoreErrorType::Other
            }
        },

        sqlx::Error::RowNotFound => DatastoreErrorType::NotFound,

        sqlx::Error::Protocol(_)
        | sqlx::Error::TypeNotFound {
            ..
        }
        | sqlx::Error::ColumnNotFound(_)
        | sqlx::Error::ColumnDecode {
            ..
        }
        | sqlx::Error::Decode(_) => DatastoreErrorType::DataCorruption,

        _ => DatastoreErrorType::Other,
    };

    Err(DatastoreError::new(
        format!("SqlDatastore::{func} error:{:?}", err),
        transform,
    ))
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    email: String,
    name: String,
}

impl TryFrom<UserRow> for domain::User {
    type Error = String;

    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        let id = domain::ID::try_from(value.id)?;
        let email = domain::Email::try_from(value.email)?;
        let name = domain::UserName::try_from(value.name)?;

        Ok(domain::User::new(id, email, name))
    }
}
