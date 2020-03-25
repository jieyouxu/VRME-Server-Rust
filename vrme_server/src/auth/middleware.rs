//! Authentication middleware.

use crate::auth::errors::AuthError;
use crate::settings::AuthSettings;

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{
	BearerAuth, Config as BearerConfig,
};
use actix_web_httpauth::extractors::{
	AuthExtractorConfig, AuthenticationError,
};

/// Validator function that filters requests and based on client-provided authentication information
/// (or the lack thereof), decides whether to reject further action (i.e. `401 Unauthorized`) or to
/// pass on the handling to further handlers down the response chain.
pub async fn identity_validator(
	req: ServiceRequest,
	credentials: BearerAuth,
) -> Result<ServiceRequest, AuthError> {
	unimplemented!()
}
