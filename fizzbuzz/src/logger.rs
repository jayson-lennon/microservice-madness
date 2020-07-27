use crate::FizzBuzzError;
use libsvc::ServiceClient;
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

#[remote(src/logger;fizzbuzz::logger)]
pub async fn info(
    service_name: String,
    message: String,
    _svc: &ServiceClient,
) -> Result<(), FizzBuzzError> {
    info!("[{}]: {}", service_name, message);
    Ok(())
}

#[remote(src/logger;fizzbuzz::logger)]
pub async fn debug(
    service_name: String,
    message: String,
    _svc: &ServiceClient,
) -> Result<(), FizzBuzzError> {
    debug!("[{}]: {}", service_name, message);
    Ok(())
}
