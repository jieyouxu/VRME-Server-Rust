use crate::types::email::Email;
use crate::types::hashed_password::HashedPassword;
use actix_web::{web, HttpResponse};
use base64;
use log::debug;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str;

/// Required information for when a user wishes to register a new account.
///
/// ```json
/// POST /register HTTP/1.1
/// Content-Type: application/json
///
/// {
///     "first_name": "John",
///     "last_name": "Doe",
///     "email": "no-reply@example.com",
///     "hashed_password": "1234567890123456789012345678901234567890123"
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct RegistrationRequest {
    pub(crate) first_name: String,
    pub(crate) last_name: String,

    /// A user must supply a valid email conforming with
    /// [RFC 2822](https://tools.ietf.org/html/rfc2822).
    pub(crate) email: Email,

    /// A base64-encoded hashed password. The client is responsible for sending
    /// the entered password in hashed form. Recommended to use `SHA-256`
    /// or stronger hash function.
    ///
    /// It is required that the hashed password have exactly 32-bytes and
    /// Base64 encoded, which means that it must have exactly `Ceil(4 * 32
    /// / 3) == 43` Base64 characters.
    ///
    /// # References
    ///
    /// - [Base64 length calculation](https://stackoverflow.com/questions/13378815/base64-length-calculation)
    #[serde(rename = "hashed_password")]
    pub(crate) base64_encoded_hashed_password: String,
}

/// The processed and validated information from a given `RegistrationRequest`.
#[derive(Debug, PartialEq)]
struct RegistrationInfo {
    first_name: String,
    last_name: String,
    email: Email,
    hashed_password: HashedPassword,
}

/// Handle user registration.
///
/// # Arguments
///
/// * `req` - HTTP request must be a valid JSON of `RegistrationRequest`.
///
/// # Responses
///
/// ## Success: `201 Created`
///
/// If the account was successfuly created.
///
/// ## Failure: `400 Bad Request` (Malformed JSON)
///
/// If the request body was malformed.
///
/// ```http
/// HTTP/1.1 400 Bad Request
/// Content-Type: application/json
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

    let req = match validate_request(&req) {
        Some(req) => req,
        None => {
            return HttpResponse::BadRequest().json(json!({
                "cause": "malformed-request",
                "message": "request is malformed, invalid JSON payload"
            }));
        }
    };

    // Check if an email already exists. We will return a `409 Conflict`
    // indicating that the provided email is already associated with an account.
    let account_exists =
        match account_exists_with_email(req.email.email()).await {
            Ok(v) => v,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "cause": "internal-server-error",
                    "message": "failed to check for existing account"
                }));
            }
        };

    if account_exists {
        return HttpResponse::Conflict().json(json!({
            "cause": "account-exists",
            "message": "an account with the provided email already exists, login instead"
        }));
    }

    // If account does not already exist, we create a new account.

    // First, we need to deal with the password.
    let _salt = generate_salt().await;

    HttpResponse::Ok().finish();

    unimplemented!();
}

/// It is required that the hashed password have exactly 32-bytes and
/// Base64 encoded, which means that it must have exactly `Ceil(4 * 32
/// / 3) == 43` Base64 characters.
const BASE64_ENCODED_PASSWORD_LENGTH: usize = 43;

fn validate_request(req: &RegistrationRequest) -> Option<RegistrationInfo> {
    let RegistrationRequest {
        first_name,
        last_name,
        email,
        base64_encoded_hashed_password,
    } = req;

    if first_name.trim().is_empty()
        || last_name.trim().is_empty()
        || email.email().is_empty()
        || base64_encoded_hashed_password.len()
            != BASE64_ENCODED_PASSWORD_LENGTH
    {
        return None;
    }

    let password = base64::decode(base64_encoded_hashed_password).ok()?;
    let password = str::from_utf8(&password).ok()?;

    let info = RegistrationInfo {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        email: email.clone(),
        hashed_password: HashedPassword::new(password).ok()?,
    };

    Some(info)
}

/// Length of `salt` in bytes.
const SALT_LENGTH: usize = 16;

/// We generate a 16-byte `salt`. This is currently generated via a CSPRNG,
/// which uses `ChaCha20` block cipher with 20 rounds.
///
/// See [rand::rngs::StdRng](https://docs.rs/rand/0.7.3/rand/rngs/struct.StdRng.html).
async fn generate_salt() -> Option<[u8; SALT_LENGTH]> {
    let mut buffer = [0u8; SALT_LENGTH];

    match web::block(move || {
        rand::thread_rng().fill(&mut buffer);
        Ok::<_, ()>(())
    })
    .await
    {
        Ok(_) => {
            debug!("successfully generated salt");
        }
        Err(e) => {
            debug!("failed to generate salt");
            debug!("{:#?}", &e);
            return None;
        }
    }

    Some(buffer)
}

async fn account_exists_with_email(email: &str) -> Result<bool, ()> {
    debug!("checking existence of account with email {}", email);
    unimplemented!()
}
