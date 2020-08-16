use std::env;

pub mod common;

#[test]
fn get_cluster() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let cluster_id = env::var(common::TEST_CLUSTER_ID).expect(
        format!(
            "Failed to read {} environment variable to test get_cluster method",
            common::TEST_CLUSTER_ID
        )
        .as_str(),
    );

    let client = common::setup();
    let cluster = client
        .get_cluster(cluster_id.as_str())
        .expect("Failed to get a cluster");

    assert_eq!(cluster.id, cluster_id);
    println!("Cluster: {:?}\n", cluster);
}

#[test]
fn list_clusters() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let client = common::setup();
    let clusters = client.list_clusters().expect("Failed to list clusters");

    assert!(!clusters.is_empty());
    println!("Clusters: {:?}\n", clusters);
}
