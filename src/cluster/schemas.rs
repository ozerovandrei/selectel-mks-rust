use chrono::{DateTime, Utc};
use serde::Deserialize;

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
    pub maintenance_window_start: String,

    /// UTC time in "hh:mm:ss" format of when the cluster will end its
    /// maintenance tasks.
    pub maintenance_window_end: String,

    /// Timestamp in UTC timezone of the last cluster maintenance start.
    pub maintenance_last_start: DateTime<Utc>,

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

/// KubernetesOptions represents additional Kubernetes-related options
/// such as pod security policy, feature gates, etc.
#[derive(Debug, Deserialize)]
pub struct KubernetesOptions {
    /// Flag that indicates if PodSecurityPolicy admission controller
    /// must be turned on or off.
    pub enable_pod_security_policy: bool,
}
