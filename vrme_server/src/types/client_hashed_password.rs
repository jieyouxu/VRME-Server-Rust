//! Represents a client-side hashed password. This password is base64-encoded to be transmitted
//! across the network and is hashed by the client.

use crate::service_errors::ServiceError;

/// Base64-encoded client-side password hash.
pub struct ClientHashedPassword {
	encoded_hash: String,
}

/// `32` bytes of password needs `(4 * 44) / 3).ceil() == 43` base64 characters to encode, and needs
/// to be rounded up to `44` which is the next multiple of `4`.
pub const BASE64_ENCODED_HASHED_PASSWORD_LEN: usize = 44;

impl ClientHashedPassword {
	pub fn new(raw: &str) -> Result<Self, ServiceError> {
		if raw.trim().is_empty() {
			return Err(ServiceError::BadRequest(
				"Hashed password cannot be empty".to_string(),
			));
		}

		if raw.len() != BASE64_ENCODED_HASHED_PASSWORD_LEN {
			return Err(ServiceError::BadRequest(format!(
				"Invalid base64-encoded hash length: expected {}, got {}",
				BASE64_ENCODED_HASHED_PASSWORD_LEN,
				raw.len()
			)));
		}

		Ok(Self {
			encoded_hash: raw.to_string(),
		})
	}

	/// Decode the base64-encoded client-side hashed password to give the client-side hashed
	/// password.
	pub async fn decode(&self) -> Result<String, ServiceError> {
		let raw = base64::decode(&self.encoded_hash)?;
		let hashed_password = std::str::from_utf8(&raw)?;
		Ok(hashed_password.to_owned())
	}
}
