use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Status represents a enum with various task statuses.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    InProgress,
    Done,
    Error,
    Unknown,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Status::InProgress => "IN_PROGRESS".fmt(f),
            Status::Done => "DONE".fmt(f),
            Status::Error => "ERROR".fmt(f),
            Status::Unknown => "UNKNOWN".fmt(f),
        }
    }
}

/// Type represents a enum with various task types.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    CreateCluster,
    DeleteCluster,
    RotateCerts,
    NodeGroupResize,
    NodeReinstall,
    ClusterResize,
    UpgradePatchVersion,
    UpgradeMinorVersion,
    UpdateNodegroupLabels,
    UpgradeMastersConfiguration,
    UpgradeClusterConfiguration,
    Unknown,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Type::CreateCluster => "CREATE_CLUSTER".fmt(f),
            Type::DeleteCluster => "DELETE_CLUSTER".fmt(f),
            Type::RotateCerts => "ROTATE_CERTS".fmt(f),
            Type::NodeGroupResize => "NODE_GROUP_RESIZE".fmt(f),
            Type::NodeReinstall => "NODE_REINSTALL".fmt(f),
            Type::ClusterResize => "CLUSTER_RESIZE".fmt(f),
            Type::UpgradePatchVersion => "UPGRADE_PATCH_VERSION".fmt(f),
            Type::UpgradeMinorVersion => "UPGRADE_MINOR_VERSION".fmt(f),
            Type::UpdateNodegroupLabels => "UPDATE_NODEGROUP_LABELS".fmt(f),
            Type::UpgradeMastersConfiguration => "UPGRADE_MASTERS_CONFIGURATION".fmt(f),
            Type::UpgradeClusterConfiguration => "UPGRADE_CLUSTER_CONFIGURATION".fmt(f),
            Type::Unknown => "UNKNOWN".fmt(f),
        }
    }
}

/// Task represents a deserialized task body from an API response.
#[derive(Debug, Deserialize)]
pub struct Task {
    /// Task identifier.
    pub id: String,

    /// Timestamp in UTC timezone of when the task has been started.
    pub started_at: DateTime<Utc>,

    /// Timestamp in UTC timezone of when the task has been updated.
    pub updated_at: Option<DateTime<Utc>>,

    /// Cluster identifier.
    pub cluster_id: String,

    /// Current task status.
    pub status: Status,

    /// Task type.
    #[serde(rename = "type")]
    pub task_type: Type,
}

/// TaskRoot represents a root of a deserialized task.
#[derive(Debug, Deserialize)]
pub struct TaskRoot {
    pub task: Task,
}

/// ListRoot represents a root of a list with deserialized tasks.
#[derive(Debug, Deserialize)]
pub struct ListRoot {
    pub tasks: Vec<Task>,
}
