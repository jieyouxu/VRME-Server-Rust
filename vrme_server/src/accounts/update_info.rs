//! Handle updating user account information.

use actix_web::{web, HttpResponse};

/// Handles endpoint for updating user information at `PUT /account/{uuid}`.
///
/// ## Path Information
///
/// The route parameter required is:
///
/// - `{uuid}`: the unique identifier of the user generated upon successful registration.
///
/// ## Note
///
/// If the `uuid` of a user is lost by the client, it is possible to recover the `uuid` of the user
/// by the `GET /user/uuid` endpoint.
pub async fn update_user_account_handler(
	_path: web::Path<uuid::Uuid>,
) -> HttpResponse {
	unimplemented!()
}
