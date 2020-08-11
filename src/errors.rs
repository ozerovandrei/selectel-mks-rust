/// MKS error return type.
#[derive(Debug, PartialEq)]
pub enum MKSError {
    /// Bad endpoint value.
    EndpointError,

    /// Empty token value.
    EmptyTokenError,
}

impl std::fmt::Display for MKSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MKSError::EndpointError => "failed to parse base endpoint URL".fmt(f),
            MKSError::EmptyTokenError => "token cannot be empty".fmt(f),
        }
    }
}
