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
/// The client should upload the image in PNG format – this should probably be done using a HTML
/// form which has a file upload entry, so the client can send the POST payload as a
/// `multipart/form-data` where the server assumes the first form data is the desired avatar.
///
/// ## Caveats
///
/// We do not perform any validation on whether the supplied binary data is in fact valid PNG – it
/// is the client's responsibility to make sure that a valid PNG is sent as the avatar.
///
/// If an invalid avatar is uploaded, then anyone who calls `GET /account/{uuid}/avatar` will get
/// back the invalid avatar.
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
		// We don't validate if the raw binary payload is in fact valid PNG. If the client uploads
		// an invalid PNG, then upon request the client will receive the same invalid PNG.
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
		Err(ServiceError::UnsupportedMediaType(
			"Avatar can only be PNG file format".to_string(),
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
	web::block(move || {
		let expected_file_size = file.metadata()?.len() + data.len() as u64;
		if expected_file_size > AVATAR_SIZE_LIMIT {
			Err(ServiceError::UnprocessableEntity(
				"File size too large".to_string(),
			))
		} else {
			file.write_all(&data).map(|_| file).map_err(|e| e.into())
		}
	})
	.await
	.map_err(|e| e.into())
}

/// Upper limit on avatar file size in bytes.
///
/// - Assume the max dimensions of the PNG file is `512 x 512` (pixels).
/// - Assume the PNG is uncompressed.
/// - Assume the PNG is RGBA, each channel requiring `8` bits, meaning each pixel requires
///   `4 * 8 = 32` bits to represent (`= 4` bytes).
///
/// Approximate max size limit in bytes is calculated by (only raw payload, no metadata or header
/// information):
///
/// ```
/// 512 * 512 * 4 / 4 = 1024 bytes
/// ```
///
/// We give some allowance to this to allow some metadata and header information.
pub const AVATAR_SIZE_LIMIT: u64 = 1280;
