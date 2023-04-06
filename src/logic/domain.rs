use uuid::Uuid as uuid_bytes;

use super::error::ServiceError;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    uuid: Uuid,
    email: Email,
    name: UserName,
}

impl User {
    pub fn new(uuid: Uuid, email: Email, name: UserName) -> Self {
        User { uuid, email, name }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Journal {
    uuid: Uuid,
    title: JournalTitle,
    year: JournalYear,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Subscription {
    user_id: Uuid,
    journal_id: Uuid,
}

impl Subscription {
    pub fn new(user_id: Uuid, journal_id: Uuid) -> Self {
        Subscription { user_id, journal_id }
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn journal_id(&self) -> &Uuid {
        &self.journal_id
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Uuid(String);

impl Uuid {
    pub fn new() -> Self {
        Uuid(uuid_bytes::new_v4().to_string())
    }

    pub fn try_parse(s: &str) -> Result<Self, String> {
        match uuid_bytes::parse_str(s) {
            Ok(raw) => Ok(Uuid(raw.to_string())),
            Err(_) => Err(format!("invalid uuid: {}", s)),
        }
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Uuid::new()
    }
}

impl ToString for Uuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Email(String);

impl Email {
    pub fn try_parse(s: &str) -> Result<Self, ServiceError> {
        // todo
        Ok(Email(s.to_string()))
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserName(String);

impl UserName {
    pub fn try_parse(s: &str) -> Result<Self, ServiceError> {
        // todo
        Ok(UserName(s.to_string()))
    }
}

impl ToString for UserName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JournalTitle(String);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JournalYear(u32);
