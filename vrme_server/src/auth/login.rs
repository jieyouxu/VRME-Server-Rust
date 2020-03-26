//! Handles user login and `auth_token` issuing.

use crate::auth::auth_token::{AuthToken, AUTH_TOKEN_LEN};
use crate::database::ConnectionPool;
use crate::service_errors::ServiceError;
use actix_web::web;
use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
pub async fn login_handler(
	pool: web::Data<ConnectionPool>,
	login_info: web::Data<LoginInfo>,
) -> HttpResponse {
	let client = match pool.get().await {
		Ok(client) => client,
		Err(e) => return e.error_response(),
	};

	let uuid = match get_uuid_by_email(&client, &login_info.email).await {
		Ok(uuid) => uuid,
		Err(e) => return e.error_response(),
	};

	let auth_token = match upsert_auth_session(&client, &uuid).await {
		Ok(auth_token) => auth_token,
		Err(e) => return e.error_response(),
	};

	// We need to base64-encode the raw `auth_token` bytes.
	let auth_token = base64::encode(auth_token);

	make_success_response(&uuid, &auth_token)
}

const GET_UUID_BY_EMAIL_QUERY: &str = r#"
    SELECT
        user_id,
        email
    FROM accounts
    WHERE
        user_id = $1::UUID
    ;
"#;

async fn get_uuid_by_email(
	client: &Client,
	email: &str,
) -> Result<Uuid, ServiceError> {
	let statement = client.prepare(GET_UUID_BY_EMAIL_QUERY).await.unwrap();
	let row = client.query_one(&statement, &[&email]).await?;

	let uuid: Uuid = row.get(0);

	Ok(uuid)
}

const UPSERT_AUTH_SESSION_QUERY: &str = r#"
    INSERT INTO auth_sessions
        (user_id, auth_token, last_used)
    VALUES
        ($1::UUID, $2::BYTEA, $3::TIMESTAMP)
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
async fn upsert_auth_session(
	client: &Client,
	user_id: &Uuid,
) -> Result<[u8; AUTH_TOKEN_LEN], ServiceError> {
	let auth_token = AuthToken::new().await?.token();
	let auth_token_ref = &auth_token[..AUTH_TOKEN_LEN];
	let last_used = chrono::Utc::now().naive_utc();

	let statement = client.prepare(UPSERT_AUTH_SESSION_QUERY).await.unwrap();

	let rows = client
		.query(&statement, &[user_id, &auth_token_ref, &last_used])
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
