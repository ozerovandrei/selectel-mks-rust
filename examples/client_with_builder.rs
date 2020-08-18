use selectel_mks::Client;
use std::time::Duration;

fn main() {
    // Configure custom timeout for a new client.
    let timeout_secs = 10;

    // Get endpoint for the needed region:
    //  - ru-1: https://ru-1.mks.selcloud.ru
    //  - ru-2: https://ru-2.mks.selcloud.ru
    //  - ru-3: https://ru-3.mks.selcloud.ru
    //  - ru-7: https://ru-7.mks.selcloud.ru
    //  - ru-8: https://ru-8.mks.selcloud.ru
    let endpoint = "https://ru-1.mks.selcloud.ru";

    // Get project-scoped token value.
    let token = "token_value";

    let _client = Client::builder()
        .with_timeout(Duration::from_secs(timeout_secs))
        .build(endpoint, token)
        .expect("failed to initialize MKS client");
}
