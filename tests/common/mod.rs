use mks::MKS;
use std::env;

const ENV_X_AUTH_TOKEN: &str = "X_AUTH_TOKEN";
const ENV_MKS_ENDPOINT: &str = "MKS_ENDPOINT";
const ENV_MKS_INTEGRATION_TESTING: &str = "MKS_INTEGRATION_TESTING";

/// Setup is used to prepare testing MKS client.
pub fn setup() -> MKS {
    let token = env::var(ENV_X_AUTH_TOKEN)
        .expect(format!("Failed to read {} environment variable", ENV_X_AUTH_TOKEN).as_str());
    let base_endpoint = env::var(ENV_MKS_ENDPOINT)
        .expect(format!("Failed to read {} environment variable", ENV_MKS_ENDPOINT).as_str());
    let client = MKS::new(base_endpoint.as_str(), token.as_str())
        .expect("Failed to initialize MKS client for tests");

    client
}

/// Check if integration tests are enabled.
pub fn integration_tests_are_enabled() -> bool {
    let enabled = match env::var(ENV_MKS_INTEGRATION_TESTING) {
        Ok(v) => v == "1",
        _ => false,
    };

    if enabled {
        println!(
            "Integration testing is enabled. You can disable it by unsetting MKS_INTEGRATION_TESTING"
        );
        return enabled;
    }

    println!(
        "Integration testing is disabled. You can enable it by setting MKS_INTEGRATION_TESTING=1"
    );

    return false;
}
