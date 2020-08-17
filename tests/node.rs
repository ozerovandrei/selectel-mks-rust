use selectel_mks::cluster;
use selectel_mks::nodegroup;

pub mod common;

#[test]
fn node_crud() {
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
    let name = "node-crud";
    let nodegroup_opts = nodegroup::schemas::CreateOpts::new(1, false, &az)
        .with_cpus(1)
        .with_ram_mb(1024)
        .with_volume_gb(10)
        .with_volume_type(&format!("fast.{}", az));
    let create_opts = cluster::schemas::CreateOpts::new(name, &kube_version, &region)
        .with_nodegroups(vec![nodegroup_opts]);

    // Create a new cluster.
    let cluster = common::cluster_common::create_cluster_or_panic(&client, &create_opts);

    // List all cluster nodegroups.
    let nodegroups = client
        .list_nodegroups(&cluster.id)
        .expect("failed to list cluster nodegroups");
    assert!(!nodegroups.is_empty());
    println!("All nodegroups: {:?}\n", nodegroups);

    // Get the first node.
    let node = client
        .get_node(&cluster.id, &nodegroups[0].id, &nodegroups[0].nodes[0].id)
        .expect("failed to get cluster node");
    println!("Node: {:?}\n", node);

    // Delete the created cluster.
    common::cluster_common::delete_cluster_or_panic(&client, &cluster.id);
}
