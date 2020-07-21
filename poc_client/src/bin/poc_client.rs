#[macro_use]
extern crate log;

use directory::{
    broker::{self, Service},
    poc_service,
};
use dotenv::dotenv;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn get_endpoint(name: &str, http_client: reqwest::Client) -> Result<Service, BoxError> {
    trace!("in get endpoint");
    let params = broker::params::GetService {
        name: name.to_owned(),
    };
    trace!("params = {:#?}", params);
    let response = http_client
        .post("http://localhost:8080/find")
        .json(&params)
        .send()
        .await?;
    trace!("response = {:#?}", response);
    let response = response.json::<Service>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();

    env_logger::init();

    info!("try doing stuff!");

    let http_client = reqwest::Client::new();
    let endpoint = get_endpoint("poc_service", http_client.clone()).await?;
    let params = poc_service::params::Capitalize {
        input: "whats up?".to_owned(),
    };

    info!("try get client");
    let res = http_client
        .post(&endpoint.address)
        .json(&params)
        .send()
        .await?
        .json::<poc_service::response::Capitalized>()
        .await?;
    trace!("getgot: {:#?}", res);

    Ok(())
}
