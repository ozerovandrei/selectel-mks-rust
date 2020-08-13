use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Node represents a deserialized node body from an API response.
#[derive(Debug, Deserialize)]
pub struct Node {
    // Node identifier.
    id: String,

    // Timestamp in UTC timezone of when the node has been created.
    created_at: DateTime<Utc>,

    // Timestamp in UTC timezone of when the node has been updated.
    updated_at: Option<DateTime<Utc>>,

    // Node hostname.
    hostname: String,

    // IP address of the node.
    ip: String,

    // Nodegroup identifier.
    nodegroup_id: String,
}
