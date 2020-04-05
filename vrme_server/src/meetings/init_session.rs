//! Handles meeting session initiation.

use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::auth_payload::AuthPayload;
use crate::database::postgresql::PersistentConnectionPool;
use crate::service_errors::ServiceError;

/// Response payload upon successful meeting session initialization.
#[derive(Debug, Deserialize, Serialize)]
pub struct MeetingSessionResponsePayload {
	pub meeting_id: Uuid,
	pub presenter: Uuid,
	pub listeners: Vec<Uuid>,
	pub started_at: chrono::NaiveDateTime,
}

/// Handler for initializing a meeting session. It must be intiated by an authenticated `presenter`.
/// If a meeting session associated with the *presenter* already exists, then info on th existing
/// meeting session is returned; otherwise a fresh meeting session is created and its info returned.
pub async fn handle_init_session(
	pool: web::Data<PersistentConnectionPool>,
	auth: BearerAuth,
) -> Result<HttpResponse, Error> {
	let client = pool.get().await?;
	let presenter_id = AuthPayload::from_bearer_auth(&auth)?.uuid;

	let meeting_session_response_payload =
		create_new_session_or_return_existing(&client, &presenter_id).await?;

	Ok(HttpResponse::Created().json(meeting_session_response_payload))
}

const UPSERT_MEETING_SESSION_QUERY: &str = r#"
    INSERT INTO meeting_sessions
    (
        meeting_id,
        presenter,
        listeners,
        started_at
    )
    VALUES
    (
        $1::UUID,
        $2::UUID,
        $3::UUID[],
        $4::TIMESTAMP
    )
    ON CONFLICT DO NOTHING;
"#;

const GET_SESSION_INFO_QUERY: &str = r#"
    SELECT
        meeting_id,
        presenter,
        listeners,
        started_at
    FROM
        meeting_sessions
    WHERE
        meeting_id = $1::UUID
    ;
"#;

async fn create_new_session_or_return_existing(
	client: &Client,
	presenter_id: &Uuid,
) -> Result<MeetingSessionResponsePayload, ServiceError> {
	let upsert_statement = client.prepare(UPSERT_MEETING_SESSION_QUERY).await?;

	let meeting_id = Uuid::new_v4();
	let listeners: Vec<Uuid> = Vec::new();
	let started_at = chrono::Utc::now().naive_utc();

	client
		.query(
			&upsert_statement,
			&[&meeting_id, &presenter_id, &listeners, &started_at],
		)
		.await?;

	let get_session_statement = client.prepare(GET_SESSION_INFO_QUERY).await?;

	let rows = client.query(&get_session_statement, &[&meeting_id]).await?;

	let (meeting_id, presenter_id, listeners_ids, started_at) = (
		rows[0].get(0),
		rows[0].get(1),
		rows[0].get(2),
		rows[0].get(3),
	);

	Ok(MeetingSessionResponsePayload {
		meeting_id,
		presenter: presenter_id,
		listeners: listeners_ids,
		started_at,
	})
}
