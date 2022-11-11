use std::{collections::HashMap, error::Error, sync::Mutex};

use crate::datastore::{InternalError, NotFoundError};

use super::{Datastore, Subscription};

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
    fn store_subscription(&self, sub: &Subscription) -> Result<(), Box<dyn Error>> {
        tracing::info!("[InMemDatastore::store_subscription]");

        let ds = self.items.lock();

        if let Err(lock_err) = ds {
            return Err(InternalError::new_box(format!(
                "datastore lock error: {}",
                lock_err
            )));
        }

        let json = serde_json::to_string(sub)?;
        ds.unwrap().insert(sub.uuid.clone(), json);

        Ok(())
    }

    fn get_subscription(&self, uuid: String) -> Result<Subscription, Box<dyn Error>> {
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
            Some(json) => {
                let sub = serde_json::from_str::<Subscription>(json)?;
                Ok(sub)
            }

            None => Err(NotFoundError::new_box(format!("uuid: {uuid}"))),
        }
    }
}
