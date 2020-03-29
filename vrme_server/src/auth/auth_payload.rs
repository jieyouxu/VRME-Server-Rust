//! Authentication payload.

use crate::service_errors::ServiceError;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

/// Required authentication payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
	/// An user's unique identifer. If the client lost this information, they can retreive it using
	/// the `GET /account/uuid` endpoint.
	pub uuid: uuid::Uuid,
	/// Authentication token issued by the server whena user successfully registered via
	/// `POST /register` and logs in successfully via `POST /login`.
	pub auth_token: String,
}

// Try convert `BearerAuth` into `AuthPayload` by base64-decoding token and deserializing it as the
// `AuthPayload` json.
impl AuthPayload {
	pub fn from_bearer_auth(auth_info: &BearerAuth) -> Result<AuthPayload, ServiceError> {
		let auth_info = auth_info.token();
		let raw = base64::decode(auth_info)?;
		let auth_payload = serde_json::from_slice(raw.as_slice())?;
		Ok(auth_payload)
	}
}
