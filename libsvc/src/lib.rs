use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod broker;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ServiceError {
    #[error("communication error {0}")]
    Comms(String),

    #[error("serialization error {0}")]
    Serialization(String),

    #[error("server error {0}")]
    ServerError(String),
}

impl From<reqwest::Error> for ServiceError {
    fn from(err: reqwest::Error) -> Self {
        ServiceError::Comms(err.to_string())
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        ServiceError::Serialization(err.to_string())
    }
}

impl From<tide::Error> for ServiceError {
    fn from(err: tide::Error) -> Self {
        ServiceError::ServerError(err.to_string())
    }
}

pub trait Microservice {
    fn is_stateful() -> bool;
    fn init() -> Self;
    fn name() -> &'static str;
}

#[derive(Debug, Clone)]
pub struct ServiceClient {
    inner: reqwest::Client,
}

impl ServiceClient {
    pub async fn request<T: Serialize>(
        &self,
        params: &T,
        url: &str,
    ) -> Result<String, ServiceError> {
        Ok(self
            .inner
            .post(url)
            .json(params)
            .send()
            .await?
            .text()
            .await?)
    }
}

impl Default for ServiceClient {
    fn default() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }
}
