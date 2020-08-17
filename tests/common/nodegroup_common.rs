use super::cluster_common;
use selectel_mks::nodegroup;
use selectel_mks::Client;

const NODEGROUP_CREATE_INTERVAL: u64 = 10_000;
const NODEGROUP_CREATE_RETRIES: usize = 60;

const NODEGROUP_RESIZE_INTERVAL: u64 = 10_000;
const NODEGROUP_RESIZE_RETRIES: usize = 60;

const NODEGROUP_UPDATE_INTERVAL: u64 = 10_000;
const NODEGROUP_UPDATE_RETRIES: usize = 30;

const NODEGROUP_DELETE_INTERVAL: u64 = 10_000;
const NODEGROUP_DELETE_RETRIES: usize = 30;

/// Function wraps nodegroup creation with retries.
/// It panics in case of errors.
pub fn create_nodegroup_or_panic(
    client: &Client,
    cluster_id: &str,
    opts: &nodegroup::schemas::CreateOpts,
) {
    client
        .create_nodegroup(cluster_id, opts)
        .unwrap_or_else(|error| panic!("unable to create a nodegroup: {}", error));

    cluster_common::wait_for_cluster_active_status_or_panic(
        client,
        cluster_id,
        NODEGROUP_CREATE_INTERVAL,
        NODEGROUP_CREATE_RETRIES,
    );

    println!("Created a new nodegroup");
}

/// Function wraps nodegroup resizing with retries.
/// It panics in case of errors.
pub fn resize_nodegroup_or_panic(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    opts: &nodegroup::schemas::ResizeOpts,
) {
    client
        .resize_nodegroup(cluster_id, nodegroup_id, opts)
        .expect("unable to resize a nodegroup");

    cluster_common::wait_for_cluster_active_status_or_panic(
        client,
        cluster_id,
        NODEGROUP_RESIZE_INTERVAL,
        NODEGROUP_RESIZE_RETRIES,
    );

    println!("Resized nodegroup {}", nodegroup_id);
}

/// Function wraps nodegroup updating with retries.
/// It panics in case of errors.
pub fn update_nodegroup_or_panic(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    opts: &nodegroup::schemas::UpdateOpts,
) {
    client
        .update_nodegroup(cluster_id, nodegroup_id, opts)
        .expect("unable to update a nodegroup");

    cluster_common::wait_for_cluster_active_status_or_panic(
        client,
        cluster_id,
        NODEGROUP_UPDATE_INTERVAL,
        NODEGROUP_UPDATE_RETRIES,
    );

    println!("Updated nodegroup {}", nodegroup_id);
}

/// Function wraps nodegroup deletion with retries.
/// It panics in case of errors.
pub fn delete_nodegroup_or_panic(client: &Client, cluster_id: &str, nodegroup_id: &str) {
    client
        .delete_nodegroup(cluster_id, nodegroup_id)
        .expect("unable to delete a nodegroup");

    cluster_common::wait_for_cluster_active_status_or_panic(
        client,
        cluster_id,
        NODEGROUP_DELETE_INTERVAL,
        NODEGROUP_DELETE_RETRIES,
    );

    println!("Deleted nodegroup {}", nodegroup_id);
}
