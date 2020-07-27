use crate::FizzBuzzError;
use libsvc::ServiceClient;
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

#[remote(src/math;fizzbuzz::math)]
pub async fn add(lhs: i32, rhs: i32, svc: &ServiceClient) -> Result<i32, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Add: {} + {}", lhs, rhs),
        svc,
    )
    .await;
    Ok(lhs + rhs)
}

#[remote(src/math;fizzbuzz::math)]
pub async fn sub(lhs: i32, rhs: i32, svc: &ServiceClient) -> Result<i32, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Sub: {} - {}", lhs, rhs),
        svc,
    )
    .await;
    Ok(lhs - rhs)
}

#[remote(src/math;fizzbuzz::math)]
pub async fn mul(lhs: i32, rhs: i32, svc: &ServiceClient) -> Result<i32, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Mul: {} * {}", lhs, rhs),
        svc,
    )
    .await;
    Ok(lhs * rhs)
}

#[remote(src/math;fizzbuzz::math)]
pub async fn div(lhs: i32, rhs: i32, svc: &ServiceClient) -> Result<Option<f64>, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Div: {} / {}", lhs, rhs),
        svc,
    )
    .await;
    if rhs == 0 {
        Ok(None)
    } else {
        Ok(Some(lhs as f64 / rhs as f64))
    }
}

#[remote(src/math;fizzbuzz::math)]
pub async fn rem(lhs: i32, rhs: i32, svc: &ServiceClient) -> Result<Option<i32>, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Rem: {} / {}", lhs, rhs),
        svc,
    )
    .await;
    if rhs == 0 {
        Ok(None)
    } else {
        Ok(Some(lhs % rhs))
    }
}
