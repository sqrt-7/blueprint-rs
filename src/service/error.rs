use std::fmt::{Debug, Display};

use actix_web::{
    http::{self, header::ContentType},
    HttpResponse, ResponseError,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServiceError {
    error_type: ServiceErrorType,
    msg: String,

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
            msg: msg.to_string(),
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

    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn internal_msg(&self) -> &str {
        self.internal_msg.as_ref()
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
            "ServiceError{{ type: {}, msg: {}, internal_msg: {} }}",
            self.error_type, self.msg, self.internal_msg
        )
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServiceError{{ type: {}, msg: {} }}",
            self.error_type, self.msg
        )
    }
}

impl Display for ServiceErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
