use libsvc::{broker, Microservice, ServiceClient, ServiceError};
use serde::{Deserialize, Serialize};

pub fn fizz() {
    println!("fizz from lib");
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Sample {
    One,
    Two,
}

pub struct Fizz;

impl Microservice for Fizz {
    fn is_stateful() -> bool {
        false
    }

    fn init() -> Self {
        println!("init ok");
        Fizz {}
    }

    fn name() -> &'static str {
        "fizz"
    }
}

// Struct generated:
#[derive(Serialize, Deserialize, Debug)]
struct _FizzAction {
    _sample: i32,
    _ok: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _SampleResponse {
    output: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct _SampleRequest {
    input: String,
}

impl Fizz {
    // The "action" function performs the logic for the microservice.
    // A proc macro attribute on this function will generate:
    //   * An additional function for use by clients to call the service.
    //   * A struct used for serialization
    async fn action(sample: i32, ok: String, usvc_client: &ServiceClient) -> Result<Sample, bool> {
        Ok(Sample::One)
    }

    // Function generated: used by clients to make requests to this microservice.
    pub async fn _action_gen_2(
        sample: i32,
        ok: String,
        usvc_client: &ServiceClient,
    ) -> Result<Sample, ServiceError> {
        let params = _FizzAction {
            _sample: sample,
            _ok: ok,
        };

        let response = usvc_client.request(&params, "nowhere").await?;
        let response: Sample = serde_json::from_str(&response)?;

        Err(ServiceError::Comms("".into()))
    }

    // Function generated: used by clients to make requests to this microservice.
    pub async fn _action_gen_3(
        input: String,
        usvc_client: &ServiceClient,
    ) -> Result<_SampleResponse, ServiceError> {
        let endpoint = broker::get_endpoint("poc_service", usvc_client).await?;
        let params = _SampleRequest { input: input };

        let response = usvc_client.request(&params, &endpoint.address).await?;
        let response: _SampleResponse = serde_json::from_str(&response)?;
        Ok(response)

        //Err(ServiceError::Comms("".into()))
    }
}
