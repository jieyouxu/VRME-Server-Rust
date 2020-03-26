use crate::service_errors::ServiceError;
use actix_web::error::{Error, JsonPayloadError};
use actix_web::HttpRequest;

/// Custom JSON error handler.
///
/// When a JSON error is encountered, it returns a detailed error message for the malformed payload,
/// both for syntactical malformed payload and for semantically malformed payload such as missing
/// fields.
///
/// # Example Error Response
///
/// ```http
/// HTTP/1.1 400 Bad Request
/// Content-Type: application/json
///
/// {
///     "cause": "bad-request",
///     "message": "Invalid JSON at [line = 1, col = 1]"
/// }
/// ```
pub fn handle_json_error(err: JsonPayloadError, _req: &HttpRequest) -> Error {
	let err_msg = make_error_message(err);

	ServiceError::BadRequest(format!("Failed to parse payload as JSON: {}", err_msg)).into()
}

fn make_error_message(err: JsonPayloadError) -> String {
	match err {
		JsonPayloadError::Overflow => format!("Payload size exceeds the max limit"),
		JsonPayloadError::ContentType => {
			"Invalid `Content-Type` header: use `application/json`".to_string()
		}
		JsonPayloadError::Deserialize(ref e) => handle_deserialize_errors(e),
		_ => "Invalid JSON payload".to_string(),
	}
}

fn handle_deserialize_errors(e: &serde_json::error::Error) -> String {
	use serde_json::error::Category;
	match e.classify() {
		Category::Syntax => format!(
			"Invalid JSON at [line = {}, col = {}]",
			e.line(),
			e.column()
		),
		Category::Data => {
			// Unfortunately `serde_json`'s Errors are opaque and do not contain useful
			// information such as missing fields.
			//
			// Hence, we can only exploit it's `std::fmt::Display`'s implementation to get
			// which field is missing that is required.
			format!(
				"Missing required field(s) and/or values have invalid types: {}",
				e.to_string()
			)
		}
		Category::Eof => "Expected EOF when trying to parse JSON".to_string(),
		Category::Io => "IO error when parsing JSON".to_string(),
	}
}
