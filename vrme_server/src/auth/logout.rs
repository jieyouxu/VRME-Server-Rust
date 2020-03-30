//! Logout handler. This deletes the associated authentication session for a user with the given
//! `{uuid}`.

use crate::auth::auth_payload::AuthPayload;
use crate::database::ConnectionPool;
use crate::service_errors::ServiceError;
use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use deadpool_postgres::Client;
use uuid::Uuid;

/// Handles logout. Deletes the user's associated authentication session. Any `auth_token`s issued
/// to that user will be invalidated and the user will need to login again.
pub async fn handle_logout(
	pool: web::Data<ConnectionPool>,
	auth: BearerAuth,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;
	let auth_payload = AuthPayload::from_bearer_auth(&auth)?;
	delete_auth_session(&client, &auth_payload.uuid).await?;
	Ok(HttpResponse::NoContent().finish())
}

const LOGOUT_QUERY: &str = r#"
    DELETE FROM auth_sessions
    WHERE user_id = $1::UUID;
"#;

async fn delete_auth_session(client: &Client, uuid: &Uuid) -> Result<(), ServiceError> {
	let statement = client.prepare(LOGOUT_QUERY).await?;
	client.query(&statement, &[&uuid]).await?;
	Ok(())
}
