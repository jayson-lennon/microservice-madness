use crate::FizzBuzzError;
use libsvc::ServiceClient;
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

#[remote(src/math;fizzbuzz::math)]
pub async fn add(lhs: i32, rhs: i32, svc: &ServiceClient) -> Result<i32, FizzBuzzError> {
    let _ = crate::logger::usvc_log(
        module_path!().to_string(),
        format!("Adding {} + {}", lhs, rhs),
        svc,
    )
    .await;
    Ok(lhs + rhs)
}
