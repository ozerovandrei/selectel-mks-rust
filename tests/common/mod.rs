pub mod cluster_common;
pub mod node_common;
pub mod nodegroup_common;

use selectel_mks::Client;
use std::env;

// Env variable to turn integration testing on or off.
const TEST_INTEGRATION: &str = "MKS_TEST_INTEGRATION";

// Env variables for auth.
const TEST_AUTH_TOKEN: &str = "MKS_TEST_AUTH_TOKEN";
const TEST_ENDPOINT: &str = "MKS_TEST_ENDPOINT";

// Env variables for MKS resources parameters.
const TEST_REGION: &str = "MKS_TEST_REGION";
const TEST_AVAILABILITY_ZONE: &str = "MKS_TEST_AVAILABILITY_ZONE";
const TEST_KUBE_VERSION: &str = "MKS_TEST_KUBE_VERSION";

/// Setup is used to prepare testing MKS client.
pub fn setup() -> Client {
    let token = env::var(TEST_AUTH_TOKEN).unwrap_or_else(|_| {
        panic!(
            "Failed to read {} environment variable for integration testing",
            TEST_AUTH_TOKEN
        )
    });

    let base_endpoint = env::var(TEST_ENDPOINT).unwrap_or_else(|_| {
        panic!(
            "Failed to read {} environment variable for integration testing",
            TEST_ENDPOINT
        )
    });

    Client::new(base_endpoint.as_str(), token.as_str())
        .expect("Failed to initialize MKS client for tests")
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

    false
}

/// Get region from environment variables for integration tests.
pub fn get_region() -> String {
    env::var(TEST_REGION).unwrap_or_else(|_| {
        panic!(
            "Failed to read {} environment variable for integration testing",
            TEST_REGION
        )
    })
}

/// Get availability zone from environment variables for integration tests.
pub fn get_availability_zone() -> String {
    env::var(TEST_AVAILABILITY_ZONE).unwrap_or_else(|_| {
        panic!(
            "Failed to read {} environment variable for integration testing",
            TEST_AVAILABILITY_ZONE
        )
    })
}

/// Get Kubernetes version from environment variables for integration tests.
pub fn get_kube_version() -> String {
    env::var(TEST_KUBE_VERSION).unwrap_or_else(|_| {
        panic!(
            "Failed to read {} environment variable for integration testing",
            TEST_KUBE_VERSION
        )
    })
}
