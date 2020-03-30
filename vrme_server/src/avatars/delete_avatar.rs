//! Delete user's avatar.

use crate::service_errors::ServiceError;
use actix_web::web;
use actix_web::{Error, HttpResponse};
use uuid::Uuid;

// Delete's the avatar of the user with id `{uuid}`. This is a destructive operation and the deleted
// avatar cannot be recovered by the client.
pub async fn handle_delete_avatar(uuid: web::Path<Uuid>) -> Result<HttpResponse, Error> {
	delete_file(&uuid).await?;
	Ok(HttpResponse::NoContent().finish())
}

async fn delete_file(uuid: &Uuid) -> Result<(), ServiceError> {
	let raw_target_avatar_path = format!("/data/avatars/{}.png", &uuid);
	let path = std::path::PathBuf::from(&raw_target_avatar_path);

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
