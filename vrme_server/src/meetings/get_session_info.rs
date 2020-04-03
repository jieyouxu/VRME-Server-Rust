//! Handler for getting meeting session information.

use actix_web::web;
use actix_web::{Error, HttpResponse};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct MeetingSessionInfoResponsePayload {
	pub presenter: Uuid,
	pub listeners: Vec<Uuid>,
	pub started_at: chrono::NaiveDateTime,
}

/// Handler for getting meeting session information.
pub async fn handle_get_meeting_session_info(
	pool: web::Data<PersistentConnectionPool>,
	meeting_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;

	let response_payload = get_meeting_session_info(&client, &meeting_id).await?;

	Ok(HttpResponse::Ok().json(response_payload))
}

const GET_MEETING_SESSION_INFO_QUERY: &str = r#"
    SELECT
    (
        presenter,
        listeners,
        started_at
    )
    FROM meeting_sessions
    WHERE
        meeting_id = $1::UUID
    ;
"#;

async fn get_meeting_session_info(
	client: &Client,
	meeting_id: &Uuid,
) -> Result<MeetingSessionInfoResponsePayload, ServiceError> {
	let statement = client.prepare(GET_MEETING_SESSION_INFO_QUERY).await?;
	let rows = client.query(&statement, &[meeting_id]).await?;

	if !rows.is_empty() {
		Ok(MeetingSessionInfoResponsePayload {
			presenter: rows[0].get(0),
			listeners: rows[0].get(1),
			started_at: rows[0].get(2),
		})
	} else {
		Err(ServiceError::NotFound(format!(
			"No associated meeting session with id {} was found",
			meeting_id
		)))
	}
}
