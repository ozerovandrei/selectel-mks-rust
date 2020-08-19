use serde::{Deserialize, Serialize};

/// KubeVersion represents a deserialized Kubernetes version body from an API response.
#[derive(Debug, Deserialize, Serialize)]
pub struct KubeVersion {
    /// Version represents the supported Kubernetes version in format: "X.Y.Z".
    pub version: String,

    /// Is default flag indicates if kubernetes version is default.
    pub is_default: bool,
}

/// KubeVersionsRoot represents a list of deserialized Kubernetes versions.
#[derive(Debug, Deserialize, Serialize)]
pub struct KubeVersionsRoot {
    pub kube_versions: Vec<KubeVersion>,
}
