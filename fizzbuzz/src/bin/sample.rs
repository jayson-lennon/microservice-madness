#[macro_use]
extern crate log;
use dotenv::dotenv;
use futures::future::TryFutureExt;
//use fizzbuzz::math::add_impl;
use libsvc::{broker, ServiceClient, ServiceError};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

fn add_impl(lhs: i32, rhs: i32, svc: &ServiceClient) -> i32 {
    lhs + rhs
}

#[derive(Clone)]
pub struct State {
    client: ServiceClient,
}

#[derive(Serialize, Deserialize, Debug)]
struct _Params {
    _lhs: i32,
    _rhs: i32,
}

fn json_body() -> impl Filter<Extract = (_Params,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn process_request(params: _Params, state: State) -> Result<impl warp::Reply, Infallible> {
    let client = state.client.clone();
    let result = add_impl(params._lhs, params._rhs, &client);
    info!("action taken!");
    debug!("params: {:#?}", params);
    debug!("responding with {:#?}", result);
    Ok(warp::reply::json(&result))
}

fn with_state(
    state: State,
) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenv().ok();
    env_logger::init();
    info!("b00ting!");
    let mut rng = rand::thread_rng();
    let port: u16 = rng.gen_range(30000, 50000);

    let service_client = ServiceClient::default();

    broker::add_endpoint("add", &format!("127.0.0.1:{}", port), &service_client).await?;

    let state = State {
        client: service_client,
    };

    let route = warp::any()
        .and(warp::post())
        .and(json_body())
        .and(with_state(state))
        .and_then(process_request);

    warp::serve(route).run(([127, 0, 0, 1], port)).await;
    Ok(())
}
