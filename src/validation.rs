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
    if credentials.token() == config.token {
        Ok(req)
    } else {
        let bearer_config = req.app_data::<Config>().cloned().unwrap_or_default();
        Err((AuthenticationError::from(bearer_config).into(), req))
    }
}
