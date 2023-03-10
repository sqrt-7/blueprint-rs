use std::{collections::HashMap, error::Error, sync::Mutex};

use crate::{
    datastore::{InternalError, NotFoundError},
    domain,
};

use super::{DBSubscription, Datastore};

pub struct InMemDatastore {
    items: Mutex<HashMap<String, String>>, // <uuid, json>
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            items: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemDatastore {
    fn default() -> Self {
        Self::new()
    }
}

impl Datastore for InMemDatastore {
    fn store_subscription(&self, sub: &domain::Subscription) -> Result<(), Box<dyn Error>> {
        tracing::info!("[InMemDatastore::store_subscription]");

        let item = DBSubscription::from_domain(sub);
        let data = serde_json::to_string(&item)?;

        let ds = self.items.lock();

        if let Err(lock_err) = ds {
            return Err(InternalError::new_box(format!(
                "datastore lock error: {}",
                lock_err
            )));
        }

        ds.unwrap().insert(item.uuid, data);

        Ok(())
    }

    fn get_subscription(&self, uuid: String) -> Result<domain::Subscription, Box<dyn Error>> {
        let svc_span = tracing::info_span!("[InMemDatastore::get_subscription]");
        let _svc_span_guard = svc_span.enter();

        let ds = self.items.lock();

        if let Err(lock_err) = ds {
            return Err(InternalError::new_box(format!(
                "datastore lock error: {}",
                lock_err
            )));
        }

        match ds.unwrap().get(&uuid) {
            Some(data) => {
                let item = serde_json::from_str::<DBSubscription>(data)?;
                let sub = item.to_domain();
                Ok(sub)
            }

            None => Err(NotFoundError::new_box(format!("uuid: {uuid}"))),
        }
    }
}
