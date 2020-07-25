use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ServiceClient, ServiceError};

pub async fn get_endpoint(
    name: &str,
    service_client: &ServiceClient,
) -> Result<Service, ServiceError> {
    trace!("in get endpoint");
    let params = params::GetService {
        name: name.to_owned(),
    };
    trace!("params = {:#?}", params);
    let response = service_client.request(&params, "http://localhost:8080/find")?;
    trace!("response = {:#?}", response);
    let response: Service = serde_json::from_str(&response)?;
    Ok(response)
}

pub async fn add_endpoint(
    name: &str,
    address: &str,
    service_client: &ServiceClient,
) -> Result<(), ServiceError> {
    trace!("in add endpoint");
    let params = params::AddService {
        name: name.to_owned(),
        address: address.to_owned(),
    };
    trace!("params = {:#?}", params);
    let _ = service_client.request(&params, "http://localhost:8080/add")?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Services(pub HashMap<String, Service>);

pub mod params {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetService {
        pub name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AddService {
        pub name: String,
        pub address: String,
    }
}
