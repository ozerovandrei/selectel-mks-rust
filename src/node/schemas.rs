use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Node represents a deserialized node body from an API response.
#[derive(Debug, Deserialize)]
pub struct Node {
    // Node identifier.
    pub id: String,

    // Timestamp in UTC timezone of when the node has been created.
    pub created_at: DateTime<Utc>,

    // Timestamp in UTC timezone of when the node has been updated.
    pub updated_at: Option<DateTime<Utc>>,

    // Node hostname.
    pub hostname: String,

    // IP address of the node.
    pub ip: String,

    // Nodegroup identifier.
    pub nodegroup_id: String,
}

/// NodeRoot represents a root of a deserialized node.
#[derive(Debug, Deserialize)]
pub struct NodeRoot {
    pub node: Node,
}
