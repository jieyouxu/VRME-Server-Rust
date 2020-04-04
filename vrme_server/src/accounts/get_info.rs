//! Handler for fetching account information.

use actix_web::web;
use actix_web::{Error, HttpResponse};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

/// Handler for getting account information (publicly available).
pub async fn handle_get_account_info(
	pool: web::Data<PersistentConnectionPool>,
	target_user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;

	let account_info = get_account_info(&client, &target_user_id).await?;

	Ok(HttpResponse::Ok().json(account_info))
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountInfoResponse {
	first_name: String,
	last_name: String,
}

const GET_ACCOUNT_INFO_QUERY: &str = r#"
    SELECT
        first_name,
        last_name
    FROM
        accounts
    WHERE
        user_id = $1::UUID
    ;
"#;

async fn get_account_info(
	client: &Client,
	target_user_id: &Uuid,
) -> Result<AccountInfoResponse, ServiceError> {
	let statement = client.prepare(GET_ACCOUNT_INFO_QUERY).await?;
	let rows = client.query(&statement, &[target_user_id]).await?;

	if rows.is_empty() {
		Err(ServiceError::NotFound(
			"No matching account found".to_string(),
		))
	} else {
		let (first_name, last_name) = (rows[0].get(0), rows[0].get(1));
		Ok(AccountInfoResponse {
			first_name,
			last_name,
		})
	}
}
