use fizzbuzz::Fizz;
use libsvc::ServiceClient;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let service_client = ServiceClient::default();
    let sup = Fizz::_action_gen_3("test".to_owned(), &service_client).await?;
    println!("{:#?}", sup);
    Ok(())
}
