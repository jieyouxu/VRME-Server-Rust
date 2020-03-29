//! Get a user's `uuid` by providing the user's email.

use crate::database::ConnectionPool;
use crate::service_errors::ServiceError;
use actix_web::{web, Error, HttpResponse, ResponseError};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

/// Required information to get a person's `uuid`.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetUuidRequest {
	pub email: String,
}

pub async fn handle_get_uuid(
	pool: web::Data<ConnectionPool>,
	req: web::Json<GetUuidRequest>,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;
	let uuid = get_uuid_given_email(&client, &req.email).await?;
	Ok(HttpResponse::Ok().json(json!({ "uuid": uuid })))
}

const GET_UUID_GIVEN_EMAIL_QUERY: &str = r#"
    SELECT user_id
    FROM accounts
    WHERE email = $1::VARCHAR(355);
"#;

/// Retreive the user's `Uuid` given their `email` address.
async fn get_uuid_given_email(client: &Client, email: &str) -> Result<Uuid, ServiceError> {
	let statement = client.prepare(GET_UUID_GIVEN_EMAIL_QUERY).await.unwrap();
	let row = client.query_one(&statement, &[&email]).await?;
	let uuid = row.get(0);
	Ok(uuid)
}
