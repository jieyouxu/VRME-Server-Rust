//! Client-facing errors.

use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

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

	#[error("unauthorized")]
	Unauthorized,
}

// Facilitates automatic conversion from `ServiceError` into HTTP error responses in JSON.
impl ResponseError for ServiceError {
	fn error_response() -> HttpResponse {
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
