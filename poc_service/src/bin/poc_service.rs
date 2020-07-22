use directory::poc_service::{params, response::Capitalized};
use libsvc::make_service;

make_service!("poc_service", params::Capitalize);

fn action(params: params::Capitalize) -> Capitalized {
    Capitalized {
        output: params.input.to_uppercase(),
    }
}
