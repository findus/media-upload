use actix_web::dev::ServiceRequest;
use actix_web::{web, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use crate::cfg::ServerConfig;

pub(crate) async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<web::Data<ServerConfig>>().unwrap();
    if constant_time_eq(credentials.token().as_bytes(), config.token.as_bytes()) {
        Ok(req)
    } else {
        let bearer_config = req.app_data::<Config>().cloned().unwrap_or_default();
        Err((AuthenticationError::from(bearer_config).into(), req))
    }
}

/// Compares two byte strings without short-circuiting on the first mismatch,
/// so comparing a secret token doesn't leak how many leading bytes matched
/// through response-time differences.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
