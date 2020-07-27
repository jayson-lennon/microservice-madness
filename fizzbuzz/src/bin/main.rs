use dotenv::dotenv;
use fizzbuzz::logger::info;
use fizzbuzz::math;
use fizzbuzz::number;
use fizzbuzz::util;
use futures::future::TryFutureExt;
use libsvc::ServiceClient;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();
    env_logger::init();
    let svc = ServiceClient::default();

    {
        let (lhs, rhs) = {
            let (lhs, rhs) = tokio::join!(
                number::get("five".to_owned(), &svc),
                number::get("two".to_owned(), &svc),
            );
            (lhs?.unwrap(), rhs?.unwrap())
        };

        let _ = tokio::join!(
            math::add(lhs, rhs, &svc)
                .and_then(|answer| log(format!("{} + {} = {}", lhs, rhs, answer), &svc)),
            math::sub(lhs, rhs, &svc)
                .and_then(|answer| log(format!("{} - {} = {}", lhs, rhs, answer), &svc)),
            math::mul(lhs, rhs, &svc)
                .and_then(|answer| log(format!("{} * {} = {}", lhs, rhs, answer), &svc)),
            math::div(lhs, rhs, &svc)
                .and_then(|answer| log(format!("{} / {} = {}", lhs, rhs, answer.unwrap()), &svc)),
        );
    }

    {
        use math::rem;
        use util::i32_eq;
        use util::vec_of_i32;

        let zero_through_fifteen = vec_of_i32(1, 15, &svc).await?;

        for i in zero_through_fifteen {
            let (by_three_remainder, by_five_remainder, by_fifteen_remainder) =
                tokio::join!(rem(i, 3, &svc), rem(i, 5, &svc), rem(i, 15, &svc));

            let (divisible_by_three, divisible_by_five, divisible_by_fifteen) = tokio::join!(
                i32_eq(by_three_remainder?.unwrap(), 0, &svc),
                i32_eq(by_five_remainder?.unwrap(), 0, &svc),
                i32_eq(by_fifteen_remainder?.unwrap(), 0, &svc)
            );

            if divisible_by_fifteen? {
                log("fizzbuzz".to_owned(), &svc).await?;
            } else if divisible_by_three? {
                log("fizz".to_owned(), &svc).await?;
            } else if divisible_by_five? {
                log("buzz".to_owned(), &svc).await?;
            } else {
                log(format!("{}", i), &svc).await?;
            }
        }
    }

    Ok(())
}

async fn log(message: String, svc: &ServiceClient) -> Result<(), fizzbuzz::FizzBuzzError> {
    info(module_path!().to_string(), message.to_owned(), &svc).await
}
