use actix_web::{web, HttpResponse};
use log::debug;
use serde::{Deserialize, Serialize};

/// A hashed password. The client is responsible for sending the entered
/// password in hashed form. Recommended to use `SHA-256` or stronger hash
/// function.
///
/// It is required that the hashed password have exactly 32-bytes.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct HashedPassword(String);

/// Required information for when a user wishes to register a new account.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct RegistrationRequest {
    pub(crate) first_name: String,
    pub(crate) last_name: String,

    /// A user must supply a valid email conforming with
    /// [RFC 2822](https://tools.ietf.org/html/rfc2822).
    pub(crate) email: String,

    /// A hashed password. The client is responsible for sending the entered
    /// password in hashed form.
    ///
    /// Recommended to use `SHA-256` or stronger hash function.
    ///
    /// It is required that the hashed password have exactly 32-bytes and Base64
    /// encoded, which means that it must have exactly `Ceil(4 * 32 / 3) == 43`
    /// Base64 characters.
    ///
    /// # References
    ///
    /// - [Base64 length calculation](https://stackoverflow.com/questions/13378815/base64-length-calculation)
    pub(crate) hashed_password: String,
}

/// Handle user registration.
///
/// # Arguments
///
/// * `req` - HTTP request must be a valid JSON of `RegistrationRequest`.
///
/// # Success: `201 Created`
///
/// If the account was successfuly created.
///
/// # Failure: `400 Bad Request` (Malformed JSON)
///
/// If the request body was malformed.
///
/// ```http
/// HTTP/1.1 400 Bad Request
///
/// {
///     "message": "malformed request body"
/// }
/// ```
pub(crate) async fn handle_register_user(
    req: web::Json<RegistrationRequest>,
) -> HttpResponse {
    debug!("request received");
    debug!("{:#?}", &req);

    HttpResponse::Ok().finish()
}
