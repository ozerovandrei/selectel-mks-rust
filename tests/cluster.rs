use selectel_mks::cluster;

pub mod common;

#[test]
fn cluster_crud() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    // Retrieve needed variables from env.
    let region = common::get_region();
    let kube_version = common::get_kube_version();

    // Prepare MKS client.
    let client = common::setup();

    // Prepare create options.
    let name = "cluster-crud";
    let create_opts = cluster::schemas::CreateOpts::new(name, &kube_version, &region);

    // Create a new cluster.
    let cluster = common::cluster_common::create_cluster_or_panic(&client, &create_opts);

    // Get the new cluster.
    let cluster = client
        .get_cluster(&cluster.id)
        .expect("failed to get the created cluster");
    println!("Created cluster: {:?}\n", cluster);

    // List all clusters.
    let clusters = client.list_clusters().expect("failed to list all clusters");
    assert!(!clusters.is_empty());
    println!("All clusters: {:?}\n", clusters);

    // Delete the created cluster.
    common::cluster_common::delete_cluster_or_panic(&client, &cluster.id);
}
