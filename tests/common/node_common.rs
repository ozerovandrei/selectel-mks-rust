use super::cluster_common;
use selectel_mks::Client;

const NODE_REINSTALL_INTERVAL: u64 = 10_000;
const NODE_REINSTALL_RETRIES: usize = 60;

/// Function wraps nodegroup creation with retries.
/// It panics in case of errors.
pub fn reinstall_node_or_panic(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    node_id: &str,
) {
    client
        .reinstall_node(cluster_id, nodegroup_id, node_id)
        .unwrap_or_else(|error| panic!("unable to reinstall a node: {}", error));

    cluster_common::wait_for_cluster_active_status_or_panic(
        client,
        cluster_id,
        NODE_REINSTALL_INTERVAL,
        NODE_REINSTALL_RETRIES,
    );

    println!("Reinstalled node {}", node_id);
}
