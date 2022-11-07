pub mod domain;
pub mod error;

use self::error::{ServiceError, ServiceErrorType};
use crate::{datastore, datastore::Datastore, settings::Settings};
use std::result;
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

    // LOGIC -----------------

    pub fn create_subscription(&self, email: String, name: String) -> Result<domain::Subscription> {
        let uuid = Uuid::new_v4().to_string();
        let sub = domain::Subscription::new(uuid.clone(), email.clone(), name.clone());

        {
            let db_sub = datastore::Subscription { uuid, name, email };

            let result = self.datastore.store_subscription(db_sub);
            if let Err(e) = result {
                let err = ServiceError::new("failed to create new subscription")
                    .with_internal(format!("datastore.store_subscription: {}", e).as_str());

                return Err(err);
            };
        }

        Ok(sub)
    }

    pub fn get_subscription(&self, uuid: String) -> Result<domain::Subscription> {
        let db_sub = self.datastore.get_subscription(uuid.clone());

        if let Err(db_err) = db_sub {
            let err = if db_err.contains("not found") {
                ServiceError::new("subscription does not exist")
                    .with_type(ServiceErrorType::NotFound)
                    .with_internal(format!("uuid: {}", uuid).as_str())
            } else {
                ServiceError::new("failed to get subscription")
                    .with_internal(format!("datastore.get_subscription: {}", db_err).as_str())
            };

            return Err(err);
        };

        let db_sub = db_sub.unwrap();

        let sub = domain::Subscription::new(db_sub.uuid, db_sub.email, db_sub.name);

        Ok(sub)
    }
}
