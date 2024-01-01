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
    error_type: ServiceErrorType,
    code: ServiceErrorCode,

    #[serde(skip_serializing, skip_deserializing)]
    internal_msg: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    wrapped: Option<Box<dyn Error>>, // wrapper error
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ServiceErrorCode {
    UnexpectedError,
    InvalidID,
    UserNotFound,
    UserInvalidData,
    JournalNotFound,
    JournalInvalidData,
    SubscriptionNotFound,
    SubscriptionInvalidData,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ServiceErrorType {
    Internal,
    NotFound,
    AlreadyExists,
    InvalidArgument,
    Unauthorized,
    Forbidden,
}

impl ServiceError {
    pub fn new(code: ServiceErrorCode) -> Self {
        ServiceError {
            error_type: ServiceErrorType::Internal,
            code,
            internal_msg: None,
            wrapped: None,
        }
    }

    pub fn wrap(mut self, prev: Box<dyn Error>) -> Self {
        self.wrapped = Some(prev);
        self
    }

    pub fn with_internal_msg(mut self, internal_msg: String) -> Self {
        self.internal_msg = Some(internal_msg);
        self
    }

    pub fn with_type(mut self, error_type: ServiceErrorType) -> Self {
        self.error_type = error_type;
        self
    }

    pub fn error_type(&self) -> ServiceErrorType {
        self.error_type.clone()
    }

    pub fn code(&self) -> ServiceErrorCode {
        self.code.clone()
    }

    pub fn internal_msg(&self) -> &Option<String> {
        &self.internal_msg
    }
}

// (HTTP) Convert ServiceError to actix_web response
impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self.error_type {
            ServiceErrorType::Internal => http::StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorType::NotFound => http::StatusCode::NOT_FOUND,
            ServiceErrorType::AlreadyExists => http::StatusCode::CONFLICT,
            ServiceErrorType::InvalidArgument => http::StatusCode::BAD_REQUEST,
            ServiceErrorType::Unauthorized => http::StatusCode::UNAUTHORIZED,
            ServiceErrorType::Forbidden => http::StatusCode::FORBIDDEN,
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
        let grpc_code = match val.error_type {
            ServiceErrorType::Internal => Code::Internal,
            ServiceErrorType::NotFound => Code::NotFound,
            ServiceErrorType::AlreadyExists => Code::AlreadyExists,
            ServiceErrorType::InvalidArgument => Code::InvalidArgument,
            ServiceErrorType::Unauthorized => Code::Unauthenticated,
            ServiceErrorType::Forbidden => Code::PermissionDenied,
        };

        Status::new(grpc_code, val.code)
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServiceError{{ type: {}, code: {} }}",
            self.error_type, self.code
        )
    }
}

impl Error for ServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.wrapped.as_deref()
    }
}

impl Display for ServiceErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
