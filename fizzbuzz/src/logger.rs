use libsvc::{broker, ServiceClient};
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

#[remote(src/logger;fizzbuzz::logger)]
pub async fn usvc_log(
    service_name: String,
    message: String,
    svc: &ServiceClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("[{}]: {}", service_name, message);
    Ok(())
}