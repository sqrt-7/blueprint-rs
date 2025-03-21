use super::{error::LogicError, LogicErrorCode};
use crate::proto;
use std::{fmt::Display, str::FromStr};
use uuid::Uuid as uuid_bytes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ID(String);

impl ID {
    pub fn new() -> Self {
        ID(uuid_bytes::new_v4().to_string())
    }
}

impl TryFrom<&str> for ID {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match uuid_bytes::parse_str(value) {
            Ok(raw) => Ok(ID(raw.to_string())),
            Err(_) => Err(format!("invalid id: {value}")),
        }
    }
}

impl TryFrom<String> for ID {
    type Error = String;

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
        f.write_str(self.0.as_str())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
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

    pub fn try_new(id: &str, email: &str, name: &str) -> Result<Self, LogicError> {
        let parsed_id = match ID::try_from(id) {
            Ok(v) => v,
            Err(e) => return Err(LogicError::new(LogicErrorCode::InvalidID).with_internal_msg(e)),
        };

        let parsed_email = match Email::try_from(email.to_string()) {
            Ok(v) => v,
            Err(e) => {
                return Err(LogicError::new(LogicErrorCode::UserInvalidData).with_internal_msg(e))
            },
        };

        let parsed_name = match UserName::try_from(name.to_string()) {
            Ok(v) => v,
            Err(e) => {
                return Err(LogicError::new(LogicErrorCode::UserInvalidData).with_internal_msg(e))
            },
        };

        Ok(User {
            id: parsed_id,
            email: parsed_email,
            name: parsed_name,
        })
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
    type Error = String;

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

impl From<Vec<User>> for proto::UserList {
    fn from(val: Vec<User>) -> Self {
        let converted: Vec<proto::User> = val
            .into_iter()
            .map(|element| element.into())
            .collect();

        proto::UserList {
            items: converted,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Email(String);

impl Email {}

impl TryFrom<&str> for Email {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if let Err(e) = email_address::EmailAddress::from_str(value) {
            return Err(format!("invalid email: {e}"));
        }
        Ok(Email(value.to_string()))
    }
}

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::try_from(value.as_str())
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct UserName(String);

impl UserName {}

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // todo
        Ok(UserName(value))
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    #[test]
    fn email_valid() {
        let v = "test@foo.com";
        let res = Email::try_from(v);
        assert!(res.is_ok());
        assert_eq!(v, res.unwrap().0);
    }

    #[test]
    fn email_invalid() {
        let res = Email::try_from("not_an_email.com");
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .contains("invalid email: Missing separator character"));
    }

    #[test]
    fn email_valid_trim() {
        let res = Email::try_from("           test@foo.com                     ");
        assert!(res.is_ok());
        assert_eq!("test@foo.com", res.unwrap().0);
    }
}
