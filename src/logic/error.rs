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
pub struct LogicError {
    code: LogicErrorCode,

    #[serde(skip)]
    internal_msg: Option<String>,

    #[serde(skip)]
    wrapped: Option<Box<dyn Error>>, // wrapped error
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub enum LogicErrorCode {
    UnexpectedError,
    InvalidID,
    DuplicateEmail,
    UserNotFound,
    UserInvalidData,
}

impl LogicError {
    pub fn new(code: LogicErrorCode) -> Self {
        LogicError {
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

    pub fn code(&self) -> LogicErrorCode {
        self.code
    }
}

// (HTTP) Convert ServiceError to actix_web response
impl ResponseError for LogicError {
    fn status_code(&self) -> http::StatusCode {
        match self.code {
            LogicErrorCode::UnexpectedError => http::StatusCode::INTERNAL_SERVER_ERROR,
            LogicErrorCode::InvalidID => http::StatusCode::BAD_REQUEST,
            LogicErrorCode::DuplicateEmail => http::StatusCode::CONFLICT,
            LogicErrorCode::UserNotFound => http::StatusCode::NOT_FOUND,
            LogicErrorCode::UserInvalidData => http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self)
    }
}

// (GRPC) Convert ServiceError to tonic::Status
impl From<LogicError> for Status {
    fn from(val: LogicError) -> Self {
        let grpc_code = match val.code {
            LogicErrorCode::UnexpectedError => Code::Internal,
            LogicErrorCode::InvalidID => Code::InvalidArgument,
            LogicErrorCode::DuplicateEmail => Code::AlreadyExists,
            LogicErrorCode::UserNotFound => Code::NotFound,
            LogicErrorCode::UserInvalidData => Code::InvalidArgument,
        };

        Status::new(grpc_code, val.code)
    }
}

impl Display for LogicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LogicError{{ code: {} }}", self.code)
    }
}

impl Error for LogicError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.wrapped.as_deref()
    }
}

impl Display for LogicErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<LogicErrorCode> for String {
    fn from(value: LogicErrorCode) -> Self {
        format!("{:?}", value)
    }
}
