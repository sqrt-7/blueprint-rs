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
            .connect(&url)
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
            Err(e) => datastore_error("store_user", e),
            Ok(_) => Ok(()),
        }
    }

    async fn get_user(&self, id: &domain::ID) -> DataResult<domain::User> {
        let res = sqlx::query_as::<_, UserRow>("SELECT * FROM `users` WHERE `id` = ?")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await;

        match res {
            Err(e) => datastore_error("get_user", e),
            Ok(row) => convert_from_row(row),
        }
    }

    async fn list_users(&self) -> DataResult<Vec<domain::User>> {
        todo!()
    }
}

fn datastore_error<T>(func: &str, err: sqlx::Error) -> DataResult<T> {
    let ds_err = match err {
        sqlx::Error::Database(ref boxed_error) => {
            if boxed_error.is_unique_violation() {
                DatastoreErrorType::Conflict
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
        ds_err,
    ))
}

fn convert_from_row<T, R>(row: R) -> DataResult<T>
where
    T: TryFrom<R, Error = String>,
{
    T::try_from(row).map_err(|err| DatastoreError::new(err, DatastoreErrorType::DataCorruption))
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
