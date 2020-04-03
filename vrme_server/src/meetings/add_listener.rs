//! Add a listener to a meeting session.

use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::auth_payload::AuthPayload;
use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddListenerRequestPayload {
	pub listener: Uuid,
}

pub async fn handle_add_listener(
	pool: web::Data<PersistentConnectionPool>,
	meeting_id: web::Path<Uuid>,
	auth: BearerAuth,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;
	let user_id = AuthPayload::from_bearer_auth(&auth)?.uuid;

	// We check that the user requesting *is* the `presenter`.
	let mut listeners = get_meeting_session_listeners(&client, &meeting_id, &user_id).await?;

	listeners.push(meeting_id.clone());

	update_meeting_session_listeners(&client, &meeting_id, &user_id, listeners).await?;

	Ok(HttpResponse::NoContent().finish())
}

const GET_MEETING_SESSION_LISTENERS: &str = r#"
    SELECT
        listeners
    FROM meeting_sessions
    WHERE
        meeting_id = $1::UUID AND
        presenter = $2::UUID
    ;
"#;

async fn get_meeting_session_listeners(
	client: &Client,
	meeting_id: &Uuid,
	user_id: &Uuid,
) -> Result<Vec<Uuid>, ServiceError> {
	let statement = client.prepare(GET_MEETING_SESSION_LISTENERS).await?;

	let rows = client.query(&statement, &[meeting_id, user_id]).await?;

	if !rows.is_empty() {
		let listeners = rows[0].get(0);
		Ok(listeners)
	} else {
		Err(ServiceError::NotFound(
			"You are not the presenter of any meeting session".to_string(),
		))
	}
}

const UPDATE_MEETING_SESSION_QUERY: &str = r#"
    UPDATE meeting_sessions
    SET
        listeners = $3::UUID[]
    WHERE
        meeting_id = $1::UUID AND
        presenter = $2::UUID
    ;
"#;

async fn update_meeting_session_listeners(
	client: &Client,
	meeting_id: &Uuid,
	user_id: &Uuid,
	listeners: Vec<Uuid>,
) -> Result<(), ServiceError> {
	let statement = client.prepare(UPDATE_MEETING_SESSION_QUERY).await?;

	client
		.query(&statement, &[meeting_id, user_id, &listeners])
		.await?;

	Ok(())
}
