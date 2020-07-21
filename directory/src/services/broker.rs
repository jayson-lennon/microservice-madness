use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
