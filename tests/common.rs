use selectel_mks::Client;
use std::env;

const TEST_INTEGRATION: &str = "MKS_TEST_INTEGRATION";

const TEST_AUTH_TOKEN: &str = "MKS_TEST_AUTH_TOKEN";
const TEST_ENDPOINT: &str = "MKS_TEST_ENDPOINT";

pub const TEST_CLUSTER_ID: &str = "MKS_TEST_CLUSTER_ID";
pub const TEST_NODEGROUP_ID: &str = "MKS_TEST_NODEGROUP_ID";

/// Setup is used to prepare testing MKS client.
pub fn setup() -> Client {
    let token = env::var(TEST_AUTH_TOKEN)
        .expect(format!("Failed to read {} environment variable", TEST_AUTH_TOKEN).as_str());
    let base_endpoint = env::var(TEST_ENDPOINT)
        .expect(format!("Failed to read {} environment variable", TEST_ENDPOINT).as_str());
    let client = Client::new(base_endpoint.as_str(), token.as_str())
        .expect("Failed to initialize MKS client for tests");

    client
}

/// Check if integration tests are enabled.
pub fn integration_tests_are_enabled() -> bool {
    let enabled = match env::var(TEST_INTEGRATION) {
        Ok(v) => v == "1",
        _ => false,
    };

    if enabled {
        println!(
            "Integration testing is enabled. You can disable it by unsetting {} env variable",
            TEST_INTEGRATION
        );
        return enabled;
    }

    println!(
        "Integration testing is disabled. You can enable it by setting env variable {}=1",
        TEST_INTEGRATION
    );

    return false;
}
