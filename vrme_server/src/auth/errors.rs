//! Authentication errors.

use actix_web::HttpResponse;
use serde_json::json;
use std::convert::From;

/// Authenitcation errors.
pub enum AuthError {
	/// Missing required authentication payload.
	MissingCredentials(String),
	/// The provided authentication payload is not base64-encoded or is not in valid JSON format.
	InvalidFormat(String),
	/// The provided `auth_token` has already expired; the client needs to login via `POST /login`
	/// again.
	AuthTokenExpired(String),
	/// The client associated with the incoming IP address, `uuid` and/or `auth_token` has been
	/// black-listed by the server.
	Banned(String),
}

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
			AuthError::AuthTokenExpired(ref s) => HttpResponse::Unauthorized()
				.json(make_error_message("token-expired", s)),
			AuthError::Banned(ref s) => {
				HttpResponse::Forbidden().json(make_error_message("banned", s))
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
