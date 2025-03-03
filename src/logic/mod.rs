pub mod domain;
pub mod dto;
pub mod error;

use self::{domain::ID, error::*};
use crate::{
    datastore::{Datastore, DatastoreErrorType},
    toolbox::{context::Context, logger},
};
use std::result;

type LogicResult<T> = result::Result<T, LogicError>;

pub struct Logic {
    datastore: Box<dyn Datastore + Send + Sync>,
}

impl Logic {
    pub fn new(datastore: Box<dyn Datastore + Send + Sync>) -> Self {
        Self {
            datastore,
        }
    }

    // -----------------------
    // USE CASES -------------
    // -----------------------

    pub async fn create_user(
        &self, ctx: &Context, data: dto::CreateUserRequest,
    ) -> LogicResult<domain::User> {
        logger::ctx_info!(ctx, "hello");

        let new_id = ID::new().to_string();
        let obj = domain::User::try_new(&new_id, &data.email, &data.name)?;

        match self.datastore.store_user(&obj).await {
            Ok(_) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::Conflict => {
                    Err(LogicError::new(LogicErrorCode::DuplicateEmail).wrap(db_err))
                },
                _ => Err(LogicError::new(LogicErrorCode::UnexpectedError).wrap(db_err)),
            },
        }
    }

    pub async fn get_user(&self, _: &Context, id: &str) -> LogicResult<domain::User> {
        let id = parse_id(id)?;

        match self.datastore.get_user(&id).await {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => {
                    Err(LogicError::new(LogicErrorCode::UserNotFound).wrap(db_err))
                },
                _ => Err(LogicError::new(LogicErrorCode::UnexpectedError).wrap(db_err)),
            },
        }
    }

    pub async fn list_users(&self, _: &Context, _: dto::Query) -> LogicResult<Vec<domain::User>> {
        match self.datastore.list_users().await {
            Ok(res) => Ok(res),
            Err(db_err) => Err(LogicError::new(LogicErrorCode::UnexpectedError).wrap(db_err)),
        }
    }
}

impl core::fmt::Debug for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logic::Logic",)
    }
}

// -----------------------
// HELPERS ---------------
// -----------------------

fn parse_id(value: &str) -> LogicResult<domain::ID> {
    match domain::ID::try_from(value) {
        Ok(id) => Ok(id),
        Err(e) => Err(LogicError::new(LogicErrorCode::InvalidID).with_internal_msg(e)),
    }
}
