use libsvc::{broker, ServiceClient};
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

//#[remote(src/math;fizzbuzz::math)]
pub async fn add(
    lhs: i32,
    rhs: i32,
    svc: &ServiceClient,
) -> Result<i32, Box<dyn std::error::Error + Send + Sync + 'static>> {
    trace!("request received");
    //let _ = crate::logger::usvc_log("add".to_string(), format!("Adding {} + {}", lhs, rhs), svc).await;
    Ok(lhs + rhs)
}
