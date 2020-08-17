use selectel_mks::cluster;
use selectel_mks::nodegroup;
use std::collections::HashMap;

pub mod common;

#[test]
fn nodegroup_crud() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    // Retrieve needed variables from env.
    let region = common::get_region();
    let az = common::get_availability_zone();
    let kube_version = common::get_kube_version();

    // Prepare MKS client.
    let client = common::setup();

    // Prepare create options.
    let name = "nodegroup-crud".to_string();
    let nodegroup_opts = nodegroup::schemas::CreateOpts::new(1, false, az.clone())
        .with_cpus(1)
        .with_ram_mb(1024)
        .with_volume_gb(10)
        .with_volume_type(format!("fast.{}", az));
    let create_opts = cluster::schemas::CreateOpts::new(name, kube_version, region)
        .with_nodegroups(vec![nodegroup_opts]);

    // Create a new cluster.
    let cluster = common::cluster_common::create_cluster_or_panic(&client, &create_opts);

    // Create a new nodegroup for the cluster.
    let new_nodegroup_opts = nodegroup::schemas::CreateOpts::new(2, false, az.clone())
        .with_cpus(1)
        .with_ram_mb(1024)
        .with_volume_gb(10)
        .with_volume_type(format!("fast.{}", az));
    common::nodegroup_common::create_nodegroup_or_panic(&client, &cluster.id, &new_nodegroup_opts);

    // List all cluster nodegroups.
    let nodegroups = client
        .list_nodegroups(&cluster.id)
        .expect("failed to list cluster nodegroups");
    assert!(!nodegroups.is_empty());
    println!("All nodegroups: {:?}\n", nodegroups);

    // Resize the first nodegroup of the cluster.
    let resize_opts = nodegroup::schemas::ResizeOpts::new(3);
    common::nodegroup_common::resize_nodegroup_or_panic(
        &client,
        &cluster.id,
        &nodegroups[0].id,
        &resize_opts,
    );

    // Add labels for the first nodegroup of the cluster.
    let mut labels: HashMap<String, String> = HashMap::new();
    labels.insert(String::from("a"), String::from("b"));
    let update_opts = nodegroup::schemas::UpdateOpts::new().with_labels(labels);
    common::nodegroup_common::update_nodegroup_or_panic(
        &client,
        &cluster.id,
        &nodegroups[0].id,
        &update_opts,
    );

    // Delete the first nodegroup of the cluster.
    common::nodegroup_common::delete_nodegroup_or_panic(&client, &cluster.id, &nodegroups[0].id);

    // Delete the created cluster.
    common::cluster_common::delete_cluster_or_panic(&client, &cluster.id);
}
