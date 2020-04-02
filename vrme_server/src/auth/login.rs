//! Handles user login and `auth_token` issuing.

use crate::auth::auth_token::AuthToken;
use crate::database::postgresql::ConnectionPool;
use crate::service_errors::ServiceError;
use crate::types::client_hashed_password::ClientHashedPassword;
use crate::types::hashed_password::{PBKDF2_ALGORITHM, PBKDF2_ITERATIONS};
use actix_web::error::BlockingError;
use actix_web::web;
use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::Client;
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::num::NonZeroU32;
use uuid::Uuid;

/// Required login payload – the user needs to login with their `email` and `hashed_password`.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginInfo {
	/// User email.
	pub email: String,
	/// base64-encoded client-side-hashed password. Must be exactly `44` base64 characters.
	pub hashed_password: String,
}

/// User login handler.
///
/// ## Success Response
///
/// Upon successful login, the client receives:
///
/// ```json
/// {
///     "uuid": "xxxx-xxxxxxx-xxxxxx",
///     "auth_token": "xxxxxxxxxxxxxxxxxx"
/// }
/// ```
///
/// This is the definition of `crate::auth::auth_payload::AuthPayload`.
///
/// ## Important Side Effect
///
/// A successful login will cause the `auth_token` used for the login to be refreshed in
/// terms of its `last_used` datetime.
pub async fn handle_login(
	pool: web::Data<ConnectionPool>,
	login_info: web::Json<LoginInfo>,
) -> HttpResponse {
	let client = match pool.get().await {
		Ok(client) => client,
		Err(e) => return e.error_response(),
	};

	let uuid =
		match check_registration(&client, &login_info.email, &login_info.hashed_password).await {
			Ok(uuid) => uuid,
			Err(e) => return e.error_response(),
		};

	let auth_token = match upsert_auth_session(&client, &uuid).await {
		Ok(auth_token) => auth_token,
		Err(e) => return e.error_response(),
	};

	make_success_response(&uuid, &auth_token)
}

const GET_PREVIOUS_HASH_QUERY: &str = r#"
    SELECT
        user_id,
        password_hash,
        salt
    FROM accounts
    WHERE
        email = $1::VARCHAR(355)
    ;
"#;

async fn check_registration(
	client: &Client,
	email: &str,
	client_hash: &str,
) -> Result<Uuid, ServiceError> {
	let client_hash = ClientHashedPassword::new(client_hash)?.decode().await?;

	let statement = client.prepare(GET_PREVIOUS_HASH_QUERY).await.unwrap();

	let row = match client.query_one(&statement, &[&email]).await {
		Ok(row) => row,
		Err(_) => {
			return Err(ServiceError::Unauthorized(
				"The email and password combination is invalid".to_string(),
			));
		}
	};

	let uuid: Uuid = row.get(0);
	let previously_derived: Vec<u8> = row.get(1);
	let salt: Vec<u8> = row.get(2);

	web::block(move || {
		pbkdf2::verify(
			PBKDF2_ALGORITHM,
			NonZeroU32::new(PBKDF2_ITERATIONS as u32).unwrap(),
			&salt,
			&client_hash,
			&previously_derived,
		)
	})
	.await
	.map_err(|e| match e {
		BlockingError::Error(e) => e.into(),
		BlockingError::Canceled => {
			ServiceError::InternalServerError("Unexpectedly cancelled".to_string())
		}
	})?;

	Ok(uuid)
}

const UPSERT_AUTH_SESSION_QUERY: &str = r#"
    INSERT INTO auth_sessions
        (user_id, auth_token, last_used)
    VALUES
        ($1::UUID, $2::VARCHAR(44), $3::TIMESTAMP)
    ON CONFLICT
        (user_id)
    DO UPDATE SET
        auth_token = EXCLUDED.auth_token,
        last_used = EXCLUDED.last_used
    RETURNING
        user_id,
        auth_token,
        last_used
    ;
"#;

/// Either creates a new `auth_session`, or refreshes an existing session with a new `auth_token`.
async fn upsert_auth_session(client: &Client, user_id: &Uuid) -> Result<String, ServiceError> {
	let auth_token = AuthToken::new().await?.token();
	let auth_token = base64::encode(&auth_token);
	let last_used = chrono::Utc::now().naive_utc();

	let statement = client.prepare(UPSERT_AUTH_SESSION_QUERY).await.unwrap();

	let rows = client
		.query(&statement, &[user_id, &auth_token, &last_used])
		.await?;

	if rows.is_empty() {
		Err(ServiceError::InternalServerError(
			"Failed to create new auth session".to_string(),
		))
	} else {
		Ok(auth_token)
	}
}

fn make_success_response(user_id: &Uuid, auth_token: &str) -> HttpResponse {
	let message = json!({
		"user_id": user_id,
		"auth_token": auth_token
	});

	HttpResponse::Created().json(message)
}
