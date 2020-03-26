//! Authentication errors.

use crate::database::DatabaseError;
use actix_web::HttpResponse;
use serde_json::json;
use std::convert::From;

/// Authenitcation errors.
pub enum AuthError {
	/// Missing required authentication payload.
	MissingCredentials(String),
	/// The provided authentication payload is not base64-encoded or is not in valid JSON format.
	InvalidFormat(String),
	/// The provided `uuid` + `auth_token` payload could not be found in the existing auth session
	/// records.
	InvalidAuthToken(String),
	/// The provided `auth_token` has already expired; the client needs to login via `POST /login`
	/// again.
	AuthTokenExpired(String),
	/// The client associated with the incoming IP address, `uuid` and/or `auth_token` has been
	/// black-listed by the server.
	Banned(String),
	/// Internal server error related to authentication.
	InternalServerError(String),
}

impl std::error::Error for AuthError {}

impl From<AuthError> for HttpResponse {
	/// Transforms from a `AuthError` to a `HttpResonse`.
	fn from(e: AuthError) -> Self {
		match e {
			AuthError::MissingCredentials(ref s) => {
				HttpResponse::Unauthorized()
					.json(make_error_message("missing-credentials", s))
			}
			AuthError::InvalidFormat(ref s) => HttpResponse::Unauthorized()
				.json(make_error_message("invalid-format", s)),
			AuthError::InvalidAuthToken(ref s) => HttpResponse::Unauthorized()
				.json(make_error_message("invalid-auth-token", s)),
			AuthError::AuthTokenExpired(ref s) => HttpResponse::Unauthorized()
				.json(make_error_message("token-expired", s)),
			AuthError::Banned(ref s) => {
				HttpResponse::Forbidden().json(make_error_message("banned", s))
			}
			AuthError::InternalServerError(ref s) => {
				HttpResponse::InternalServerError()
					.json(make_error_message("internal-server-error", s))
			}
		}
	}
}

/// Template a JSON error response.
fn make_error_message(cause: &str, message: &str) -> serde_json::value::Value {
	json!({
		"cause": cause,
		"message": message
	})
}

impl From<DatabaseError> for AuthError {
	fn from(e: DatabaseError) -> Self {
		AuthError::InternalServerError(e.to_string())
	}
}
