#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Subscription {
    uuid: String,
    email: String,
    name: String,
}

impl Subscription {
    pub fn new(uuid: String, email: String, name: String) -> Self {
        Subscription { uuid, email, name }
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }

    pub fn uuid(&self) -> &str {
        self.uuid.as_ref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
