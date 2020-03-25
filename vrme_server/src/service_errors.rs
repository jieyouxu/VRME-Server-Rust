//! Errors and various error conversions.

use actix_web::{error::ResponseError, HttpResponse};
use deadpool_postgres::PoolError;
use derive_more::Display;
use serde::Serialize;
use serde_json::json;
use std::convert::{From, Into};
use tokio_pg_mapper::Error as TPGMError;
use tokio_postgres::error::Error as TPGError;

/// Client-facing service errors.
#[derive(Debug, Display, Serialize)]
pub enum ServiceError {
	#[display(fmt = "internal server error: {}", "_0")]
	InternalServerError(String),

	#[display(fmt = "bad bequest: {}", "_0")]
	BadRequest(String),

	#[display(fmt = "unauthorized")]
	Unauthorized,

	#[display(fmt = "forbidden")]
	Forbidden,

	#[display(fmt = "conflict")]
	Conflict(String),
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
			ServiceError::BadRequest(ref s) => {
				HttpResponse::BadRequest().json(json!({
					"cause": "bad-request",
					"message": s
				}))
			}
			ServiceError::Forbidden => HttpResponse::Forbidden().json(json!({
				"cause": "forbidden",
				"message": "check login details; you may not have sufficient priviledges"
			})),
			ServiceError::Unauthorized => {
				HttpResponse::Unauthorized().json(json!({
					"cause": "unauthorized",
					"message": "attempting to access protected endpoint with invalid credentials"
				}))
			}
			ServiceError::Conflict(ref s) => {
				HttpResponse::Conflict().json(json!({
					"cause": "conflict",
					"message": s
				}))
			}
		}
	}
}
