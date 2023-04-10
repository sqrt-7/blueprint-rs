use super::error::ServiceError;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    uuid: crate::Uuid,
    email: Email,
    name: UserName,
}

impl User {
    pub fn new(uuid: crate::Uuid, email: Email, name: UserName) -> Self {
        User { uuid, email, name }
    }

    pub fn uuid(&self) -> &crate::Uuid {
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
    uuid: crate::Uuid,
    title: JournalTitle,
    year: JournalYear,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Subscription {
    user_id: crate::Uuid,
    journal_id: crate::Uuid,
}

impl Subscription {
    pub fn new(user_id: crate::Uuid, journal_id: crate::Uuid) -> Self {
        Subscription { user_id, journal_id }
    }

    pub fn user_id(&self) -> &crate::Uuid {
        &self.user_id
    }

    pub fn journal_id(&self) -> &crate::Uuid {
        &self.journal_id
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Email(String);

impl Email {}

impl TryFrom<String> for Email {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // todo
        Ok(Email(value))
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserName(String);

impl UserName {}

impl TryFrom<String> for UserName {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // todo
        Ok(UserName(value))
    }
}

impl ToString for UserName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JournalTitle(String);

impl TryFrom<String> for JournalTitle {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // todo
        Ok(JournalTitle(value))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JournalYear(u32);

impl TryFrom<u32> for JournalYear {
    type Error = ServiceError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        // todo
        Ok(JournalYear(value))
    }
}
