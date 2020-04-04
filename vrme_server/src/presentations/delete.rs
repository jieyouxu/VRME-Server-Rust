//! Handles delete presentation.

use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use deadpool_postgres::Client;
use uuid::Uuid;

use crate::auth::auth_payload::AuthPayload;
use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

/// Handler for deleting presentation file.
pub async fn handle_delete_presentation(
	pool: web::Data<PersistentConnectionPool>,
	meeting_id: web::Data<Uuid>,
	auth: BearerAuth,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;
	let user_id = AuthPayload::from_bearer_auth(&auth)?.uuid;

	// Only presenter may delete the presentation file.
	if !is_presenter(&client, &meeting_id, &user_id).await? {
		return Err(ServiceError::Unauthorized(
			"Only presenter of meeting session may delete presentation file".to_string(),
		)
		.into());
	}

	delete_file(&meeting_id).await?;
	Ok(HttpResponse::NoContent().finish())
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

async fn delete_file(meeting_id: &Uuid) -> Result<(), ServiceError> {
	let raw_path = format!("/data/presentations/{}.png", &meeting_id);
	let path = std::path::PathBuf::from(&raw_path);

	web::block(move || {
		if path.exists() {
			std::fs::remove_file(&path)
		} else {
			Ok(())
		}
	})
	.await
	.map_err(|e| e.into())
}
