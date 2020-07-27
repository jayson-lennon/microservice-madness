use crate::FizzBuzzError;
use libsvc::ServiceClient;
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

#[remote(src/util;fizzbuzz::util)]
pub async fn vec_of_i32(
    start: i32,
    end: i32,
    svc: &ServiceClient,
) -> Result<Vec<i32>, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Create Vec<i32> range {}..={}", start, end),
        svc,
    )
    .await;
    Ok((start..=end).collect())
}

#[remote(src/util;fizzbuzz::util)]
pub async fn i32_eq(lhs: i32, rhs: i32, svc: &ServiceClient) -> Result<bool, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Cmp: {} & {}", lhs, rhs),
        svc,
    )
    .await;
    Ok(lhs == rhs)
}
