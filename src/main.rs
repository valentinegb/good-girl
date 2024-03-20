use std::time::Duration;

use megalodon::{megalodon::PostStatusOutput, Megalodon};
use rand::{seq::SliceRandom, thread_rng};
use shuttle_runtime::SecretStore;
use tracing::{error, info};

const SLEEP_SECS: u64 = 60 * 60 * 24;
const NAMES: &[&str] = &[
    "Lily", "Alice", "Emily", "Rose", "Sophie", "Sophia", "Samantha", "Natalie", "Luna", "Ruby",
    "Zoey", "Chloe", "Emma", "Alex", "Amy", "Sammy", "Sabrina",
];

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> Result<GoodGirlService, shuttle_runtime::Error> {
    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        "https://gaygeek.social".to_string(),
        secret_store.get("ACCESS_TOKEN"),
        None,
    );

    Ok(GoodGirlService { client })
}

struct GoodGirlService {
    client: Box<dyn Megalodon + Send + Sync>,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for GoodGirlService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        loop {
            tokio::time::sleep(Duration::from_secs(SLEEP_SECS)).await;

            let name;

            {
                let mut rng = thread_rng();

                name = NAMES.choose(&mut rng).unwrap();
            }

            match self
                .client
                .post_status(format!("{} is such a good girl", name), None)
                .await
            {
                Ok(response) => {
                    match response.json {
                        PostStatusOutput::Status(status) => {
                            info!("Posted a status with content \"{}\"", status.content);
                        }
                        other => {
                            error!("Expected `PostStatusOutput` to be `Status` variant, got: {other:#?}");
                        }
                    }
                }
                Err(err) => error!("Failed to post status: {err}"),
            }
        }
    }
}
