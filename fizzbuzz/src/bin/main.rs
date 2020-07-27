use dotenv::dotenv;
use fizzbuzz::logger::{debug, info};
use fizzbuzz::math;
use fizzbuzz::number;
use fizzbuzz::util;
use libsvc::ServiceClient;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();
    env_logger::init();
    let svc = ServiceClient::default();

    let lhs = number::get("five".to_owned(), &svc).await?.unwrap();
    let rhs = number::get("two".to_owned(), &svc).await?.unwrap();
    {
        let answer = math::add(lhs, rhs, &svc).await?;
        log(&format!("{} + {} = {}", lhs, rhs, answer), &svc).await?;
    }

    {
        let answer = math::sub(lhs, rhs, &svc).await?;
        log(&format!("{} - {} = {}", lhs, rhs, answer), &svc).await?;
    }

    {
        let answer = math::mul(lhs, rhs, &svc).await?;
        log(&format!("{} * {} = {}", lhs, rhs, answer), &svc).await?;
    }

    {
        let lhs = lhs;
        let rhs = 0;
        let answer = math::div(lhs, rhs, &svc).await?;
        log(&format!("{} / {} = {:?}", lhs, rhs, answer), &svc).await?;
    }

    {
        use math::rem;
        use util::i32_eq;
        use util::vec_of_i32;

        let one_hundred_numbers = vec_of_i32(0, 100, &svc).await?;

        for i in one_hundred_numbers {
            let div_by_three = rem(i, 3, &svc).await?.unwrap();
            let div_by_five = rem(i, 5, &svc).await?.unwrap();

            if i32_eq(div_by_three, 0, &svc).await? {
                log("fizz", &svc).await?;
            } else if i32_eq(div_by_five, 0, &svc).await? {
                log("buzz", &svc).await?;
            } else {
                log(&format!("{}", i), &svc).await?;
            }
        }
    }

    Ok(())
}

async fn log(message: &str, svc: &ServiceClient) -> Result<(), fizzbuzz::FizzBuzzError> {
    info(module_path!().to_string(), message.to_owned(), &svc).await
}
