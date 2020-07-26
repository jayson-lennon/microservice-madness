#[macro_use]
extern crate log;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod logger;
pub mod math;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum FizzBuzzError {
    #[error("service error: {0}")]
    Service(#[from] libsvc::ServiceError),
    #[error("serialization error: {0}")]
    Serialization(String),
}

impl From<serde_json::Error> for FizzBuzzError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}
