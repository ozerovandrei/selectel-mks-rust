use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

use super::super::node::schemas::Node;

/// Nodegroup represents a deserialized nodegroup body from an API response.
#[derive(Debug, Deserialize)]
pub struct Nodegroup {
    // Nodegroup identifier.
    pub id: String,

    // Timestamp in UTC timezone of when the nodegroup has been created.
    pub created_at: DateTime<Utc>,

    // Timestamp in UTC timezone of when the nodegroup has been updated.
    pub updated_at: Option<DateTime<Utc>>,

    // Cluster identifier.
    pub cluster_id: String,

    // OpenStack flavor identifier for all nodes in the nodegroup.
    pub flavor_id: String,

    // Initial volume size in GB for each node.
    pub volume_gb: u32,

    // Initial blockstorage volume type for each node.
    pub volume_type: String,

    // Flag that represents if nodes use local volume.
    pub local_volume: bool,

    // OpenStack availability zone for all nodes in the nodegroup.
    pub availability_zone: String,

    // All nodes in the nodegroup.
    pub nodes: Vec<Node>,

    // A map of user-defined Kubernetes labels for each node in the group.
    pub labels: HashMap<String, String>,
}

/// NodegroupRoot represents a root of a deserialized nodegroup.
#[derive(Debug, Deserialize)]
pub struct NodegroupRoot {
    pub nodegroup: Nodegroup,
}

/// NodegroupsRoot represents a root of a list with deserialized nodegroups.
#[derive(Debug, Deserialize)]
pub struct NodegroupsRoot {
    pub nodegroups: Vec<Nodegroup>,
}
