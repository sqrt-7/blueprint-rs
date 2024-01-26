pub mod domain;
pub mod dto;
pub mod error;

use self::{
    domain::{Email, JournalTitle, JournalYear, UserName, ID},
    error::*,
};
use crate::{
    datastore::{Datastore, DatastoreError, DatastoreErrorType},
    toolbox::{context::Context, logger},
};
use std::{result, sync::Arc};

type Result<T> = result::Result<T, ServiceError>;

pub struct Logic {
    datastore: Arc<dyn Datastore + Send + Sync>,
}

impl Logic {
    pub fn new(datastore: Arc<dyn Datastore + Send + Sync>) -> Self {
        Self {
            datastore,
        }
    }

    // -----------------------
    // USE CASES -------------
    // -----------------------

    pub fn create_user(
        &self,
        ctx: Arc<Context>,
        data: dto::CreateUserRequest,
    ) -> Result<domain::User> {
        logger::ctx_info!(ctx, "hellllo");

        let new_id = ID::new();
        let email = Email::try_from(data.email)?;
        let name = UserName::try_from(data.name)?;

        let obj = domain::User::new(new_id, email, name);

        if let Err(db_err) = self.datastore.store_user(&obj) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(obj)
    }

    pub fn get_user(&self, _: Arc<Context>, id: &str) -> Result<domain::User> {
        let id = ID::try_from(id)?;
        match self.datastore.get_user(&id) {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => {
                    Err(ServiceError::new(ServiceErrorCode::UserNotFound)
                        .with_type(ServiceErrorType::NotFound)
                        .wrap(db_err))
                },
                _ => Err(datastore_internal_error(db_err)),
            },
        }
    }

    pub fn create_journal(
        &self,
        _: Arc<Context>,
        data: dto::CreateJournalRequest,
    ) -> Result<domain::Journal> {
        let new_id = ID::new();
        let title = JournalTitle::try_from(data.title)?;
        let year = JournalYear::try_from(data.year)?;

        let obj = domain::Journal::new(new_id, title, year);

        if let Err(db_err) = self.datastore.store_journal(&obj) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(obj)
    }

    pub fn get_journal(&self, _: Arc<Context>, id: &str) -> Result<domain::Journal> {
        let id = ID::try_from(id)?;
        match self.datastore.get_journal(&id) {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => Err(ServiceError::new(
                    ServiceErrorCode::JournalNotFound,
                )
                .with_type(ServiceErrorType::NotFound)
                .wrap(db_err)),
                _ => Err(datastore_internal_error(db_err)),
            },
        }
    }

    pub fn create_subscription(
        &self,
        _: Arc<Context>,
        data: dto::CreateSubscriptionRequest,
    ) -> Result<domain::Subscription> {
        let user_id = ID::try_from(data.user_id)?;
        let journal_id = ID::try_from(data.journal_id)?;
        let new_id = ID::new();

        let sub = domain::Subscription::new(new_id, user_id, journal_id);

        if let Err(db_err) = self.datastore.store_subscription(&sub) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(sub)
    }

    pub fn list_subscriptions_by_user(
        &self,
        _: Arc<Context>,
        user_id: &str,
    ) -> Result<Vec<domain::Subscription>> {
        let user_id = ID::try_from(user_id)?;
        match self
            .datastore
            .list_subscriptions_by_user(&user_id)
        {
            Ok(res) => Ok(res),
            Err(db_err) => Err(datastore_internal_error(db_err)),
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

fn datastore_internal_error(db_err: DatastoreError) -> ServiceError {
    ServiceError::new(ServiceErrorCode::UnexpectedError)
        .with_type(ServiceErrorType::Internal)
        .wrap(db_err)
}
