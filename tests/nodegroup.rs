use std::env;

pub mod common;

#[test]
fn list_nodegroups() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let cluster_id = env::var(common::TEST_CLUSTER_ID).expect(
        format!(
            "Failed to read {} environment variable to test list_nodegroups method",
            common::TEST_CLUSTER_ID
        )
        .as_str(),
    );

    let client = common::setup();
    let nodegroups = client
        .list_nodegroups(cluster_id.as_str())
        .expect("Failed to list nodegroups");

    assert!(!nodegroups.is_empty());
    println!("Nodegroups: {:?}\n", nodegroups);
}

#[test]
fn get_nodegroup() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let cluster_id = env::var(common::TEST_CLUSTER_ID).expect(
        format!(
            "Failed to read {} environment variable to test get_nodegroup method",
            common::TEST_CLUSTER_ID
        )
        .as_str(),
    );

    let nodegroup_id = env::var(common::TEST_NODEGROUP_ID).expect(
        format!(
            "Failed to read {} environment variable to test get_nodegroup method",
            common::TEST_NODEGROUP_ID
        )
        .as_str(),
    );

    let client = common::setup();
    let nodegroup = client
        .get_nodegroup(cluster_id.as_str(), nodegroup_id.as_str())
        .expect("Failed to get a nodegroup");

    assert_eq!(nodegroup.id, nodegroup_id);
    println!("Nodegroup: {:?}\n", nodegroup);
}
