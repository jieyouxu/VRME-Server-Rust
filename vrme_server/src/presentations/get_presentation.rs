//! Handler for getting presentation file for the given meeting session.

use actix_files as afs;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use deadpool_postgres::Client;
use uuid::Uuid;

use crate::auth::auth_payload::AuthPayload;
use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

/// Handler for getting presentation file. Only meeting participants may get the presentation file.
pub async fn handle_get_presentation(
	pool: web::Data<PersistentConnectionPool>,
	meeting_id: web::Path<Uuid>,
	auth: BearerAuth,
) -> Result<afs::NamedFile, Error> {
	let client = pool.get().await?;
	let user_id = AuthPayload::from_bearer_auth(&auth)?.uuid;

	if !is_participant(&client, &meeting_id, &user_id).await? {
		return Err(ServiceError::Unauthorized(
			"Only meeting participants may get presentation file".to_string(),
		)
		.into());
	}

	let file = get_presentation_file(&meeting_id).await?;

	Ok(file
		.use_last_modified(true)
		.set_content_type(ContentType::png().0))
}

async fn get_presentation_file(meeting_id: &Uuid) -> Result<afs::NamedFile, ServiceError> {
	let raw_path = format!("data/presentations/{}.png", meeting_id);
	let path = std::path::PathBuf::from(&raw_path);

	web::block(move || -> Result<afs::NamedFile, ServiceError> {
		if path.exists() {
			Ok(afs::NamedFile::open(&path)?)
		} else {
			Err(ServiceError::NotFound(
				"No presentation file found for meeting session".to_string(),
			))
		}
	})
	.await
	.map_err(|e| e.into())
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

async fn is_participant(
	client: &Client,
	meeting_id: &Uuid,
	user_id: &Uuid,
) -> Result<bool, ServiceError> {
	if let Some((presenter, listeners)) = get_participants(client, meeting_id).await? {
		Ok(user_id == &presenter || listeners.contains(user_id))
	} else {
		Ok(false)
	}
}
