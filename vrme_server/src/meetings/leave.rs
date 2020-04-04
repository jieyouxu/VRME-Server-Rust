//! Handler for a listener leaving the meeting session.

use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use deadpool_postgres::Client;
use uuid::Uuid;

use crate::auth::auth_payload::AuthPayload;
use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

/// Handler for leaving meeting session.
///
/// If the _presenter_ leaves the meeting session, the meeting session terminates.
pub async fn handle_leave_meeting_session(
	pool: web::Data<PersistentConnectionPool>,
	meeting_id: web::Path<Uuid>,
	auth: BearerAuth,
) -> Result<HttpResponse, Error> {
	let user_id = AuthPayload::from_bearer_auth(&auth)?.uuid;

	let client = pool.get().await?;

	if is_presenter(&client, &meeting_id, &user_id).await? {
		delete_meeting_session(&client, &meeting_id).await?;
		Ok(HttpResponse::NoContent().finish())
	} else if is_listener(&client, &meeting_id, &user_id).await? {
		delete_meeting_session(&client, &meeting_id).await?;
		Ok(HttpResponse::NoContent().finish())
	} else {
		Err(
			ServiceError::NotFound("User does not belong to the meeting session".to_string())
				.into(),
		)
	}
}

async fn is_presenter(
	client: &Client,
	meeting_id: &Uuid,
	user_id: &Uuid,
) -> Result<bool, ServiceError> {
	if let Some((presenter_id, _)) = get_participants(&client, &meeting_id).await? {
		Ok(&presenter_id == user_id)
	} else {
		Ok(false)
	}
}

const GET_PARTICIPANTS_QUERY: &str = r#"
    SELECT
        presenter,
        listeners
    FROM
        meeting_sessions
    WHERE
        meeting_id = $1::UUID
    ;
"#;

async fn get_participants(
	client: &Client,
	meeting_id: &Uuid,
) -> Result<Option<(Uuid, Vec<Uuid>)>, ServiceError> {
	let statement = client.prepare(GET_PARTICIPANTS_QUERY).await?;

	let rows = client.query(&statement, &[meeting_id]).await?;

	if rows.is_empty() {
		Ok(None)
	} else {
		let (presenter, listeners) = (rows[0].get(0), rows[0].get(1));
		Ok(Some((presenter, listeners)))
	}
}

async fn is_listener(
	client: &Client,
	meeting_id: &Uuid,
	user_id: &Uuid,
) -> Result<bool, ServiceError> {
	if let Some((_, listeners_ids)) = get_participants(&client, &meeting_id).await? {
		Ok(listeners_ids.contains(user_id))
	} else {
		Ok(false)
	}
}

const DELETE_MEETING_SESSION_QUERY: &str = r#"
    DELETE FROM
        meeting_sessions
    WHERE
        meeting_id = $1::UUID
    ;
"#;

async fn delete_meeting_session(client: &Client, meeting_id: &Uuid) -> Result<(), ServiceError> {
	let statement = client.prepare(DELETE_MEETING_SESSION_QUERY).await?;
	client.query(&statement, &[meeting_id]).await?;
	Ok(())
}
