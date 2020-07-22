#[macro_use]
extern crate log;

pub mod broker;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ServiceError {
    #[error("communication error {0}")]
    Comms(String),

    #[error("serialization error {0}")]
    Serialization(String),
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

#[macro_export]
macro_rules! make_service {
    ($name:expr, $params:ty) => {
        #[macro_use]
        extern crate log;

        use directory::broker;
        use dotenv::dotenv;
        use rand::Rng;
        use serde_json::json;
        use tide::Request;

        async fn recv_request(mut req: Request<()>) -> tide::Result<serde_json::Value> {
            // Generate a params type from the function signature
            let params: $params = req.body_json().await?;
            let result = action(params);

            info!("action taken!");

            trace!("responding with {:#?}", result);

            Ok(serde_json::to_value(result).expect("failed to convert to JSON value"))
        }

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            dotenv().ok();

            env_logger::init();

            info!("b00ting!");

            let http_client = reqwest::Client::new();
            let mut rng = rand::thread_rng();

            loop {
                let port: u32 = rng.gen_range(30000, 50000);

                let mut app = tide::new();
                app.at("/").post(recv_request);

                let bind = format!("http://127.0.0.1:{}", port);

                let params = broker::params::AddService {
                    name: $name.to_owned(),
                    address: bind.to_owned(),
                };

                let _ = http_client
                    .post("http://localhost:8080/add")
                    .json(&params)
                    .send()
                    .await?;

                if app.listen(bind).await.is_err() {
                    continue;
                } else {
                    break;
                }
            }
            Ok(())
        }
    };
}
