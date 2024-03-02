use actix_web::{
    http::{self, header::ContentType},
    HttpResponse, ResponseError,
};
use std::{
    error::Error,
    fmt::{Debug, Display},
};
use tonic::{Code, Status};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServiceError {
    code: ServiceErrorCode,

    #[serde(skip)]
    internal_msg: Option<String>,

    #[serde(skip)]
    wrapped: Option<Box<dyn Error>>, // wrapped error
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub enum ServiceErrorCode {
    UnexpectedError,
    InvalidID,
    DuplicateEmail,
    UserNotFound,
    UserInvalidData,
    JournalNotFound,
    JournalInvalidData,
    SubscriptionNotFound,
    SubscriptionInvalidData,
}

impl ServiceError {
    pub fn new(code: ServiceErrorCode) -> Self {
        ServiceError {
            code,
            internal_msg: None,
            wrapped: None,
        }
    }

    pub fn wrap(mut self, prev: impl Error + 'static) -> Self {
        self.wrapped = Some(Box::new(prev));
        self
    }

    pub fn with_internal_msg(mut self, internal_msg: String) -> Self {
        self.internal_msg = Some(internal_msg);
        self
    }

    pub fn code(&self) -> ServiceErrorCode {
        self.code
    }
}

// (HTTP) Convert ServiceError to actix_web response
impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self.code {
            ServiceErrorCode::UnexpectedError => http::StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorCode::InvalidID => http::StatusCode::BAD_REQUEST,
            ServiceErrorCode::DuplicateEmail => http::StatusCode::CONFLICT,
            ServiceErrorCode::UserNotFound => http::StatusCode::NOT_FOUND,
            ServiceErrorCode::UserInvalidData => http::StatusCode::BAD_REQUEST,
            ServiceErrorCode::JournalNotFound => http::StatusCode::NOT_FOUND,
            ServiceErrorCode::JournalInvalidData => http::StatusCode::BAD_REQUEST,
            ServiceErrorCode::SubscriptionNotFound => http::StatusCode::NOT_FOUND,
            ServiceErrorCode::SubscriptionInvalidData => http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self)
    }
}

// (GRPC) Convert ServiceError to tonic::Status
impl From<ServiceError> for Status {
    fn from(val: ServiceError) -> Self {
        let grpc_code = match val.code {
            ServiceErrorCode::UnexpectedError => Code::Internal,
            ServiceErrorCode::InvalidID => Code::InvalidArgument,
            ServiceErrorCode::DuplicateEmail => Code::AlreadyExists,
            ServiceErrorCode::UserNotFound => Code::NotFound,
            ServiceErrorCode::UserInvalidData => Code::InvalidArgument,
            ServiceErrorCode::JournalNotFound => Code::NotFound,
            ServiceErrorCode::JournalInvalidData => Code::InvalidArgument,
            ServiceErrorCode::SubscriptionNotFound => Code::NotFound,
            ServiceErrorCode::SubscriptionInvalidData => Code::InvalidArgument,
        };

        Status::new(grpc_code, val.code)
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceError{{ code: {} }}", self.code)
    }
}

impl Error for ServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.wrapped.as_deref()
    }
}

impl Display for ServiceErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ServiceErrorCode> for String {
    fn from(value: ServiceErrorCode) -> Self {
        format!("{:?}", value)
    }
}
