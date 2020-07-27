#[macro_use]
extern crate log;

use dashmap::DashMap;
use dotenv::dotenv;
use libsvc::broker::{params, Service, Services};
use std::collections::HashMap;
use std::sync::Arc;
use tide::Request;

#[derive(Debug, Clone)]
struct State {
    pub services: Arc<DashMap<String, Service>>,
}

async fn get_directory(req: Request<State>) -> tide::Result<serde_json::Value> {
    let service_map = &req.state().services;
    let mut services = Services(HashMap::new());
    for service in service_map.iter() {
        services.0.insert(
            service.key().to_owned(),
            Service {
                name: service.key().to_owned(),
                address: service.value().address.to_owned(),
            },
        );
    }
    let directory = Ok(serde_json::to_value(services).expect("failed to convert to json value"));
    directory
}

async fn find_service(mut req: Request<State>) -> tide::Result<serde_json::Value> {
    let service_name = {
        let params: params::GetService = req.body_json().await?;
        params.name
    };
    req.state()
        .services
        .get(&service_name)
        .map(|service| {
            serde_json::to_value(service.value()).expect("failed to convert service into value")
        })
        .ok_or_else(|| tide::Error::from_str(tide::StatusCode::NotFound, "Service not found"))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    env_logger::init();

    info!("Listening on 127.0.0.1:8080");

    let state = State {
        services: Arc::new(DashMap::new()),
    };

    let mut app = tide::with_state(state);

    app.at("/").get(get_directory);
    app.at("/find").post(find_service);

    app.at("/add").post(|mut req: Request<State>| async move {
        let params: params::AddService = req.body_json().await?;
        info!("Add service @ {} - {}", params.address, params.name);
        let services = &req.state().services;
        let service = Service {
            name: params.name.to_owned(),
            address: params.address.to_owned(),
        };
        services.insert(params.name, service);
        Ok("added")
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
