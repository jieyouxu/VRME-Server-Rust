//! Handle updating user account information.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

/// The user is only allowed to update their first name and last name.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountInformationRequest {
	pub first_name: Option<String>,
	pub last_name: Option<String>,
}

/// Handles endpoint for updating user information at `PUT /account/{uuid}`.
///
/// ## Path Information
///
/// The route parameter required is:
///
/// - `{uuid}`: the unique identifier of the user generated upon successful registration.
///
/// ## Authentication Required
///
/// Required to supply the `Authentication: Bearer <uuid-token-pair>`.
///
/// ## Required Payload
///
/// Specify `Content-Type: application/json` and `Content-Length`.
///
/// ```rust,no_run
/// struct UpdatedAccountInformation {
///     first_name: String,
///     last_name: String,
/// }
/// ```
///
/// ## Example
///
/// ```http
/// PUT /account/{uuid}
/// Authorization: Bearer eyJ1dWlkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjU1NDQwMDAwIiwiYXV0aC10b2tlbiI6ImJhc2U2NC1lbmNvZGVkIn0=
/// Content-Type: application/json
///
/// {
///     "first_name": "NewFirstName",
///     "last_name": "NewLastName"
/// }
/// ```
///
/// ## Note
///
/// If the `uuid` of a user is lost by the client, it is possible to recover the `uuid` of the user
/// by the `GET /user/uuid` endpoint.
pub async fn update_user_account_handler(_path: web::Path<uuid::Uuid>) -> HttpResponse {
	unimplemented!()
}
