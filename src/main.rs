use std::time::Duration;

use rand::{seq::SliceRandom, thread_rng};
use shuttle_secrets::SecretStore;

const NAMES: &[&str] = &[
    "Lily", "Alice", "Emily", "Rose", "Sophie", "Sophia", "Samantha", "Natalie", "Luna", "Ruby",
    "Zoey", "Chloe", "Emma", "Alex", "Amy", "Sammy",
];

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<MyService, shuttle_runtime::Error> {
    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        "https://gaygeek.social".to_string(),
        secret_store.get("ACCESS_TOKEN"),
        None,
    );

    tokio::spawn(async move {
        loop {
            let name;

            {
                let mut rng = thread_rng();

                name = NAMES.choose(&mut rng).unwrap();
            }

            client
                .post_status(format!("{} is such a good girl", name), None)
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        }
    });

    Ok(MyService {})
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct MyService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for MyService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        Ok(())
    }
}
