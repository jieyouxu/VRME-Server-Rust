//! Errors and various error conversions.

use actix_web::error::BlockingError;
use actix_web::{error::ResponseError, HttpResponse};
use base64::DecodeError;
use deadpool_postgres::PoolError;
use derive_more::Display;
use rand::Error as RandError;
use serde::Serialize;
use serde_json::json;
use serde_json::{error::Category as JsonErrorCategory, Error as JsonError};
use std::convert::{From, Into};
use tokio_pg_mapper::Error as TPGMError;
use tokio_postgres::error::Error as TPGError;

/// Client-facing service errors.
#[derive(Debug, Display, Serialize, PartialEq)]
pub enum ServiceError {
	#[display(fmt = "internal server error: {}", "_0")]
	InternalServerError(String),

	#[display(fmt = "bad bequest: {}", "_0")]
	BadRequest(String),

	#[display(fmt = "unauthorized: {}", "_0")]
	Unauthorized(String),

	#[display(fmt = "forbidden: {}", "_0")]
	Forbidden(String),

	#[display(fmt = "conflict: {}", "_0")]
	Conflict(String),

	#[display(fmt = "unprocessable entity: {}", "_0")]
	UnprocessableEntity(String),

	#[display(fmt = "unsupported media type: {}", "_0")]
	UnsupportedMediaType(String),

	#[display(fmt = "not found: {}", "_0")]
	NotFound(String),
}

impl std::error::Error for ServiceError {}

// Allows easy conversion from `ServiceError -> HttpResponse`.
impl Into<HttpResponse> for ServiceError {
	fn into(self) -> HttpResponse {
		self.error_response()
	}
}

impl From<TPGMError> for ServiceError {
	fn from(e: TPGMError) -> Self {
		Self::InternalServerError(e.to_string())
	}
}

impl From<TPGError> for ServiceError {
	fn from(e: TPGError) -> Self {
		Self::InternalServerError(e.to_string())
	}
}

impl From<PoolError> for ServiceError {
	fn from(e: PoolError) -> Self {
		Self::InternalServerError(e.to_string())
	}
}

impl ResponseError for ServiceError {
	/// Facilitates automatic conversion from `ServiceError` into HTTP error responses in JSON.
	fn error_response(&self) -> HttpResponse {
		match self {
			ServiceError::InternalServerError(ref s) => {
				HttpResponse::InternalServerError().json(json!({
					"cause": "internal-server-error",
					"message": s
				}))
			}
			ServiceError::BadRequest(ref s) => HttpResponse::BadRequest().json(json!({
				"cause": "bad-request",
				"message": s
			})),
			ServiceError::Forbidden(ref s) => HttpResponse::Forbidden().json(json!({
				"cause": "forbidden",
				"message": s
			})),
			ServiceError::Unauthorized(ref s) => HttpResponse::Unauthorized().json(json!({
				"cause": "unauthorized",
				"message": s
			})),
			ServiceError::Conflict(ref s) => HttpResponse::Conflict().json(json!({
				"cause": "conflict",
				"message": s
			})),
			ServiceError::UnprocessableEntity(ref s) => {
				HttpResponse::UnprocessableEntity().json(json!({
					"cause": "unprocessable-entity",
					"message": s
				}))
			}
			ServiceError::UnsupportedMediaType(ref s) => {
				HttpResponse::UnsupportedMediaType().json(json!({
					"cause": "unsupported-media-type",
					"message": s
				}))
			}
			ServiceError::NotFound(ref s) => HttpResponse::NotFound().json(json!({
				"cause": "not-found",
				"message": s
			})),
		}
	}
}

impl From<RandError> for ServiceError {
	fn from(e: RandError) -> Self {
		Self::InternalServerError(e.to_string())
	}
}

impl From<DecodeError> for ServiceError {
	fn from(e: DecodeError) -> Self {
		Self::BadRequest(e.to_string())
	}
}

impl From<std::str::Utf8Error> for ServiceError {
	fn from(e: std::str::Utf8Error) -> Self {
		Self::BadRequest(format!("Invalid UTF-8 byte sequence: {}", e.to_string()))
	}
}

impl From<ring::error::Unspecified> for ServiceError {
	fn from(_: ring::error::Unspecified) -> Self {
		Self::InternalServerError("Error encountered when performing crypto tasks".to_string())
	}
}

impl From<JsonError> for ServiceError {
	fn from(json_error: JsonError) -> Self {
		match &json_error.classify() {
			JsonErrorCategory::Io => Self::InternalServerError(
				"Encountered I/O error when trying to process JSON".to_string(),
			),
			JsonErrorCategory::Syntax | JsonErrorCategory::Eof => {
				Self::UnprocessableEntity("Invalid JSON (syntax error)".to_string())
			}
			JsonErrorCategory::Data => {
				Self::BadRequest("Missing required fields (semantic error)".to_string())
			}
		}
	}
}

impl<E: std::fmt::Debug> From<BlockingError<E>> for ServiceError {
	fn from(e: BlockingError<E>) -> Self {
		Self::InternalServerError(format!("Encountered blocking error: {:?}", e.to_string()))
	}
}

impl From<std::io::Error> for ServiceError {
	fn from(e: std::io::Error) -> Self {
		Self::InternalServerError(format!("I/O error: {}", e.to_string()))
	}
}
