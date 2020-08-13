use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

use super::super::node::schemas::Node;

/// Nodegroup represents a deserialized nodegroup body from an API response.
#[derive(Debug, Deserialize)]
pub struct Nodegroup {
    // Nodegroup identifier.
    id: String,

    // Timestamp in UTC timezone of when the nodegroup has been created.
    created_at: DateTime<Utc>,

    // Timestamp in UTC timezone of when the nodegroup has been updated.
    updated_at: Option<DateTime<Utc>>,

    // Cluster identifier.
    cluster_id: String,

    // OpenStack flavor identifier for all nodes in the nodegroup.
    flavor_id: String,

    // Initial volume size in GB for each node.
    volume_gb: u32,

    // Initial blockstorage volume type for each node.
    volume_type: String,

    // Flag that represents if nodes use local volume.
    local_volume: bool,

    // OpenStack availability zone for all nodes in the nodegroup.
    availability_zone: String,

    // All nodes in the nodegroup.
    nodes: Vec<Node>,

    // A map of user-defined Kubernetes labels for each node in the group.
    labels: HashMap<String, String>,
}

/// NodegroupsRoot represents a list of deserialized nodegroups.
#[derive(Debug, Deserialize)]
pub struct NodegroupsRoot {
    pub nodegroups: Vec<Nodegroup>,
}
