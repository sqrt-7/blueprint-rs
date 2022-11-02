pub mod inmem;

pub trait Datastore {
    fn store_subscription(&mut self, sub: Subscription) -> Result<(), String>;
    fn get_subscription(&self, uuid: String) -> Result<&Subscription, String>;
}

pub struct Subscription {
    uuid: String,
    name: String,
    email: String,
}
