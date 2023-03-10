use std::fmt::{Debug, Display};

use actix_web::{
    http::{self, header::ContentType},
    HttpResponse, ResponseError,
};

pub const CODE_SUB_NOT_FOUND: &str = "CODE_SUB_NOT_FOUND";
pub const CODE_SUB_CREATE_FAIL: &str = "CODE_SUB_CREATE_FAIL";
pub const CODE_UNEXPECTED: &str = "CODE_UNEXPECTED";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServiceError {
    error_type: ServiceErrorType,
    code: String,

    #[serde(skip_serializing, skip_deserializing)]
    internal_msg: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ServiceErrorType {
    Internal,
    Validation,
    NotFound,
}

impl ServiceError {
    pub fn new(msg: &str) -> Self {
        ServiceError {
            error_type: ServiceErrorType::Internal,
            code: msg.to_string(),
            internal_msg: String::new(),
        }
    }

    pub fn with_internal(mut self, internal_msg: &str) -> Self {
        self.internal_msg = internal_msg.to_string();
        self
    }

    pub fn with_type(mut self, error_type: ServiceErrorType) -> Self {
        self.error_type = error_type;
        self
    }

    pub fn error_type(&self) -> ServiceErrorType {
        self.error_type.clone()
    }

    pub fn msg(&self) -> String {
        self.code.clone()
    }

    pub fn internal_msg(&self) -> String {
        self.internal_msg.clone()
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self.error_type {
            ServiceErrorType::Internal => http::StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorType::Validation => http::StatusCode::BAD_REQUEST,
            ServiceErrorType::NotFound => http::StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self)
    }
}

impl Debug for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServiceError{{ type: {}, code: {}, internal_msg: {} }}",
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
