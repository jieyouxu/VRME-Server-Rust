use actix_web::error;
use actix_web::{HttpRequest, HttpResponse};

/// JSON error handler.
///
/// Take from: https://github.com/actix/examples/blob/master/json_decode_error/src/main.rs.
pub fn handle_json_error(cause: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
	use actix_web::error::JsonPayloadError;

	let detail = cause.to_string();
	let response = match &cause {
		JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
		JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
			HttpResponse::UnprocessableEntity().body(detail)
		}
		_ => HttpResponse::BadRequest().body(detail),
	};

	error::InternalError::from_response(cause, response).into()
}
