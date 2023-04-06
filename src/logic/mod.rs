pub mod domain;
pub mod dto;
pub mod error;

use self::{
    domain::{Email, UserName, Uuid},
    error::*,
};
use crate::datastore::{Datastore, DatastoreError, DatastoreErrorType};
use std::{result, sync::Arc};

pub type Result<T> = result::Result<T, ServiceError>;

pub struct Service {
    datastore: Arc<dyn Datastore>,
}

impl Service {
    pub fn new(datastore: Arc<dyn Datastore>) -> Self {
        Service { datastore }
    }

    // -----------------------
    // USE CASES -------------
    // -----------------------

    pub fn create_user(&self, data: dto::CreateUserRequest) -> Result<domain::User> {
        let new_uuid = Uuid::new();
        let email = Email::try_parse(data.email.as_str())?;
        let name = UserName::try_parse(data.name.as_str())?;

        let obj = domain::User::new(new_uuid, email, name);

        if let Err(db_err) = self.datastore.store_user(&obj) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(obj)
    }

    pub fn get_user(&self, uuid: &str) -> Result<domain::User> {
        match self.datastore.get_user(uuid) {
            Ok(obj) => Ok(obj),
            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => Err(ServiceError::new(CODE_USER_NOT_FOUND)
                    .with_type(ServiceErrorType::NotFound)
                    .with_internal(format!("datastore: {}", db_err))),
                _ => Err(datastore_internal_error(db_err)),
            },
        }
    }

    pub fn create_subscription(&self, data: dto::CreateSubscriptionRequest) -> Result<domain::Subscription> {
        let user_uuid = valid_uuid(data.user_id)?;
        let journal_uuid = valid_uuid(data.journal_id)?;

        let sub = domain::Subscription::new(user_uuid, journal_uuid);

        if let Err(db_err) = self.datastore.store_subscription(&sub) {
            return Err(datastore_internal_error(db_err));
        };

        Ok(sub)
    }

    pub fn list_subscriptions_by_user(&self, user_id: &str) -> Result<Vec<domain::Subscription>> {
        match self.datastore.list_subscriptions_by_user(user_id) {
            Ok(sub) => Ok(sub),
            Err(db_err) => Err(datastore_internal_error(db_err)),
        }
    }
}

impl core::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logic::Service",)
    }
}

// -----------------------
// HELPERS ---------------
// -----------------------

fn valid_uuid(raw: String) -> Result<Uuid> {
    let uuid = domain::Uuid::try_parse(raw.as_str());
    if let Err(e) = uuid {
        return Err(ServiceError::new(CODE_INVALID_UUID)
            .with_type(ServiceErrorType::Validation)
            .with_internal(e));
    };
    Ok(uuid.unwrap())
}

fn datastore_internal_error(db_err: DatastoreError) -> ServiceError {
    ServiceError::new(CODE_DB_ERROR)
        .with_type(ServiceErrorType::Internal)
        .with_internal(format!("datastore: {}", db_err))
}
