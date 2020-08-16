use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::super::nodegroup::schemas::NodegroupCreateOpts;

/// Status represents a enum with various cluster statuses.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active,
    PendingCreate,
    PendingUpdate,
    PendingUpgrade,
    PendingRotateCerts,
    PendingDelete,
    PendingResize,
    PendingNodeReinstall,
    PendingUpgradePatchVersion,
    PendingUpgradeMinorVersion,
    PendingUpdateNodegroup,
    PendingUpgradeMastersConfiguration,
    PendingUpgradeClusterConfiguration,
    Maintenance,
    Error,
    Unknown,
}

/// Cluster represents a deserialized cluster body from an API response.
#[derive(Deserialize, Debug)]
pub struct Cluster {
    /// Cluster identifier.
    pub id: String,

    /// Timestamp in UTC timezone of when the cluster has been created.
    pub created_at: DateTime<Utc>,

    /// Timestamp in UTC timezone of when the cluster has been updated.
    pub updated_at: Option<DateTime<Utc>>,

    /// Cluster name.
    pub name: String,

    /// Cluster status.
    pub status: Status,

    /// Project reference.
    pub project_id: String,

    /// Network reference
    pub network_id: String,

    /// Subnet reference.
    pub subnet_id: String,

    /// IP of the Kubernetes API.
    pub kube_api_ip: String,

    /// Current Kubernetes version of the cluster.
    pub kube_version: String,

    /// Region of where the cluster is located.
    pub region: String,

    /// Timestamp in UTC timezone of when the PKI-tree of the cluster has been updated.
    pub pki_tree_updated_at: Option<DateTime<Utc>>,

    /// UTC time in "hh:mm:ss" format of when the cluster will start its
    /// maintenance tasks.
    pub maintenance_window_start: Option<String>,

    /// UTC time in "hh:mm:ss" format of when the cluster will end its
    /// maintenance tasks.
    pub maintenance_window_end: Option<String>,

    /// Timestamp in UTC timezone of the last cluster maintenance start.
    pub maintenance_last_start: Option<DateTime<Utc>>,

    /// Flag that indicates if worker nodes are allowed to be reinstalled automatically
    /// in case of their unavailability or unhealthiness.
    pub enable_autorepair: bool,

    /// Flag that indicates if Kubernetes patch version of the cluster is allowed to be upgraded
    /// automatically.
    pub enable_patch_version_auto_upgrade: bool,

    /// Flag that indicates that cluster has only a single master and that
    /// control-plane is not in highly available mode.
    pub zonal: bool,

    /// Additional Kubernetes-related options
    /// such as pod security policy, feature gates, etc.
    pub kubernetes_options: KubernetesOptions,
}

/// ClusterRoot represents a root of a deserialized cluster.
#[derive(Debug, Deserialize)]
pub struct ClusterRoot {
    pub cluster: Cluster,
}

/// ListRoot represents a root of a list with deserialized clusters.
#[derive(Debug, Deserialize)]
pub struct ListRoot {
    pub clusters: Vec<Cluster>,
}

/// KubernetesOptions represents additional Kubernetes-related options
/// such as pod security policy, feature gates, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct KubernetesOptions {
    /// Flag that indicates if PodSecurityPolicy admission controller
    /// must be turned on or off.
    pub enable_pod_security_policy: bool,
}

/// Create options for a new cluster.
#[derive(Debug, Serialize)]
pub struct CreateOpts {
    name: String,
    network_id: Option<String>,
    subnet_id: Option<String>,
    kube_version: String,
    region: String,
    nodegroups: Option<Vec<NodegroupCreateOpts>>,
    maintenance_window_start: Option<String>,
    enable_autorepair: Option<bool>,
    enable_patch_version_auto_upgrade: Option<bool>,
    zonal: Option<bool>,
    kubernetes_options: Option<KubernetesOptions>,
}

impl CreateOpts {
    pub fn new(name: String, kube_version: String, region: String) -> CreateOpts {
        CreateOpts {
            name,
            network_id: None,
            subnet_id: None,
            kube_version,
            region,
            nodegroups: None,
            maintenance_window_start: None,
            enable_autorepair: None,
            enable_patch_version_auto_upgrade: None,
            zonal: None,
            kubernetes_options: None,
        }
    }

    /// Add a reference to a pre-created network.
    pub fn with_network_id(mut self, network_id: String) -> CreateOpts {
        self.network_id = Some(network_id);
        self
    }

    /// Add a reference to a pre-created subnet.
    pub fn with_subnet_id(mut self, subnet_id: String) -> CreateOpts {
        self.subnet_id = Some(subnet_id);
        self
    }

    /// Add nodegroups parameters.
    pub fn with_nodegroups(mut self, nodegroups: Vec<NodegroupCreateOpts>) -> CreateOpts {
        self.nodegroups = Some(nodegroups);
        self
    }

    /// Add maintenance_window_start in UTC.
    /// It should be in hh:mm:ss format.
    pub fn with_maintenance_window_start(mut self, maintenance_window_start: String) -> CreateOpts {
        self.maintenance_window_start = Some(maintenance_window_start);
        self
    }

    /// Add enable_autorepair flag.
    pub fn with_enable_autorepair(mut self, enable_autorepair: bool) -> CreateOpts {
        self.enable_autorepair = Some(enable_autorepair);
        self
    }

    /// Add enable_patch_version_auto_upgrade flag.
    pub fn with_enable_patch_version_auto_upgrade(
        mut self,
        enable_patch_version_auto_upgrade: bool,
    ) -> CreateOpts {
        self.enable_patch_version_auto_upgrade = Some(enable_patch_version_auto_upgrade);
        self
    }

    /// Add zonal flag.
    pub fn with_zonal(mut self, zonal: bool) -> CreateOpts {
        self.zonal = Some(zonal);
        self
    }

    /// Add kubernetes_options.
    pub fn with_kubernetes_options(mut self, kubernetes_options: KubernetesOptions) -> CreateOpts {
        self.kubernetes_options = Some(kubernetes_options);
        self
    }
}

/// CreateOptsRoot represents a root of cluster create options.
#[derive(Debug, Serialize)]
pub struct CreateOptsRoot<'a> {
    pub cluster: &'a CreateOpts,
}
