pub mod domain;
pub mod error;

use self::{domain::Uuid, error::*};
use crate::datastore::Datastore;
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

    pub fn create_subscription(&self, user_id: String, journal_id: String) -> Result<domain::Subscription> {
        let user_uuid = Service::valid_uuid(user_id)?;
        let journal_uuid = Service::valid_uuid(journal_id)?;

        let sub = domain::Subscription::new(user_uuid, journal_uuid);

        let result = self.datastore.store_subscription(&sub);

        if let Err(db_err) = result {
            let err = ServiceError::new(CODE_DB_ERROR)
                .with_type(ServiceErrorType::Internal)
                .with_internal(format!("datastore.store_subscription: {}", db_err));
            return Err(err);
        };

        Ok(sub)
    }

    // pub fn get_subscription(&self, uuid: &str) -> Result<domain::Subscription> {
    //     match self.datastore.get_subscription(uuid) {
    //         Ok(sub) => {
    //             // do some stuff idk
    //             Ok(sub)
    //         }

    //         Err(db_err) => match db_err.error_type {
    //             DatastoreErrorType::NotFound => Err(ServiceError::new(CODE_SUB_NOT_FOUND)
    //                 .with_type(ServiceErrorType::NotFound)
    //                 .with_internal(db_err.msg)),

    //             _ => Err(ServiceError::new(CODE_DB_ERROR)
    //                 .with_type(ServiceErrorType::Internal)
    //                 .with_internal(format!("datastore.get_subscription: {}", db_err))),
    //         },
    //     }
    // }

    pub fn list_subscriptions_by_user(&self, user_id: &str) -> Result<Vec<domain::Subscription>> {
        match self.datastore.list_subscriptions_by_user(user_id) {
            Ok(sub) => {
                // do some stuff idk
                Ok(sub)
            },

            Err(db_err) => Err(ServiceError::new(CODE_DB_ERROR)
                .with_type(ServiceErrorType::Internal)
                .with_internal(format!("datastore.list_subscriptions_by_user: {}", db_err))),
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
}

impl core::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logic::Service",)
    }
}
