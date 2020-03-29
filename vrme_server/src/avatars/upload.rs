//! Handler for uploading an avatar – either to replace a "default" avatar or to replace the current
//! active avatar.

use crate::auth::auth_payload::AuthPayload;
use crate::service_errors::ServiceError;
use actix_multipart::{Field, Multipart};
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bytes::Bytes;
use futures::{StreamExt, TryStreamExt};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

/// Handler for avatar image upload.
///
/// The endpoint is protected.
///
/// The client should upload the image in `PNG` format – this should probably be done using a HTML
/// form which has a file upload entry, so the client can send the POST payload as a
/// `multipart/form-data` where the server assumes the first form data is the desired avatar.
///
/// ## References
///
/// - Multipart form specified by [RFC 7578](https://tools.ietf.org/html/rfc7578)
pub async fn handle_upload_avatar(
	auth: BearerAuth,
	uuid: web::Path<Uuid>,
	mut payload: Multipart,
) -> Result<HttpResponse, Error> {
	let auth_payload = AuthPayload::from_bearer_auth(&auth)?;

	if &auth_payload.uuid != &uuid.into_inner() {
		return Err(
			ServiceError::Forbidden("Cannot modify someone else's avatar".to_string()).into(),
		);
	}

	// We don't iterate over the stream – instead we only take the first form entry assuming that is
	// the desired PNG avatar file itself.
	if let Ok(Some(mut field)) = payload.try_next().await {
		check_content_type_is_png(&field)?;

		let mut file = create_avatar_file(&auth_payload.uuid).await?;
		while let Some(chunk) = field.next().await {
			let data = chunk?;
			file = write_to_avatar_file(file, data).await?;
		}
	}

	Ok(HttpResponse::Created().finish())
}

fn check_content_type_is_png(field: &Field) -> Result<(), ServiceError> {
	use std::ops::Deref;

	if field.content_type() == ContentType::png().deref() {
		Ok(())
	} else {
		Err(ServiceError::UnprocessableEntity(
			"`Content-Disposition` must be specified".to_string(),
		))
	}
}

async fn create_avatar_file(uuid: &Uuid) -> Result<File, ServiceError> {
	// We save the avatar to a file with the user's `uuid` as filename.
	let file_path = format!("data/avatars/{}.png", uuid);

	// Delegate blocking fs operation to the thread pool.
	web::block(move || File::create(&file_path))
		.await
		.map_err(|_| {
			ServiceError::InternalServerError(
				"Encountered I/O error when trying to create avatar file".to_string(),
			)
		})
}

async fn write_to_avatar_file(mut file: File, data: Bytes) -> Result<File, ServiceError> {
	web::block(move || file.write_all(&data).map(|_| file))
		.await
		.map_err(|e| e.into())
}
