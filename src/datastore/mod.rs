pub mod inmem;

pub trait Datastore: Send + Sync {
    fn store_subscription(&self, sub: Subscription) -> Result<(), String>;
    fn get_subscription(&self, uuid: String) -> Result<Subscription, String>;
}
#[derive(Clone)]
pub struct Subscription {
    pub uuid: String,
    pub name: String,
    pub email: String,
}
