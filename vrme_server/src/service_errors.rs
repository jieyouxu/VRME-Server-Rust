//! Client-facing errors and various error conversions.

use actix_web::{error::ResponseError, HttpResponse};
use deadpool_postgres;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tokio_pg_mapper;
use tokio_postgres;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
	#[error("Internal Server Error: {0}")]
	InternalServerError(String),

	#[error("Bad Request: {0}")]
	BadRequest(String),
	#[error("Unauthorized")]
	Unauthorized,

	#[error("Forbidden")]
	Forbidden,
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
		}
	}
}

impl From<deadpool_postgres::PoolError> for ServiceError {
	/// Converts `deadpool_postgres::PoolError` into `ServiceError::InternalServerError`.
	fn from(error: deadpool_postgres::PoolError) -> Self {
		use deadpool_postgres::PoolError;

		match &error {
			PoolError::Timeout(_) => Self::InternalServerError(
				"Connection pool timed out".to_string(),
			),
			PoolError::Backend(ref e) => Self::InternalServerError(format!(
				"Database error: {}",
				e.to_string()
			)),
		}
	}
}

impl From<tokio_pg_mapper::Error> for ServiceError {
	/// Converts from `tokio_pg_mapper::tokio_pg_mapper` into `ServiceError::InternalServerError`.
	fn from(error: tokio_pg_mapper::Error) -> Self {
		use tokio_pg_mapper::Error;

		match &error {
			Error::ColumnNotFound => Self::InternalServerError(
				"Database column not found".to_string(),
			),
			Error::Conversion(e) => Self::InternalServerError(format!(
				"Database conversion mapper failed: {}",
				e.to_string()
			)),
		}
	}
}

impl From<tokio_postgres::error::Error> for ServiceError {
	/// Converts from `tokio_postgres::error::Error` into `ServiceError::InternalServerError`.
	fn from(error: tokio_postgres::error::Error) -> Self {
		Self::InternalServerError(format!("Database error: {:?}", error))
	}
}
