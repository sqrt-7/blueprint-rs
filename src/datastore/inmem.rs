use std::collections::HashMap;

use super::{Datastore, Subscription};

pub struct InMemDatastore {
    items: HashMap<String, Subscription>,
}

impl InMemDatastore {
    pub fn new() -> Self {
        InMemDatastore {
            items: HashMap::new(),
        }
    }
}

impl Datastore for InMemDatastore {
    fn store_subscription(&mut self, sub: Subscription) -> Result<(), String> {
        self.items.insert(sub.uuid.clone(), sub);
        Ok(())
    }

    fn get_subscription(&self, uuid: String) -> Result<&Subscription, String> {
        match self.items.get(&uuid) {
            Some(v) => Ok(v),
            None => Err(format!("item {} not found", uuid)),
        }
    }
}
