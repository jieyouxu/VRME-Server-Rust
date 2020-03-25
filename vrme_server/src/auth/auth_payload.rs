//! Authentication payload.

use serde::{Serialize, Deserialize};
use base64;

/// Required authentication payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    /// An user's unique identifer. If the client lost this information, they can retreive it using
    /// the `GET /account/uuid` endpoint.
    pub uuid: uuid::Uuid,
    /// Authentication token issued by the server whena user successfully registered via
    /// `POST /register` and logs in successfully via `POST /login`.
    pub auth_token: String,
}
