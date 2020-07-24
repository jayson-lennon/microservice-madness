use fizzbuzz::buzz;
use libsvc::ServiceClient;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let service_client = ServiceClient::default();
    let res = buzz::buzz(1, "ok".to_owned(), vec![], &service_client).await?;
    println!("{:#?}", res);
    Ok(())
}
