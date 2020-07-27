use crate::FizzBuzzError;
use libsvc::ServiceClient;
use libsvc_codegen::remote;
use serde::{Deserialize, Serialize};

pub trait StatefulService {
    // Called when the service is first initialized on the server.
    fn init(svc_client: &ServiceClient) -> Self;
}

// This struct will get renamed to SampleServiceImpl (below).
// After renaming, a new struct will be generated with this name (SampleService)
// that contains only a single member: a pointer to the ServiceClient.
pub struct SampleService__in_source_code {
    // All members should be private. This will prevent confusion when using this
    // structure on the client side.
    some_data: i32,

    // Public fields should result in a compiler error since the data present
    // would not be valid when running on the client.
    pub not_allowed: i32,

    // All stateful services should store a pointer to the service client.
    // This can then be used on the client side to avoid having to pass in the
    // service client for each method call.
    svc_client: ServiceClient,
}

// This is the structure used by the client in order to commnunicate with the server.
pub struct SampleService__after_codegen_pass {
    svc_client: ServiceClient,
}

// This is the function that the client can use to initialize a connection to the server.
impl StatefulService for SampleService__after_codegen_pass {
    fn init(svc_client: &ServiceClient) -> Self {
        Self {
            svc_client: svc_client.clone(),
        }
    }
}

// This struct will be the one created on the server.
pub struct SampleServiceImpl {
    some_data: i32,
    pub not_allowed: i32,
    svc_client: ServiceClient,
}

// This implementation is the one used on the server to initialize the struct.
impl StatefulService for SampleServiceImpl {
    fn init(svc_client: &ServiceClient) -> Self {
        Self {
            some_data: 1,
            not_allowed: 2,
            svc_client: svc_client.clone(),
        }
    }
}

// This implementation block should be duplicated with one being renamed to SampleServiceImpl.
// The SampleService struct must have all public functions changed to relay functions.
// Private static methods can be copied verbatim in both implementations
// since the client cannot access them and they can only be ran in the context
// of the server.
impl SampleService {
    // A typical public method accepted &self to be dispatched on the server.
    pub fn do_stateless_stuff(&self) -> Result<i32, FizzBuzzError> {
        // Get self.svc_client here
        Ok(5)
    }

    // Public static methods require a service client to be passed in since they will
    // be dispatched to the server.
    pub fn static_method(i: i32, svc: &ServiceClient) -> Result<i32, FizzBuzzError> {
        Ok(i)
    }

    // Static private methods are ok since they will only ever be ran on the server.
    fn private_static_method(i: i32) -> i32 {
        5
    }

    // Moving self should get rejected since this would result in the remote service
    // terminating.
    pub fn move_self_not_allowed(self) -> Result<i32, FizzBuzzError> {
        panic!("Not allowed to move self")
    }

    // This public static method should get rejected since there is no way to dispatch
    // this method call to the server.
    pub fn static_method_svc_client_required(i: i32) -> Result<i32, FizzBuzzError> {
        panic!("Must use svc_client on all public methods")
    }
}
