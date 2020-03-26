//! Authentication middleware.

use crate::auth::auth_payload::AuthPayload;
use crate::auth::errors::AuthError;
use crate::settings::AuthSettings;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{
	BearerAuth, Config as BearerConfig,
};
use actix_web_httpauth::extractors::{
	AuthExtractorConfig, AuthenticationError,
};
use base64::DecodeError;
use log::debug;
use serde_json::Error as JsonError;
use std::convert::From;

/// Validator function that filters requests and based on client-provided authentication information
/// (or the lack thereof), decides whether to reject further action (i.e. `401 Unauthorized`) or to
/// pass on the handling to further handlers down the response chain.
pub async fn identity_validator(
	req: ServiceRequest,
	credentials: BearerAuth,
) -> Result<ServiceRequest, AuthError> {
	let auth_payload = deserialize_payload(credentials.token())?;

	unimplemented!()
}

fn deserialize_payload(base64_encoded: &str) -> Result<AuthPayload, AuthError> {
	// We get the `AuthPayload` contained within the `Authorization: Bearer <token>` header, which
	// is required to be base64-encoded.
	let encoded_auth_payload = base64_encoded;
	// We decode the base64-encoded `AuthPayload` to get the raw `AuthPayload` (i.e. the
	// `AuthPayload` JSON in bytes.
	let raw_auth_payload = base64::decode(encoded_auth_payload)?;
	// We attempt to deserialize the `AuthPayload` as JSON.
	let auth_payload =
		serde_json::from_slice::<AuthPayload>(&raw_auth_payload[..])?;

	debug!("Received `AuthPayload`: {:?}", &auth_payload);

	Ok(auth_payload)
}

impl From<DecodeError> for AuthError {
	fn from(e: DecodeError) -> Self {
		Self::InvalidFormat(e.to_string())
	}
}

impl From<JsonError> for AuthError {
	fn from(e: JsonError) -> Self {
		Self::InvalidFormat(e.to_string())
	}
}
