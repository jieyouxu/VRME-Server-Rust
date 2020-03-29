//! Retrieve avatar.

use crate::service_errors::ServiceError;
use actix_files as afs;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::Error;
use uuid::Uuid;

/// Serves the avatar of the user with `uuid`. If the user did *not* upload an avatar yet, the
/// default avatar is served instead.
///
/// ## Path Parameter
///
/// 1. `{uuid}`: the unique id of the user whose avatar the client is trying to get.
pub async fn handle_get_avatar(uuid: web::Path<Uuid>) -> Result<afs::NamedFile, Error> {
	let file = get_avatar_or_default(&uuid).await?;

	Ok(file
		.use_last_modified(true)
		.set_content_type(ContentType::png().0))
}

async fn get_avatar_or_default(uuid: &Uuid) -> Result<afs::NamedFile, ServiceError> {
	let raw_path = build_path(&uuid);
	let path = std::path::PathBuf::from(&raw_path);

	// We delegate the responsibility of reading a PNG file to a thread pool to avoid blocking.
	web::block(move || -> Result<afs::NamedFile, ServiceError> {
		let file = if path.exists() {
			afs::NamedFile::open(&path)?
		} else {
			afs::NamedFile::open("data/avatars/default.png")?
		};

		Ok(file.use_last_modified(true).use_etag(true))
	})
	.await
	.map_err(|e| e.into())
}

fn build_path(uuid: &Uuid) -> String {
	format!("/data/avatars/{}.png", uuid)
}
