use super::error::{ServiceError, ServiceErrorType, CODE_INVALID_UUID};
use crate::proto;
use std::fmt::Display;
use uuid::Uuid as uuid_bytes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Uuid(String);

impl Uuid {
    pub fn new() -> Self {
        Uuid(uuid_bytes::new_v4().to_string())
    }
}

impl TryFrom<String> for Uuid {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match uuid_bytes::parse_str(value.as_str()) {
            Ok(raw) => Ok(Uuid(raw.to_string())),
            Err(_) => Err(ServiceError::new(CODE_INVALID_UUID)
                .with_type(ServiceErrorType::InvalidArgument)
                .with_internal(format!("invalid uuid: {}", value))),
        }
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Uuid::new()
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    uuid: Uuid,
    email: Email,
    name: UserName,
}

impl User {
    pub fn new(uuid: Uuid, email: Email, name: UserName) -> Self {
        User {
            uuid,
            email,
            name,
        }
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

impl TryFrom<proto::User> for User {
    type Error = ServiceError;

    fn try_from(value: proto::User) -> Result<Self, Self::Error> {
        let uuid = Uuid::try_from(value.uuid)?;
        let email = Email::try_from(value.email)?;
        let name = UserName::try_from(value.name)?;

        Ok(User {
            uuid,
            email,
            name,
        })
    }
}

impl Into<proto::User> for User {
    fn into(self) -> proto::User {
        proto::User {
            uuid: self.uuid.0,
            name: self.name.0,
            email: self.email.0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Journal {
    uuid: Uuid,
    title: JournalTitle,
    year: JournalYear,
}

impl Journal {
    pub fn new(uuid: Uuid, title: JournalTitle, year: JournalYear) -> Self {
        Journal {
            uuid,
            title,
            year,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn title(&self) -> &JournalTitle {
        &self.title
    }

    pub fn year(&self) -> &JournalYear {
        &self.year
    }
}

impl TryFrom<proto::Journal> for Journal {
    type Error = ServiceError;

    fn try_from(value: proto::Journal) -> Result<Self, Self::Error> {
        let uuid = Uuid::try_from(value.uuid)?;
        let title = JournalTitle::try_from(value.title)?;
        let year = JournalYear::try_from(value.year)?;

        Ok(Journal {
            uuid,
            title,
            year,
        })
    }
}

impl Into<proto::Journal> for Journal {
    fn into(self) -> proto::Journal {
        proto::Journal {
            uuid: self.uuid.0,
            title: self.title.0,
            year: self.year.0,
        }
    }
}

impl Display for JournalTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Subscription {
    uuid: Uuid,
    user_id: Uuid,
    journal_id: Uuid,
}

impl Subscription {
    pub fn new(uuid: Uuid, user_id: Uuid, journal_id: Uuid) -> Self {
        Subscription {
            uuid,
            user_id,
            journal_id,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn journal_id(&self) -> &Uuid {
        &self.journal_id
    }
}

impl Into<proto::Subscription> for Subscription {
    fn into(self) -> proto::Subscription {
        proto::Subscription {
            uuid: self.uuid.0,
            user_id: self.user_id.0,
            journal_id: self.journal_id.0,
        }
    }
}

impl Into<proto::SubscriptionList> for Vec<Subscription> {
    fn into(self) -> proto::SubscriptionList {
        let mut res = proto::SubscriptionList {
            items: Vec::new(),
        };

        for val in self.into_iter() {
            res.items.push(val.into());
        }

        res
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

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
