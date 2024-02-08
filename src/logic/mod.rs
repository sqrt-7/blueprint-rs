pub mod domain;
pub mod dto;
pub mod error;

use self::{domain::ID, error::*};
use crate::{
    datastore::{Datastore, DatastoreErrorType},
    toolbox::{context::Context, logger},
};
use std::result;

type Result<T> = result::Result<T, ServiceError>;

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
    ) -> Result<domain::User> {
        logger::ctx_info!(ctx, "hellllo");

        let new_id = ID::new().to_string();
        let obj = domain::User::try_new(&new_id, &data.email, &data.name)?;

        match self.datastore.store_user(&obj).await {
            Ok(_) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::Duplicate => {
                    Err(ServiceError::new(ServiceErrorCode::DuplicateEmail).wrap(db_err))
                },
                _ => Err(ServiceError::new(ServiceErrorCode::UnexpectedError).wrap(db_err)),
            },
        }
    }

    pub async fn get_user(&self, _: &Context, id: &str) -> Result<domain::User> {
        let id = parse_id(id)?;

        match self.datastore.get_user(&id).await {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => {
                    Err(ServiceError::new(ServiceErrorCode::UserNotFound).wrap(db_err))
                },
                _ => Err(ServiceError::new(ServiceErrorCode::UnexpectedError).wrap(db_err)),
            },
        }
    }

    pub async fn create_journal(
        &self, _: &Context, data: dto::CreateJournalRequest,
    ) -> Result<domain::Journal> {
        let new_id = ID::new().to_string();
        let obj = domain::Journal::try_new(&new_id, &data.title, data.year)?;

        if let Err(db_err) = self.datastore.store_journal(&obj).await {
            return Err(ServiceError::new(ServiceErrorCode::UnexpectedError).wrap(db_err));
        };

        Ok(obj)
    }

    pub async fn get_journal(&self, _: &Context, id: &str) -> Result<domain::Journal> {
        let id = parse_id(id)?;
        match self.datastore.get_journal(&id).await {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => {
                    Err(ServiceError::new(ServiceErrorCode::JournalNotFound).wrap(db_err))
                },
                _ => Err(ServiceError::new(ServiceErrorCode::UnexpectedError).wrap(db_err)),
            },
        }
    }

    pub async fn create_subscription(
        &self, _: &Context, data: dto::CreateSubscriptionRequest,
    ) -> Result<domain::Subscription> {
        let user_id = parse_id(&data.user_id)?;
        let journal_id = parse_id(&data.journal_id)?;
        let new_id = ID::new();

        let sub = domain::Subscription::new(new_id, user_id, journal_id);

        if let Err(db_err) = self
            .datastore
            .store_subscription(&sub)
            .await
        {
            return Err(ServiceError::new(ServiceErrorCode::UnexpectedError).wrap(db_err));
        };

        Ok(sub)
    }

    pub async fn list_subscriptions_by_user(
        &self, _: &Context, user_id: &str,
    ) -> Result<Vec<domain::Subscription>> {
        let user_id = parse_id(user_id)?;
        match self
            .datastore
            .list_subscriptions_by_user(&user_id)
            .await
        {
            Ok(res) => Ok(res),
            Err(db_err) => Err(ServiceError::new(ServiceErrorCode::UnexpectedError).wrap(db_err)),
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

fn parse_id(value: &str) -> Result<domain::ID> {
    match domain::ID::try_from(value) {
        Ok(id) => Ok(id),
        Err(e) => Err(ServiceError::new(ServiceErrorCode::InvalidID).with_internal_msg(e)),
    }
}
