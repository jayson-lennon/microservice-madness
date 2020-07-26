use dotenv::dotenv;
use fizzbuzz::logger::usvc_log;
use fizzbuzz::math;
use libsvc::ServiceClient;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();
    env_logger::init();
    let svc = ServiceClient::default();
    let answer = math::add(2, 2, &svc).await?;
    usvc_log(
        "main".to_string(),
        format!("the answer is: {:?}", answer),
        &svc,
    )
    .await?;
    Ok(())
}
