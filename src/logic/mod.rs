pub mod domain;
pub mod dto;
pub mod error;

use self::{
    domain::{Email, JournalTitle, JournalYear, UserName, Uuid},
    error::*,
};
use crate::datastore::{Datastore, DatastoreError, DatastoreErrorType};
use std::{result, sync::Arc};

pub type Result<T> = result::Result<T, ServiceError>;

pub struct Controller {
    datastore: Arc<dyn Datastore + Send + Sync>,
}

impl Controller {
    pub fn new(datastore: Arc<dyn Datastore + Send + Sync>) -> Self {
        Controller {
            datastore,
        }
    }

    // -----------------------
    // USE CASES -------------
    // -----------------------

    pub fn create_user(&self, data: dto::CreateUserRequest) -> Result<domain::User> {
        let new_uuid = Uuid::new();
        let email = Email::try_from(data.email)?;
        let name = UserName::try_from(data.name)?;

        let obj = domain::User::new(new_uuid, email, name);

        if let Err(db_err) = self.datastore.store_user(&obj) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(obj)
    }

    pub fn get_user(&self, uuid: &str) -> Result<domain::User> {
        let uuid = Uuid::try_from(uuid)?;
        match self.datastore.get_user(&uuid) {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => Err(ServiceError::new(CODE_USER_NOT_FOUND)
                    .with_type(ServiceErrorType::NotFound)
                    .with_internal(format!("datastore: {}", db_err))),
                _ => Err(datastore_internal_error(db_err)),
            },
        }
    }

    pub fn create_journal(&self, data: dto::CreateJournalRequest) -> Result<domain::Journal> {
        let new_uuid = Uuid::new();
        let title = JournalTitle::try_from(data.title)?;
        let year = JournalYear::try_from(data.year)?;

        let obj = domain::Journal::new(new_uuid, title, year);

        if let Err(db_err) = self.datastore.store_journal(&obj) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(obj)
    }

    pub fn get_journal(&self, uuid: &str) -> Result<domain::Journal> {
        let uuid = Uuid::try_from(uuid)?;
        match self.datastore.get_journal(&uuid) {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => Err(ServiceError::new(CODE_JOURNAL_NOT_FOUND)
                    .with_type(ServiceErrorType::NotFound)
                    .with_internal(format!("datastore: {}", db_err))),
                _ => Err(datastore_internal_error(db_err)),
            },
        }
    }

    pub fn create_subscription(
        &self,
        data: dto::CreateSubscriptionRequest,
    ) -> Result<domain::Subscription> {
        let user_uuid = Uuid::try_from(data.user_id)?;
        let journal_uuid = Uuid::try_from(data.journal_id)?;
        let new_uuid = Uuid::new();

        let sub = domain::Subscription::new(new_uuid, user_uuid, journal_uuid);

        if let Err(db_err) = self.datastore.store_subscription(&sub) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(sub)
    }

    pub fn list_subscriptions_by_user(&self, user_id: &str) -> Result<Vec<domain::Subscription>> {
        let user_id = Uuid::try_from(user_id)?;
        match self.datastore.list_subscriptions_by_user(&user_id) {
            Ok(res) => Ok(res),
            Err(db_err) => Err(datastore_internal_error(db_err)),
        }
    }
}

impl core::fmt::Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logic::Controller",)
    }
}

// -----------------------
// HELPERS ---------------
// -----------------------

fn datastore_internal_error(db_err: DatastoreError) -> ServiceError {
    ServiceError::new(CODE_DB_ERROR)
        .with_type(ServiceErrorType::Internal)
        .with_internal(format!("datastore: {}", db_err))
}
