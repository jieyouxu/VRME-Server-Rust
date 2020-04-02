//! Authentication middleware.

use crate::auth::auth_payload::AuthPayload;
use crate::auth::errors::AuthError;
use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;
use crate::settings::Settings;
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
	let settings = req.app_data::<Settings>().unwrap();
	let auth_payload = AuthPayload::from_bearer_auth(&credentials)?;

	debug!("Received `auth_payload` {:#?}", &auth_payload);

	let pool = req.app_data::<PersistentConnectionPool>().unwrap();
	let client = pool.get().await?;

	let last_used = match find_auth_session(&client, &auth_payload).await {
		Ok(last_used) => last_used,
		Err(_) => {
			return Err(ServiceError::Unauthorized(
				"No matching auth session found with the given `uuid`".to_string(),
			)
			.into())
		}
	};
	let time_since = Utc::now().signed_duration_since(last_used).num_hours();

	if time_since > settings.auth.auth_token_validity_duration as i64 {
		Err(AuthError::AuthTokenExpired("`auth_token` has expired; login again".to_string()).into())
	} else {
		Ok(req)
	}
}

const FIND_AUTH_SESSION_QUERY: &str = r#"
    SELECT
        user_id,
        auth_token,
        last_used
    FROM auth_sessions
    WHERE
        user_id = $1::UUID AND
        auth_token = $2::VARCHAR(44);
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
