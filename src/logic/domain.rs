use super::error::{ServiceError, ServiceErrorCode, ServiceErrorType};
use crate::proto;
use std::fmt::Display;
use uuid::Uuid as uuid_bytes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ID(String);

impl ID {
    pub fn new() -> Self {
        ID(uuid_bytes::new_v4().to_string())
    }
}

impl TryFrom<&str> for ID {
    type Error = ServiceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match uuid_bytes::parse_str(value) {
            Ok(raw) => Ok(ID(raw.to_string())),
            Err(_) => Err(ServiceError::new(ServiceErrorCode::InvalidID)
                .with_type(ServiceErrorType::InvalidArgument)
                .with_internal_msg(format!("invalid id: {}", value))),
        }
    }
}

impl TryFrom<String> for ID {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ID::try_from(value.as_str())
    }
}

impl Default for ID {
    fn default() -> Self {
        ID::new()
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    id: ID,
    email: Email,
    name: UserName,
}

impl User {
    pub fn new(id: ID, email: Email, name: UserName) -> Self {
        User {
            id,
            email,
            name,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
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
        let id = ID::try_from(value.id)?;
        let email = Email::try_from(value.email)?;
        let name = UserName::try_from(value.name)?;

        Ok(User {
            id,
            email,
            name,
        })
    }
}

impl From<User> for proto::User {
    fn from(val: User) -> Self {
        proto::User {
            id: val.id.0,
            name: val.name.0,
            email: val.email.0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Journal {
    id: ID,
    title: JournalTitle,
    year: JournalYear,
}

impl Journal {
    pub fn new(id: ID, title: JournalTitle, year: JournalYear) -> Self {
        Journal {
            id,
            title,
            year,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
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
        let id = ID::try_from(value.id)?;
        let title = JournalTitle::try_from(value.title)?;
        let year = JournalYear::try_from(value.year)?;

        Ok(Journal {
            id,
            title,
            year,
        })
    }
}

impl From<Journal> for proto::Journal {
    fn from(val: Journal) -> Self {
        proto::Journal {
            id: val.id.0,
            title: val.title.0,
            year: val.year.0,
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
    id: ID,
    user_id: ID,
    journal_id: ID,
}

impl Subscription {
    pub fn new(id: ID, user_id: ID, journal_id: ID) -> Self {
        Subscription {
            id,
            user_id,
            journal_id,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn user_id(&self) -> &ID {
        &self.user_id
    }

    pub fn journal_id(&self) -> &ID {
        &self.journal_id
    }
}

impl From<Subscription> for proto::Subscription {
    fn from(val: Subscription) -> Self {
        proto::Subscription {
            id: val.id.0,
            user_id: val.user_id.0,
            journal_id: val.journal_id.0,
        }
    }
}

impl From<Vec<Subscription>> for proto::SubscriptionList {
    fn from(val: Vec<Subscription>) -> Self {
        let mut res = proto::SubscriptionList {
            items: Vec::new(),
        };

        for v in val.into_iter() {
            res.items.push(v.into());
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
