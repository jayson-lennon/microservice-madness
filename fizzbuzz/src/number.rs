use crate::FizzBuzzError;
use libsvc::ServiceClient;
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

#[remote(src/number;fizzbuzz::number)]
pub async fn get(name: String, svc: &ServiceClient) -> Result<Option<i32>, FizzBuzzError> {
    let _ = crate::logger::debug(
        module_path!().to_string(),
        format!("Get number '{}'", name),
        svc,
    )
    .await;
    Ok(match name.as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        _ => None,
    })
}
