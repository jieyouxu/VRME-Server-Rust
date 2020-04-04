//! Handler for uploading presentation slides.

use actix_multipart::{Field, Multipart};
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bytes::Bytes;
use deadpool_postgres::Client;
use futures::{StreamExt, TryStreamExt};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

use crate::auth::auth_payload::AuthPayload;
use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

/// Handler for the presenter to upload presentation slides for the given meeting session.
pub async fn handle_upload_presentation_slides(
	pool: web::Data<PersistentConnectionPool>,
	auth: BearerAuth,
	meeting_id: web::Path<Uuid>,
	mut payload: Multipart,
) -> Result<HttpResponse, Error> {
	let user_id = AuthPayload::from_bearer_auth(&auth)?.uuid;
	let client = pool.get().await?;

	if !is_presenter(&client, &meeting_id, &user_id).await? {
		return Err(ServiceError::Unauthorized(
			"Cannot modify the presentation slide if you are not the presenter".to_string(),
		)
		.into());
	}

	if let Ok(Some(mut field)) = payload.try_next().await {
		// TODO: use PDF instead of images (limited by front-end).
		check_content_type(&field)?;

		let mut file = create_presentation_file(&meeting_id).await?;

		while let Some(chunk) = field.next().await {
			let data = chunk?;
			file = write_to_presentation_file(file, data).await?;
		}
	}

	Ok(HttpResponse::Created().finish())
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

fn check_content_type(field: &Field) -> Result<(), ServiceError> {
	use std::ops::Deref;

	if field.content_type() == ContentType::png().deref() {
		Ok(())
	} else {
		Err(ServiceError::UnsupportedMediaType(
			"The provided file format is not supported for presentation".to_string(),
		))
	}
}

async fn create_presentation_file(meeting_id: &Uuid) -> Result<File, ServiceError> {
	// TODO: support PDF file format (limited by front-end).
	let file_path = format!("data/presentations/{}.png", meeting_id);

	// Delegate blocking fs calls to thread pool.
	web::block(move || File::create(&file_path))
		.await
		.map_err(|_| {
			ServiceError::InternalServerError(
				"Encountered error when trying to create presentation file".to_string(),
			)
		})
}

async fn write_to_presentation_file(mut file: File, data: Bytes) -> Result<File, ServiceError> {
	web::block(move || -> Result<File, ServiceError> {
		file.write_all(&data).map(|_| file).map_err(|e| e.into())
	})
	.await
	.map_err(|e| e.into())
}
