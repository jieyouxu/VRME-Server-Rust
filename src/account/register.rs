use actix_web::{web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use validator;
use validator_derive;

/// A hashed password. The client is responsible for sending the entered password in hashed form.
/// Recommended to use `SHA-256` or stronger hash function.
///
/// It is required that the hashed password have exactly 32-bytes.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct HashedPassword(String);

/// Required information for when a user wishes to register a new account.
#[derive(Debug, Deserialize, Serialize, PartialEq, Validate, ValidationError)]
pub(crate) struct RegistrationRequest {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    /// A user must supply a valid email conforming with
    /// [RFC 2822](https://tools.ietf.org/html/rfc2822).
    pub(crate) email: String,
    /// A hashed password. The client is responsible for sending the entered password in hashed form.
    /// Recommended to use `SHA-256` or stronger hash function.
    ///
    /// It is required that the hashed password have exactly 32-bytes.
    pub(crate) hashed_password: String,
}

/// User registration endpoint.
///
/// # Arguments
///
/// * `req` - HTTP request must be a valid JSON of `RegistrationRequest`.
async fn register_user(req: web::Json<RegistrationRequest>) -> impl Responder {}
