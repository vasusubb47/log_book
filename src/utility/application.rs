use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{
    basic::{self, BasicAuth},
    AuthenticationError,
};

use crate::model::application::ApplicationKey;

pub async fn get_application_key_from_headder(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("get_application_key_from_headder");
    let key = req.headers().get("Api_Key");

    if key.is_none() {
        let config = req.app_data::<basic::Config>().cloned().unwrap_or_default();
        return Err((AuthenticationError::from(config).into(), req));
    }

    let key = key.unwrap().to_str().unwrap().to_string();

    println!("Api_Key {}", key);

    req.extensions_mut().insert(ApplicationKey { key });
    Ok(req)
}
