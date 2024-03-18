use shuttle_secrets::SecretStore;

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

    client
        .post_status("I AM ALIVE".to_string(), None)
        .await
        .unwrap();

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
