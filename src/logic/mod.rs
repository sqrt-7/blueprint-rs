pub mod domain;
pub mod error;
pub mod validation;

use self::{error::*, validation::ValidateDomain, validation::Validator};
use crate::datastore::{Datastore, DatastoreErrorType};
use std::{result, sync::Arc};
use uuid::Uuid;

pub type Result<T> = result::Result<T, ServiceError>;

pub struct Service {
    datastore: Arc<dyn Datastore>,
}

impl Service {
    pub fn new(datastore: Arc<dyn Datastore>) -> Self {
        Service { datastore }
    }

    fn validator(&self) -> Validator {
        Validator::new(Arc::clone(&self.datastore))
    }

    // LOGIC -----------------

    pub fn create_subscription(&self, email: String, name: String) -> Result<domain::Subscription> {
        let uuid = Uuid::new_v4().to_string();
        let sub = domain::Subscription::new(uuid, email, name);
        self.validator().validate(&sub)?;

        let result = self.datastore.store_subscription(&sub);

        if let Err(db_err) = result {
            let err = ServiceError::new(CODE_DB_ERROR)
                .with_type(ServiceErrorType::Internal)
                .with_internal(format!("datastore.store_subscription: {}", db_err));
            return Err(err);
        };

        Ok(sub)
    }

    pub fn get_subscription(&self, uuid: &str) -> Result<domain::Subscription> {
        match self.datastore.get_subscription(uuid) {
            Ok(sub) => {
                // do some stuff idk
                Ok(sub)
            }

            Err(db_err) => match db_err.error_type {
                DatastoreErrorType::NotFound => Err(ServiceError::new(CODE_SUB_NOT_FOUND)
                    .with_type(ServiceErrorType::NotFound)
                    .with_internal(db_err.msg)),

                _ => Err(ServiceError::new(CODE_DB_ERROR)
                    .with_type(ServiceErrorType::Internal)
                    .with_internal(format!("datastore.get_subscription: {}", db_err))),
            },
        }
    }
}

impl core::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logic::Service",)
    }
}
