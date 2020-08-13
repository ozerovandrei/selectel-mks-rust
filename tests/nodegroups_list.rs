mod common;

use std::env;

const TEST_CLUSTER_ID: &str = "MKS_TEST_CLUSTER_ID";

#[test]
fn list_nodegroups() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let cluster_id = env::var(TEST_CLUSTER_ID).expect(
        format!(
            "Failed to read {} environment variable to test list_nodegroups method",
            TEST_CLUSTER_ID
        )
        .as_str(),
    );

    let client = common::setup();
    let nodegroups = client
        .list_nodegroups(cluster_id.as_str())
        .expect("Failed to list nodegroups");

    assert!(!nodegroups.is_empty());
    println!("Nodegroups: {:?}", nodegroups);
}
