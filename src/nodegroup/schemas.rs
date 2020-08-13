use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

/// NodegroupCreateOpts represents a nodegroup create options for the API create request.
#[derive(Debug, Serialize)]
pub struct NodegroupCreateOpts {
    count: u32,
    flavor_id: Option<String>,
    cpus: Option<u32>,
    ram_mb: Option<u32>,
    volume_gb: Option<u32>,
    volume_type: Option<String>,
    local_volume: bool,
    keypair_name: Option<String>,
    affinity_policy: Option<String>,
    availability_zone: String,
    labels: Option<HashMap<String, String>>,
}

impl NodegroupCreateOpts {
    // Initialize a new NodegroupCreateOpts.
    pub fn new(count: u32, local_volume: bool, availability_zone: String) -> NodegroupCreateOpts {
        NodegroupCreateOpts {
            count,
            flavor_id: None,
            cpus: None,
            ram_mb: None,
            volume_gb: None,
            volume_type: None,
            local_volume,
            keypair_name: None,
            affinity_policy: None,
            availability_zone,
            labels: None,
        }
    }

    // Add a reference to a pre-created flavor.
    // It can be omitted in most cases.
    pub fn with_flavor_id(mut self, flavor_id: String) -> NodegroupCreateOpts {
        self.flavor_id = Some(flavor_id);
        self
    }

    // Add a CPU count for each node.
    // It can be omitted only in cases when flavor_id is set.
    pub fn with_cpus(mut self, cpus: u32) -> NodegroupCreateOpts {
        self.cpus = Some(cpus);
        self
    }

    // Add a RAM count in MB for each node.
    // It can be omitted only in cases when flavor_id is set.
    pub fn with_ram_mb(mut self, ram_mb: u32) -> NodegroupCreateOpts {
        self.ram_mb = Some(ram_mb);
        self
    }

    // Add a volume size in GB for each node.
    // It can be omitted only in cases when flavor_id is set and volume is local.
    pub fn with_volume_gb(mut self, volume_gb: u32) -> NodegroupCreateOpts {
        self.volume_gb = Some(volume_gb);
        self
    }

    // Add a blockstorage volume type for each node.
    // It can be omitted only in cases when flavor_id is set and volume is local.
    pub fn with_volume_type(mut self, volume_type: String) -> NodegroupCreateOpts {
        self.volume_type = Some(volume_type);
        self
    }

    // Add a name of the SSH key that will be added to all nodes.
    pub fn with_keypair_name(mut self, keypair_name: String) -> NodegroupCreateOpts {
        self.keypair_name = Some(keypair_name);
        self
    }

    // Add an optional parameter to tune nodes affinity.
    pub fn with_affinity_policy(mut self, affinity_policy: String) -> NodegroupCreateOpts {
        self.affinity_policy = Some(affinity_policy);
        self
    }

    // Add a map of user-defined Kubernetes labels for each node in the group.
    pub fn with_labels(mut self, labels: HashMap<String, String>) -> NodegroupCreateOpts {
        self.labels = Some(labels);
        self
    }
}

/// NodegroupCreateOptsRoot represents a root of a nodegroup create options.
#[derive(Debug, Serialize)]
pub struct NodegroupCreateOptsRoot<'a> {
    pub nodegroup: &'a NodegroupCreateOpts,
}

/// NodegroupResizeOpts represents a nodegroup resize options for the API resize request.
#[derive(Debug, Serialize)]
pub struct NodegroupResizeOpts {
    desired: u32,
}

impl NodegroupResizeOpts {
    // Initialize a new NodegroupResizeOpts.
    pub fn new(desired: u32) -> NodegroupResizeOpts {
        NodegroupResizeOpts { desired }
    }
}
