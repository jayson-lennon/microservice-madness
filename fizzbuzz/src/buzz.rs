use libsvc::{broker, Microservice, ServiceClient, ServiceError};
use libsvc_codegen::microservice;
use serde::{Deserialize, Serialize};

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
        "buzz"
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
    #[microservice]
    async fn action(sample: i32, ok: String, usvc_client: &ServiceClient) -> Result<String, bool> {
        Ok("hello".to_string())
    }

    // Function generated: used by clients to make requests to this microservice.
    //    pub async fn _action_gen_2(
    //        sample: i32,
    //        ok: String,
    //        usvc_client: &ServiceClient,
    //    ) -> Result<Sample, ServiceError> {
    //        let params = _BuzzAction {
    //            _sample: sample,
    //            _ok: ok,
    //        };
    //
    //        let response = usvc_client.request(&params, "nowhere").await?;
    //        let response: Sample = serde_json::from_str(&response)?;
    //
    //        Err(ServiceError::Comms("".into()))
    //    }
    //
    //    // Function generated: used by clients to make requests to this microservice.
    //    pub async fn _action_gen_3(
    //        input: String,
    //        usvc_client: &ServiceClient,
    //    ) -> Result<_SampleResponse, ServiceError> {
    //        let endpoint = broker::get_endpoint("poc_service", usvc_client).await?;
    //        let params = _SampleRequest { input: input };
    //
    //        let response = usvc_client.request(&params, &endpoint.address).await?;
    //        let response: _SampleResponse = serde_json::from_str(&response)?;
    //        Ok(response)
    //
    //        //Err(ServiceError::Comms("".into()))
    //    }
}
