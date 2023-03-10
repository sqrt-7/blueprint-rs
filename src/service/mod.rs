pub mod error;

use self::error::{
    ServiceError, ServiceErrorType, CODE_SUB_CREATE_FAIL, CODE_SUB_NOT_FOUND, CODE_UNEXPECTED,
};
use crate::{datastore, datastore::Datastore, domain, settings::Settings};
use std::{result, sync::Arc};
use uuid::Uuid;

pub type Result<T> = result::Result<T, ServiceError>;

pub struct Service {
    datastore: Box<dyn Datastore + 'static>,
    settings: Settings,
}

impl Service {
    pub fn new(settings: Settings, datastore: impl Datastore + 'static) -> Self {
        Service {
            datastore: Box::new(datastore),
            settings,
        }
    }

    pub fn new_arc(settings: Settings, datastore: impl Datastore + 'static) -> Arc<Self> {
        Arc::new(Service {
            datastore: Box::new(datastore),
            settings,
        })
    }

    // LOGIC -----------------

    pub fn create_subscription(&self, email: String, name: String) -> Result<domain::Subscription> {
        let uuid = Uuid::new_v4().to_string();
        let sub = domain::Subscription::new(uuid, email, name);

        let result = self.datastore.store_subscription(&sub);
        if let Err(e) = result {
            let err = ServiceError::new(CODE_SUB_CREATE_FAIL)
                .with_internal(format!("datastore.store_subscription: {}", e).as_str());

            return Err(err);
        };

        Ok(sub)
    }

    pub fn get_subscription(&self, uuid: String) -> Result<domain::Subscription> {
        match self.datastore.get_subscription(uuid) {
            Ok(sub) => {
                // do some validation etc...
                Ok(sub)
            }

            Err(db_err) => {
                // Not found
                if let Some(not_found_err) = db_err.downcast_ref::<datastore::NotFoundError>() {
                    return Err(ServiceError::new(CODE_SUB_NOT_FOUND)
                        .with_type(ServiceErrorType::NotFound)
                        .with_internal(not_found_err.msg.as_str()));
                }

                // All other errors
                return Err(ServiceError::new(CODE_UNEXPECTED)
                    .with_internal(format!("datastore.get_subscription: {}", db_err).as_str()));
            }
        }
    }
}
