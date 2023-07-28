use dynomite::dynamodb::{DynamoDb, DynamoDbClient};
use dynomite::retry::Policy;
use dynomite::Retries;
use lambda_http::lambda_runtime::LambdaEvent;
use lambda_http::{lambda_runtime, service_fn};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = DynamoDbClient::new(Default::default()).with_retries(Policy::default());

    lambda_runtime::run(service_fn(move |_event: LambdaEvent<()>| {
        let client = client.clone();

        async move {
            let tables = client
                .list_tables(Default::default())
                .await?
                .table_names
                .unwrap_or_default();

            Ok::<_, Error>(tables.join("\n"))
        }
    }))
    .await?;

    Ok(())
}
