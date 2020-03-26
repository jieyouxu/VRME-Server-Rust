//! Authentication middleware.

use crate::auth::auth_payload::AuthPayload;
use crate::auth::errors::AuthError;
use crate::database::ConnectionPool;
use crate::settings::{AuthSettings, Settings};
use actix_web::dev::ServiceRequest;
use actix_web::Error as ActixError;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use base64::DecodeError;
use chrono::{DateTime, NaiveDateTime, Utc};
use deadpool_postgres::Client;
use log::debug;
use serde_json::Error as JsonError;
use std::convert::From;

/// Validator function that filters requests and based on client-provided authentication information
/// (or the lack thereof), decides whether to reject further action (i.e. `401 Unauthorized`) or to
/// pass on the handling to further handlers down the response chain.
///
/// ## Note
///
/// - If the `auth_token` associated with the `uuid` exists, but is outdated, the request will be
///   rejected as the client shall `POST /login` again.
/// - If the `auth_token` associated with the `uuid` exists and has not expired yet, the request
///   will be accepted AND the `auth_token`'s validity duration will be refreshed.
pub async fn identity_validator(
	req: ServiceRequest,
	credentials: BearerAuth,
) -> Result<ServiceRequest, ActixError> {
	let auth_settings = req.app_data::<Settings>().unwrap();
	let auth_payload = match deserialize_payload(credentials.token()) {
		Ok(payload) => payload,
		Err(e) => {
			return Err(e.into());
		}
	};

	validate_auth_payload(&auth_settings.get_ref().auth, &auth_payload)?;

	let pool = req.app_data::<ConnectionPool>().unwrap();
	let client = match pool.get().await {
		Ok(client) => client,
		Err(e) => {
			return Err(AuthError::from(e).into());
		}
	};

	let last_used = match find_auth_session(&client, &auth_payload).await {
		Ok(last_used) => last_used,
		Err(e) => {
			return Err(e.into());
		}
	};

	let time_since = Utc::now().signed_duration_since(last_used).num_hours();

	if time_since > auth_settings.auth.auth_token_validity_duration as i64 {
		Err(AuthError::AuthTokenExpired(
			"`auth_token` has expired; login again".to_string(),
		)
		.into())
	} else {
		Ok(req)
	}
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

fn validate_auth_payload(
	auth_settings: &AuthSettings,
	auth_payload: &AuthPayload,
) -> Result<(), AuthError> {
	if auth_payload.auth_token.trim().is_empty() {
		return Err(AuthError::InvalidFormat(
			"`auth_token` cannot be empty".to_string(),
		));
	}

	if auth_payload.auth_token.len() != auth_settings.auth_token_length as usize
	{
		return Err(AuthError::InvalidFormat(
			"Invalid AuthPayload".to_string(),
		));
	}

	Ok(())
}

const FIND_AUTH_SESSION_QUERY: &str = r#"
    SELECT
        user_id,
        auth_token,
        last_used
    FROM auth_sessions
    WHERE
        user_id = $1::UUID,
        auth_token = $2::VARCHAR(32);
"#;

async fn find_auth_session(
	client: &Client,
	auth_info: &AuthPayload,
) -> Result<DateTime<Utc>, AuthError> {
	let statement = client.prepare(FIND_AUTH_SESSION_QUERY).await.unwrap();

	let rows = client
		.query(&statement, &[&auth_info.uuid, &auth_info.auth_token])
		.await
		.map_err(|e| AuthError::InternalServerError(e.to_string()))?;

	if rows.is_empty() {
		// We did not find a matching auth session.
		return Err(AuthError::InvalidAuthToken(
			"No matching auth session found, try to login again".to_string(),
		));
	} else {
		// We found a matching auth session.
		let last_used: NaiveDateTime = rows[0].get(2);
		// We store all datetimes in `UTC+0` timezone.
		let last_used = DateTime::from_utc(last_used, Utc);
		return Ok(last_used);
	}
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

impl From<AuthError> for ActixError {
	fn from(e: AuthError) -> Self {
		Self { cause: Box::new(e) }
	}
}
