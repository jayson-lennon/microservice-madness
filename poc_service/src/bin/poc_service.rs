#[macro_use]
extern crate log;

use directory::{
    broker,
    poc_service::{self, response::Capitalized},
};
use dotenv::dotenv;
use serde_json::json;
use tide::Request;

fn action(params: poc_service::params::Capitalize) -> Capitalized {
    Capitalized {
        output: params.input.to_uppercase(),
    }
}

async fn recv_request(mut req: Request<()>) -> tide::Result<serde_json::Value> {
    let params: poc_service::params::Capitalize = req.body_json().await?;
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

    let http_client = surf::Client::new();
    let service_info = broker::params::AddService {
        name: "poc_service".to_owned(),
        address: "http://127.0.0.1:9001".to_owned(),
    };

    http_client
        .post("http://localhost:8080/add")
        .body_json(&service_info)?
        .await?;

    let mut app = tide::new();
    app.at("/").post(recv_request);

    app.listen("127.0.0.1:9001").await?;
    Ok(())
}
