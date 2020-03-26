//! An authentication token for verifying identity.

use crate::service_errors::ServiceError;
use serde::{Deserialize, Serialize};

/// Length of an `auth_token` in bytes.
pub const AUTH_TOKEN_LEN: usize = 32;

/// Newtype wrapper around an authentication token.
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
	token: [u8; AUTH_TOKEN_LEN],
}

impl AuthToken {
	/// Generate a new `auth_token` using cryptographically-strong CSPRNG with periodic seeding from
	/// the system entropy.
	pub async fn new() -> Result<Self, ServiceError> {
		use rand::RngCore;

		let mut auth_token = [0u8; AUTH_TOKEN_LEN];
		rand::thread_rng().try_fill_bytes(&mut auth_token)?;

		Ok(Self { token: auth_token })
	}

	/// Extract the token. Consumes the wrapper type.
	pub fn token(self) -> [u8; AUTH_TOKEN_LEN] {
		self.token
	}
}
