use crate::new_error_code;
use actix_web::{
    http::{self, header::ContentType},
    HttpResponse, ResponseError,
};
use std::fmt::{Debug, Display};
use tonic::{Code, Status};

new_error_code!(UNEXPECTED_ERROR);
new_error_code!(DB_ERROR);
new_error_code!(INVALID_ID);
new_error_code!(USER_NOT_FOUND);
new_error_code!(USER_INVALID_DATA);
new_error_code!(JOURNAL_NOT_FOUND);
new_error_code!(JOURNAL_INVALID_DATA);
new_error_code!(SUB_NOT_FOUND);
new_error_code!(SUB_INVALID_DATA);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServiceError {
    error_type: ServiceErrorType,
    code: String,

    #[serde(skip_serializing, skip_deserializing)]
    internal_msg: Option<String>,
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
    pub fn new(code: &str) -> Self {
        ServiceError {
            error_type: ServiceErrorType::Internal,
            code: code.to_string(),
            internal_msg: None,
        }
    }

    pub fn with_internal(mut self, internal_msg: String) -> Self {
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

    pub fn code(&self) -> &String {
        &self.code
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

impl Debug for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServiceError{{ type: {}, code: {}, internal_msg: {:?} }}",
            self.error_type, self.code, self.internal_msg
        )
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

impl Display for ServiceErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
