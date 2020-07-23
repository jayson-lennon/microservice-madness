use libsvc::{broker, Microservice, ServiceClient, ServiceError};
use libsvc_codegen::{remote, Microservice};
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &'static str = "buzz";

#[derive(Microservice)]
pub struct Buzz;

impl Microservice for Buzz {
    fn is_stateful() -> bool {
        false
    }

    fn init() -> Self {
        println!("init ok");
        Buzz {}
    }

    fn name() -> &'static str {
        SERVICE_NAME
    }
}

// Struct generated:
#[derive(Serialize, Deserialize, Debug)]
struct _BuzzAction {
    _sample: i32,
    _ok: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _SampleResponse {
    output: String,
}

impl Buzz {
    // The "action" function performs the logic for the microservice.
    // A proc macro attribute on this function will generate:
    //   * An additional function for use by clients to call the service.
    //   * A struct used for serialization
    #[remote]
    async fn action(
        sample: i32,
        ok: String,
        a_vec: Vec<i32>,
        usvc_client: &ServiceClient,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok("hello".to_string())
    }
}
