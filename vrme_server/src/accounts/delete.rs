//! Deletes user account.

use crate::auth::auth_payload::AuthPayload;
use crate::database::postgresql::ConnectionPool;
use crate::service_errors::ServiceError;
use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use deadpool_postgres::Client;
use futures::future;
use tokio_postgres::{Error as TPError, Statement};
use uuid::Uuid;

/// Handler for deleting a user account. This is a destructive operation and the user account cannot
/// be recovered.
pub async fn handle_delete_account(
	pool: web::Data<ConnectionPool>,
	auth: BearerAuth,
) -> Result<HttpResponse, Error> {
	let auth_payload = AuthPayload::from_bearer_auth(&auth)?;
	let client = pool.get().await?;
	delete_user_account(&client, &auth_payload.uuid).await?;
	Ok(HttpResponse::NoContent().finish())
}

const DELETE_ACCOUNT_QUERY: &str = r#"
    DELETE FROM accounts
    WHERE user_id = $1::UUID;
"#;

const DELETE_AUTH_SESSION_QUERY: &str = r#"
    DELETE FROM auth_sessions
    WHERE user_id = $1::UUID;
"#;

async fn delete_user_account(client: &Client, uuid: &Uuid) -> Result<(), ServiceError> {
	let (acc_statement, auth_statement) = pipelined_prepare(client).await?;

	future::try_join(
		client.query(&acc_statement, &[&uuid]),
		client.query(&auth_statement, &[&uuid]),
	)
	.await?;

	Ok(())
}

async fn pipelined_prepare(client: &Client) -> Result<(Statement, Statement), TPError> {
	future::try_join(
		client.prepare(DELETE_ACCOUNT_QUERY),
		client.prepare(DELETE_AUTH_SESSION_QUERY),
	)
	.await
}
