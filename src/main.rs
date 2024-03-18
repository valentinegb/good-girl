use std::time::Duration;

use rand::{seq::SliceRandom, thread_rng};
use shuttle_secrets::SecretStore;
use tracing::info;

const NAMES: &[&str] = &[
    "Lily", "Alice", "Emily", "Rose", "Sophie", "Sophia", "Samantha", "Natalie", "Luna", "Ruby",
    "Zoey", "Chloe", "Emma", "Alex", "Amy", "Sammy",
];

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<GoodGirlService, shuttle_runtime::Error> {
    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        "https://gaygeek.social".to_string(),
        secret_store.get("ACCESS_TOKEN"),
        None,
    );

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;

            let name;

            {
                let mut rng = thread_rng();

                name = NAMES.choose(&mut rng).unwrap();
            }

            let status = format!("{} is such a good girl", name);

            client.post_status(status.clone(), None).await.unwrap();
            info!("Posted a status: {status}");
        }
    });

    Ok(GoodGirlService {})
}

struct GoodGirlService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for GoodGirlService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Keep the service going for as long as possible
        loop {}
    }
}
