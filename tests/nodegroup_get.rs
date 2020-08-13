mod common;

use std::env;

const TEST_CLUSTER_ID: &str = "MKS_TEST_CLUSTER_ID";
const TEST_NODEGROUP_ID: &str = "MKS_TEST_NODEGROUP_ID";

#[test]
fn get_nodegroup() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let cluster_id = env::var(TEST_CLUSTER_ID).expect(
        format!(
            "Failed to read {} environment variable to test get_nodegroup method",
            TEST_CLUSTER_ID
        )
        .as_str(),
    );

    let nodegroup_id = env::var(TEST_NODEGROUP_ID).expect(
        format!(
            "Failed to read {} environment variable to test get_nodegroup method",
            TEST_NODEGROUP_ID
        )
        .as_str(),
    );

    let client = common::setup();
    let nodegroup = client
        .get_nodegroup(cluster_id.as_str(), nodegroup_id.as_str())
        .expect("Failed to get a nodegroup");

    assert_eq!(nodegroup.id, nodegroup_id);
    println!("Nodegroup: {:?}", nodegroup);
}
