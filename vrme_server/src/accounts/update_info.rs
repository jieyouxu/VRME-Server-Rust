//! Handle updating user account information.

use crate::database::ConnectionPool;
use crate::service_errors::ServiceError;
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::Client;
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
pub async fn handle_update_user_account(
	pool: web::Data<ConnectionPool>,
	path: web::Path<uuid::Uuid>,
	req: web::Json<UpdateAccountInformationRequest>,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;
	update_names(&client, &path, &req.first_name, &req.last_name).await?;
	Ok(HttpResponse::NoContent().finish())
}

const UPSERT_NAME_QUERY: &str = r#"
    INSERT INTO accounts
        (first_name, last_name)
    VALUES (
        $2::VARCHAR(100),
        $3::VARCHAR(100)
    )
    WHERE
        user_id = $1::UUID
    ON CONFLCIT (first_name, last_name)
    DO UPDATE SET
        first_name = COALESCE(EXCLUDED.first_name, first_name),
        last_name =  COALESCE(EXCLUDED.last_name, last_name)
    ;
"#;

async fn update_names(
	client: &Client,
	uuid: &uuid::Uuid,
	first_name: &Option<String>,
	last_name: &Option<String>,
) -> Result<(), ServiceError> {
	let statement = client.prepare(UPSERT_NAME_QUERY).await?;
	client
		.query(
			&statement,
			&[
				&uuid,
				first_name.as_ref().unwrap_or(&"".to_string()),
				last_name.as_ref().unwrap_or(&"".to_string()),
			],
		)
		.await?;
	Ok(())
}
