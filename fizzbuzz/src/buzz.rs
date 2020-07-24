use libsvc::{broker, Microservice, ServiceClient, ServiceError};
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

#[remote(src/buzz;fizzbuzz::buzz)]
pub async fn buzz(
    sample: i32,
    ok: String,
    a_vec: Vec<i32>,
    usvc_client: &ServiceClient,
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok("hello".to_string())
}
