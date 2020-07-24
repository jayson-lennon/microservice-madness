#[macro_use] extern crate log ; use dotenv :: dotenv ; use libsvc ::
{ broker, ServiceClient, ServiceError } ; use rand :: Rng ; use serde ::
{ Deserialize, Serialize } ; use tide :: { Request, StatusCode } ; use
fizzbuzz :: logger :: usvc_log_impl ; #[derive(Clone)] pub struct State
{ client : ServiceClient, } #[derive(Serialize, Deserialize)] struct _Params
{ _service_name : String, _message : String } async fn
recv_request(mut req : Request < State >) -> tide :: Result < serde_json ::
Value >
{
    let client = & req . state() . client . clone() ; let params : _Params =
    req . body_json() . await ? ; let result =
    usvc_log_impl(params . _service_name, params . _message, & client) . await
    .
    map_err(| e | tide :: Error ::
            from_str(StatusCode :: InternalServerError, e . to_string())) ? ;
    info ! ("action taken!") ; trace ! ("responding with {:#?}", result) ;
    Ok(serde_json :: to_value(result) .
       expect("failed to convert to JSON value"))
} #[tokio :: main] async fn main() -> Result < (), Box < dyn std :: error ::
Error + Send + Sync + 'static >>
{
    dotenv() . ok() ; env_logger :: init() ; info ! ("b00ting!") ; let mut rng
    = rand :: thread_rng() ; loop
    {
        let port : u32 = rng . gen_range(30000, 50000) ; let bind = format !
        ("http://127.0.0.1:{}", port) ; let service_client = ServiceClient ::
        default() ; broker ::
        add_endpoint("usvc_log", & bind, & service_client) . await ? ; let mut
        app = tide :: Server :: with_state(State { client : service_client, })
        ; app . at("/") . post(recv_request) ; if app . listen(bind) . await .
        is_err() { continue ; } else { break ; }
    } Ok(())
}