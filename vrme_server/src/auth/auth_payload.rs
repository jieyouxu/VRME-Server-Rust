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

const INVALID_AUTH_TOKEN_LEN_ERR_MSG: &str =
	"The `auth_token` must be base64-encoded and have a length of `44` base64 characters";

// Try convert `BearerAuth` into `AuthPayload` by base64-decoding token and deserializing it as the
// `AuthPayload` json.
impl AuthPayload {
	pub fn from_bearer_auth(auth_info: &BearerAuth) -> Result<AuthPayload, ServiceError> {
		let auth_info = auth_info.token();
		let raw = base64::decode(auth_info)?;
		let auth_payload = serde_json::from_slice::<AuthPayload>(raw.as_slice())?;

		if auth_payload.auth_token.len() != BASE64_ENCODED_AUTH_TOKEN_LEN {
			return Err(ServiceError::Unauthorized(
				INVALID_AUTH_TOKEN_LEN_ERR_MSG.to_string(),
			));
		}

		Ok(auth_payload)
	}
}

/// Length of an `base64`-encoded `auth_token`.
///
/// An `auth_token` is 32-bytes in raw binary form. To encode this as proper UTF-8 when transmitting
/// over the network, we encode the 32 bytes with `base64` encoding.
///
/// The minimum `base64` characters required to encode `32` bytes is `4 * 32 / 3` which rounds up to
/// `43`. However, `base64` requires the length to be a multiple of `4` so we round `43` up to the
/// next multiple of `4` which is `44`.
pub const BASE64_ENCODED_AUTH_TOKEN_LEN: usize = 44;
