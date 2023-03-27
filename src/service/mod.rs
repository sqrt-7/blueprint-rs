pub mod error;

use self::error::*;

use crate::{
    datastore::{Datastore, DatastoreErrorType},
    domain,
};
use std::{result, sync::Arc};
use uuid::Uuid;

pub type Result<T> = result::Result<T, ServiceError>;

pub struct Service {
    datastore: Box<dyn Datastore + 'static>,
}

impl Service {
    pub fn new(datastore: impl Datastore + 'static) -> Self {
        Service {
            datastore: Box::new(datastore),
        }
    }

    pub fn new_arc(datastore: impl Datastore + 'static) -> Arc<Self> {
        Arc::new(Service {
            datastore: Box::new(datastore),
        })
    }

    // LOGIC -----------------

    pub fn create_subscription(&self, email: String, name: String) -> Result<domain::Subscription> {
        let uuid = Uuid::new_v4().to_string();
        let sub = domain::Subscription::new(uuid, email, name);

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
                    .with_internal(db_err.msg.to_owned())),

                _ => Err(ServiceError::new(CODE_DB_ERROR)
                    .with_type(ServiceErrorType::Internal)
                    .with_internal(format!("datastore.get_subscription: {}", db_err))),
            },
        }
    }
}
