use std::{collections::HashMap, sync::Mutex};

use super::{Datastore, Subscription};

pub struct InMemDatastore {
    items: Mutex<HashMap<String, Subscription>>,
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
    fn store_subscription(&self, sub: Subscription) -> Result<(), String> {
        let mut ds = self.items.lock().unwrap();
        ds.insert(sub.uuid.clone(), sub);
        Ok(())
    }

    fn get_subscription(&self, uuid: String) -> Result<Subscription, String> {
        let ds = self.items.lock().unwrap();
        match ds.get(&uuid) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("item {} not found", uuid)),
        }
    }
}
